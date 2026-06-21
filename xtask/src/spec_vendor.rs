//! Vendored JSON Schema meta-schemas: validate SHA-256 locks and fetch from upstream.

use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

use crate::workspace::WORKSPACE_ROOT;

const LOCK_FILE: &str = "meta-schemas.lock.toml";
const META_SCHEMAS_DIR: &str = "meta-schemas";

#[derive(Clone, Copy)]
pub enum Family {
    OpenApi,
    AsyncApi,
    OpenRpc,
    Avro,
    All,
}

impl Family {
    pub fn from_str(value: &str) -> Result<Self> {
        match value {
            "openapi" => Ok(Self::OpenApi),
            "asyncapi" => Ok(Self::AsyncApi),
            "openrpc" => Ok(Self::OpenRpc),
            "avro" => Ok(Self::Avro),
            "all" => Ok(Self::All),
            _ => {
                bail!("unknown family {value}; expected openapi|asyncapi|openrpc|avro|all")
            }
        }
    }

    fn families(self) -> Vec<Family> {
        match self {
            Self::All => vec![
                Self::OpenApi,
                Self::AsyncApi,
                Self::OpenRpc,
                Self::Avro,
            ],
            other => vec![other],
        }
    }
}

struct FamilyConfig {
    name: &'static str,
    crate_dir: &'static str,
    raw_base: &'static str,
}

fn family_config(family: Family) -> FamilyConfig {
    match family {
        Family::OpenApi => FamilyConfig {
            name: "openapi",
            crate_dir: "crates/switchback-openapi",
            raw_base: "https://raw.githubusercontent.com/OAI/spec.openapis.org/main",
        },
        Family::AsyncApi => FamilyConfig {
            name: "asyncapi",
            crate_dir: "crates/switchback-asyncapi",
            raw_base: "https://raw.githubusercontent.com/asyncapi/spec-json-schemas/master",
        },
        Family::OpenRpc => FamilyConfig {
            name: "openrpc",
            crate_dir: "crates/switchback-openrpc",
            raw_base: "https://raw.githubusercontent.com/open-rpc/spec/master",
        },
        Family::Avro => FamilyConfig {
            name: "avro",
            crate_dir: "crates/switchback-avro",
            raw_base: "https://raw.githubusercontent.com/asyncapi/spec-json-schemas/master",
        },
        Family::All => unreachable!("Family::All has no config"),
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct LockFile {
    asset: Vec<LockAsset>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct LockAsset {
    id: String,
    path: String,
    url: String,
    sha256: String,
}

pub fn validate(family: Family) -> Result<()> {
    let mut errors = Vec::new();
    for fam in family.families() {
        let cfg = family_config(fam);
        let root = workspace_root().join(cfg.crate_dir);
        let lock = read_lock(&root)?;
        for asset in &lock.asset {
            if let Err(err) = validate_asset(&root, asset) {
                errors.push(format!("{}: {}", cfg.name, err));
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        bail!("spec-vendor validate failed:\n{}", errors.join("\n"));
    }
}

pub fn fetch(family: Family, write_lock: bool) -> Result<()> {
    for fam in family.families() {
        fetch_one(fam, write_lock)?;
    }
    Ok(())
}

fn fetch_one(family: Family, write_lock: bool) -> Result<()> {
    let cfg = family_config(family);
    let root = workspace_root().join(cfg.crate_dir);
    let lock_path = root.join(LOCK_FILE);

    let need_bootstrap = write_lock || !lock_path.exists();
    let lock = if need_bootstrap {
        let lock = bootstrap_lock(family)?;
        write_lock_file(&root, &lock)?;
        generate_rust_assets(&root, &lock)?;
        lock
    } else {
        read_lock(&root)?
    };

    if !need_bootstrap {
        let client = reqwest::blocking::Client::new();
        for asset in &lock.asset {
            download_asset(&client, &root, asset)?;
        }
    }

    Ok(())
}

fn bootstrap_lock(family: Family) -> Result<LockFile> {
    let paths = bootstrap_paths(family)?;
    let cfg = family_config(family);
    let client = reqwest::blocking::Client::new();
    let root = workspace_root().join(cfg.crate_dir);
    let mut assets = Vec::new();
    for path in paths {
        let url = format!("{}/{}", cfg.raw_base, path);
        let asset = download_and_hash(&client, &root, &url, &path, cfg.name)?;
        assets.push(asset);
    }
    Ok(LockFile { asset: assets })
}

fn download_asset(
    client: &reqwest::blocking::Client,
    root: &Path,
    asset: &LockAsset,
) -> Result<()> {
    let dest = root.join(META_SCHEMAS_DIR).join(&asset.path);
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    let bytes = client
        .get(&asset.url)
        .send()
        .with_context(|| format!("GET {}", asset.url))?
        .error_for_status()
        .with_context(|| format!("GET {}", asset.url))?
        .bytes()
        .with_context(|| format!("read body {}", asset.url))?;
    fs::write(&dest, &bytes).with_context(|| format!("write {}", dest.display()))?;
    eprintln!("spec-vendor: fetched {} -> {}", asset.url, dest.display());
    Ok(())
}

fn download_and_hash(
    client: &reqwest::blocking::Client,
    root: &Path,
    url: &str,
    path: &str,
    prefix: &str,
) -> Result<LockAsset> {
    let dest = root.join(META_SCHEMAS_DIR).join(path);
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    let bytes = client
        .get(url)
        .send()
        .with_context(|| format!("GET {}", url))?
        .error_for_status()
        .with_context(|| format!("GET {}", url))?
        .bytes()
        .with_context(|| format!("read body {}", url))?;
    fs::write(&dest, &bytes).with_context(|| format!("write {}", dest.display()))?;
    eprintln!("spec-vendor: fetched {} -> {}", url, dest.display());
    let sha256 = hex_sha256(&bytes);
    Ok(LockAsset {
        id: slug_id(prefix, path),
        path: path.to_string(),
        url: url.to_string(),
        sha256,
    })
}

fn bootstrap_paths(family: Family) -> Result<Vec<String>> {
    match family {
        Family::OpenApi => Ok(openapi_paths()),
        Family::AsyncApi => asyncapi_paths_from_github(),
        Family::OpenRpc => Ok(openrpc_paths()),
        Family::Avro => Ok(avro_paths()),
        Family::All => unreachable!(),
    }
}

fn avro_paths() -> Vec<String> {
    vec![
        "common/avroSchema_v1.json".to_string(),
        "definitions/2.6.0/avroSchema_v1.json".to_string(),
    ]
}

fn openapi_paths() -> Vec<String> {
    let mut paths = Vec::new();
    paths.push("oas/2.0/schema/2017-08-27".to_string());
    paths.extend([
        "oas/3.0/schema/2021-09-28".to_string(),
        "oas/3.0/schema/2024-10-18".to_string(),
    ]);
    for date in [
        "2021-03-02",
        "2021-04-15",
        "2021-05-20",
        "2021-09-28",
        "2022-02-27",
        "2022-10-07",
        "2024-11-14",
        "2025-02-13",
        "2025-08-31",
        "2025-09-15",
        "2025-11-23",
    ] {
        paths.push(format!("oas/3.1/schema/{date}"));
        paths.push(format!("oas/3.1/schema-base/{date}"));
    }
    paths.extend([
        "oas/3.1/dialect/base".to_string(),
        "oas/3.1/dialect/2024-10-25".to_string(),
        "oas/3.1/dialect/2024-11-10".to_string(),
        "oas/3.1/meta/base".to_string(),
        "oas/3.1/meta/2024-10-25".to_string(),
        "oas/3.1/meta/2024-11-10".to_string(),
    ]);
    paths.extend([
        "oas/3.2/schema/2025-09-17".to_string(),
        "oas/3.2/schema/2025-11-23".to_string(),
        "oas/3.2/schema-base/2025-09-17".to_string(),
        "oas/3.2/schema-base/2025-11-23".to_string(),
        "oas/3.2/dialect/2025-09-17".to_string(),
        "oas/3.2/meta/2025-09-17".to_string(),
    ]);
    paths
}

fn openrpc_paths() -> Vec<String> {
    vec![
        "spec/1.3/schema.json".to_string(),
        "spec/1.4/schema.json".to_string(),
    ]
}

fn asyncapi_paths_from_github() -> Result<Vec<String>> {
    const PREFIXES: &[&str] = &[
        "schemas/",
        "definitions/",
        "bindings/",
        "common/",
        "extensions/",
    ];
    let url =
        "https://api.github.com/repos/asyncapi/spec-json-schemas/git/trees/master?recursive=1";
    let client = reqwest::blocking::Client::new();
    let body = client
        .get(url)
        .header("User-Agent", "switchback-rs-xtask")
        .send()
        .context("GET asyncapi tree")?
        .error_for_status()
        .context("GET asyncapi tree")?
        .text()
        .context("read asyncapi tree")?;
    let tree: serde_json::Value = serde_json::from_str(&body).context("parse asyncapi tree")?;
    let entries = tree["tree"]
        .as_array()
        .context("asyncapi tree missing tree array")?;
    let mut paths = Vec::new();
    for entry in entries {
        let path = entry["path"].as_str().unwrap_or_default();
        let typ = entry["type"].as_str().unwrap_or_default();
        if typ != "blob" || !path.ends_with(".json") {
            continue;
        }
        if PREFIXES.iter().any(|prefix| path.starts_with(prefix)) {
            paths.push(path.to_string());
        }
    }
    paths.sort();
    Ok(paths)
}

fn read_lock(crate_root: &Path) -> Result<LockFile> {
    let path = crate_root.join(LOCK_FILE);
    let text = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    toml::from_str(&text).with_context(|| format!("parse {}", path.display()))
}

fn write_lock_file(crate_root: &Path, lock: &LockFile) -> Result<()> {
    let path = crate_root.join(LOCK_FILE);
    let text = toml::to_string_pretty(lock).context("serialize lock")?;
    fs::write(&path, text).with_context(|| format!("write {}", path.display()))?;
    eprintln!("spec-vendor: wrote {}", path.display());
    Ok(())
}

fn validate_asset(crate_root: &Path, asset: &LockAsset) -> Result<()> {
    let path = crate_root.join(META_SCHEMAS_DIR).join(&asset.path);
    let bytes = fs::read(&path).with_context(|| format!("read {}", path.display()))?;
    let actual = hex_sha256(&bytes);
    if actual != asset.sha256 {
        bail!(
            "sha256 mismatch for {}: lock={} actual={} (paste actual into {} if intentional)",
            asset.path,
            asset.sha256,
            actual,
            LOCK_FILE
        );
    }
    Ok(())
}

fn generate_rust_assets(crate_root: &Path, lock: &LockFile) -> Result<()> {
    let mut consts = Vec::new();
    let mut all_refs = Vec::new();
    let mut id_to_name = std::collections::BTreeMap::new();
    for asset in &lock.asset {
        let name = rust_const_name(&asset.id);
        id_to_name.insert(asset.path.clone(), name.clone());
        consts.push(format!(
            r#"pub const {name}: MetaSchemaAsset = MetaSchemaAsset {{ id: "{id}", relative_path: "{path}", source_url: "{url}" }};"#,
            name = name,
            id = escape_rust_str(&asset.id),
            path = escape_rust_str(&asset.path),
            url = escape_rust_str(&asset.url),
        ));
        all_refs.push(name);
    }

    let highlights = highlight_aliases(crate_root, &id_to_name);

    let content = format!(
        r#"// Generated by `cargo xtask spec-vendor fetch --write-lock`. Do not edit by hand.

{consts}

pub const ALL: &[MetaSchemaAsset] = &[{all_refs}];

{highlights}
"#,
        consts = consts.join("\n\n"),
        all_refs = all_refs.join(", "),
        highlights = highlights,
    );

    let path = crate_root.join("src/meta_schemas_assets.rs");
    fs::write(&path, content).with_context(|| format!("write {}", path.display()))?;
    eprintln!("spec-vendor: wrote {}", path.display());
    Ok(())
}

fn highlight_aliases(
    crate_root: &Path,
    id_to_name: &std::collections::BTreeMap<String, String>,
) -> String {
    let crate_name = crate_root
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default();
    let highlights: &[(&str, &str)] = match crate_name {
        "switchback-openapi" => &[
            ("OAS_3_1_SCHEMA_2025_11_23", "oas/3.1/schema/2025-11-23"),
            (
                "OAS_3_1_SCHEMA_BASE_2025_11_23",
                "oas/3.1/schema-base/2025-11-23",
            ),
        ],
        "switchback-asyncapi" => &[
            ("SCHEMAS_3_1_0", "schemas/3.1.0.json"),
            ("SCHEMAS_3_1_0_WITHOUT_ID", "schemas/3.1.0-without-$id.json"),
            ("ALL_SCHEMA_STORE", "schemas/all.schema-store.json"),
        ],
        "switchback-openrpc" => &[
            ("SCHEMA_1_4", "spec/1.4/schema.json"),
            ("SCHEMA_1_3", "spec/1.3/schema.json"),
        ],
        "switchback-avro" => &[
            ("AVRO_SCHEMA_V1_3_0", "common/avroSchema_v1.json"),
            ("AVRO_SCHEMA_V1_2_6", "definitions/2.6.0/avroSchema_v1.json"),
        ],
        _ => &[],
    };

    highlights
        .iter()
        .filter_map(|(alias, path)| {
            id_to_name.get(*path).map(|name| {
                format!(
                    "pub const {alias}: MetaSchemaAsset = {name};",
                    alias = alias,
                    name = name
                )
            })
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn rust_const_name(id: &str) -> String {
    let mut out = String::new();
    for ch in id.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_uppercase());
        } else {
            out.push('_');
        }
    }
    if out.is_empty() || !out.chars().next().unwrap().is_ascii_alphabetic() {
        out = format!("ASSET_{out}");
    }
    out
}

fn slug_id(prefix: &str, path: &str) -> String {
    let slug = path
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>();
    format!("{prefix}-{slug}")
}

fn escape_rust_str(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn hex_sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{:02x}", b)).collect()
}

fn workspace_root() -> PathBuf {
    PathBuf::from(WORKSPACE_ROOT)
}
