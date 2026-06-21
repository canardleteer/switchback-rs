//! Vendored example API descriptions for integration tests.

use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

use crate::workspace::WORKSPACE_ROOT;

const LOCK_FILE: &str = "example-fixtures.lock.toml";
const FIXTURES_PREFIX: &str = "tests/fixtures/upstream";

const SPEC_REPO: &str = "https://github.com/OAI/OpenAPI-Specification";
const SPEC_COMMIT: &str = "f8449d1a893cc6a811c6f3d87e88b05761dc5397";
const SPEC_RAW: &str = "https://raw.githubusercontent.com/OAI/OpenAPI-Specification/f8449d1a893cc6a811c6f3d87e88b05761dc5397/examples";

const LEARN_REPO: &str = "https://github.com/OAI/learn.openapis.org";
const LEARN_COMMIT: &str = "43756549c27cbf84107b190b82c65e0336f2f09f";
const LEARN_RAW: &str = "https://raw.githubusercontent.com/OAI/learn.openapis.org/43756549c27cbf84107b190b82c65e0336f2f09f/examples/v3.1";

const ASYNCAPI_SPEC_REPO: &str = "https://github.com/asyncapi/spec";
const ASYNCAPI_V26_COMMIT: &str = "v2.6.0";
const ASYNCAPI_V26_RAW: &str = "https://raw.githubusercontent.com/asyncapi/spec/v2.6.0/examples";
const ASYNCAPI_V31_COMMIT: &str = "v3.1.0";
const ASYNCAPI_V31_RAW: &str = "https://raw.githubusercontent.com/asyncapi/spec/v3.1.0/examples";

const OPENRPC_EXAMPLES_REPO: &str = "https://github.com/open-rpc/examples";
const OPENRPC_EXAMPLES_COMMIT: &str = "dce69463ba9a3ca2232506b734606fa97f25dd45";
const OPENRPC_EXAMPLES_RAW: &str = "https://raw.githubusercontent.com/open-rpc/examples/dce69463ba9a3ca2232506b734606fa97f25dd45/service-descriptions";

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
    license: String,
    upstream_repo: String,
    upstream_commit: String,
}

pub fn validate_openapi() -> Result<()> {
    validate_family(openapi_crate_root())
}

pub fn validate_asyncapi() -> Result<()> {
    validate_family(asyncapi_crate_root())
}

pub fn validate_openrpc() -> Result<()> {
    validate_family(openrpc_crate_root())
}

pub fn fetch_openapi(write_lock: bool) -> Result<()> {
    fetch_family(openapi_crate_root(), bootstrap_openapi_lock, write_lock)
}

pub fn fetch_asyncapi(write_lock: bool) -> Result<()> {
    fetch_family(asyncapi_crate_root(), bootstrap_asyncapi_lock, write_lock)
}

pub fn fetch_openrpc(write_lock: bool) -> Result<()> {
    fetch_family(openrpc_crate_root(), bootstrap_openrpc_lock, write_lock)?;
    normalize_openrpc_version_bumps(&openrpc_crate_root())?;
    if write_lock || !openrpc_crate_root().join(LOCK_FILE).exists() {
        let lock = read_lock(&openrpc_crate_root())?;
        let mut updated = lock;
        for asset in &mut updated.asset {
            let path = openrpc_crate_root().join(&asset.path);
            let bytes = fs::read(&path).with_context(|| format!("read {}", path.display()))?;
            asset.sha256 = hex_sha256(&bytes);
        }
        write_lock_file(&openrpc_crate_root(), &updated)?;
    }
    Ok(())
}

fn validate_family(root: PathBuf) -> Result<()> {
    let lock = read_lock(&root)?;
    let mut errors = Vec::new();
    for asset in &lock.asset {
        if let Err(err) = validate_asset(&root, asset) {
            errors.push(format!("{}: {err}", asset.path));
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        bail!("example-fixtures validate failed:\n{}", errors.join("\n"));
    }
}

fn fetch_family(
    root: PathBuf,
    bootstrap: fn() -> Result<LockFile>,
    write_lock: bool,
) -> Result<()> {
    let lock = if write_lock || !root.join(LOCK_FILE).exists() {
        let lock = bootstrap()?;
        write_lock_file(&root, &lock)?;
        lock
    } else {
        read_lock(&root)?
    };

    for asset in &lock.asset {
        fetch_asset(&root, asset)?;
    }
    if write_lock {
        let mut updated = lock;
        for asset in &mut updated.asset {
            let path = root.join(&asset.path);
            let bytes = fs::read(&path).with_context(|| format!("read {}", path.display()))?;
            asset.sha256 = hex_sha256(&bytes);
        }
        write_lock_file(&root, &updated)?;
    }
    Ok(())
}

fn openapi_crate_root() -> PathBuf {
    PathBuf::from(WORKSPACE_ROOT).join("crates/switchback-openapi")
}

fn asyncapi_crate_root() -> PathBuf {
    PathBuf::from(WORKSPACE_ROOT).join("crates/switchback-asyncapi")
}

fn openrpc_crate_root() -> PathBuf {
    PathBuf::from(WORKSPACE_ROOT).join("crates/switchback-openrpc")
}

fn bootstrap_openapi_lock() -> Result<LockFile> {
    Ok(LockFile {
        asset: vec![
            asset_entry(
                "oas3.0-petstore",
                "oas3.0-petstore/petstore.yaml",
                &format!("{SPEC_RAW}/v3.0/petstore.yaml"),
                SPEC_REPO,
                SPEC_COMMIT,
            ),
            asset_entry(
                "oas3.0-link-example",
                "oas3.0-link-example/link-example.yaml",
                &format!("{SPEC_RAW}/v3.0/link-example.yaml"),
                SPEC_REPO,
                SPEC_COMMIT,
            ),
            asset_entry(
                "oas3.1-tictactoe",
                "oas3.1-tictactoe/tictactoe.yaml",
                &format!("{LEARN_RAW}/tictactoe.yaml"),
                LEARN_REPO,
                LEARN_COMMIT,
            ),
            asset_entry(
                "oas3.1-webhook",
                "oas3.1-webhook/webhook-example.yaml",
                &format!("{LEARN_RAW}/webhook-example.yaml"),
                LEARN_REPO,
                LEARN_COMMIT,
            ),
        ],
    })
}

fn bootstrap_asyncapi_lock() -> Result<LockFile> {
    Ok(LockFile {
        asset: vec![
            asset_entry(
                "streetlights-kafka",
                "streetlights-kafka/asyncapi.yaml",
                &format!("{ASYNCAPI_V26_RAW}/streetlights-kafka.yml"),
                ASYNCAPI_SPEC_REPO,
                ASYNCAPI_V26_COMMIT,
            ),
            asset_entry(
                "streetlights-mqtt",
                "streetlights-mqtt/asyncapi.yaml",
                &format!("{ASYNCAPI_V26_RAW}/streetlights-mqtt.yml"),
                ASYNCAPI_SPEC_REPO,
                ASYNCAPI_V26_COMMIT,
            ),
            asset_entry(
                "simple-3.1",
                "simple-3.1/asyncapi.yaml",
                &format!("{ASYNCAPI_V31_RAW}/simple-asyncapi.yml"),
                ASYNCAPI_SPEC_REPO,
                ASYNCAPI_V31_COMMIT,
            ),
            asset_entry(
                "streetlights-kafka-3.1",
                "streetlights-kafka-3.1/asyncapi.yaml",
                &format!("{ASYNCAPI_V31_RAW}/streetlights-kafka-asyncapi.yml"),
                ASYNCAPI_SPEC_REPO,
                ASYNCAPI_V31_COMMIT,
            ),
        ],
    })
}

fn bootstrap_openrpc_lock() -> Result<LockFile> {
    Ok(LockFile {
        asset: vec![
            asset_entry(
                "metrics-1.3",
                "metrics-1.3/openrpc.json",
                &format!("{OPENRPC_EXAMPLES_RAW}/metrics-openrpc.json"),
                OPENRPC_EXAMPLES_REPO,
                OPENRPC_EXAMPLES_COMMIT,
            ),
            asset_entry(
                "petstore-expanded-1.4",
                "petstore-expanded-1.4/openrpc.json",
                &format!("{OPENRPC_EXAMPLES_RAW}/petstore-expanded-openrpc.json"),
                OPENRPC_EXAMPLES_REPO,
                OPENRPC_EXAMPLES_COMMIT,
            ),
            asset_entry(
                "link-example-1.4",
                "link-example-1.4/openrpc.json",
                &format!("{OPENRPC_EXAMPLES_RAW}/link-example-openrpc.json"),
                OPENRPC_EXAMPLES_REPO,
                OPENRPC_EXAMPLES_COMMIT,
            ),
        ],
    })
}

fn asset_entry(
    id: &str,
    rel: &str,
    url: &str,
    upstream_repo: &str,
    upstream_commit: &str,
) -> LockAsset {
    LockAsset {
        id: id.to_string(),
        path: format!("{FIXTURES_PREFIX}/{rel}"),
        url: url.to_string(),
        sha256: String::new(),
        license: "Apache-2.0".to_string(),
        upstream_repo: upstream_repo.to_string(),
        upstream_commit: upstream_commit.to_string(),
    }
}

fn read_lock(root: &Path) -> Result<LockFile> {
    let text = fs::read_to_string(root.join(LOCK_FILE))
        .with_context(|| format!("read {}", root.join(LOCK_FILE).display()))?;
    toml::from_str(&text).context("parse example-fixtures lock")
}

fn write_lock_file(root: &Path, lock: &LockFile) -> Result<()> {
    let text = toml::to_string_pretty(lock).context("serialize example-fixtures lock")?;
    fs::write(root.join(LOCK_FILE), text).context("write example-fixtures lock")
}

fn validate_asset(root: &Path, asset: &LockAsset) -> Result<()> {
    let path = root.join(&asset.path);
    let bytes = fs::read(&path).with_context(|| format!("read {}", path.display()))?;
    let actual = hex_sha256(&bytes);
    if asset.sha256.is_empty() {
        bail!(
            "{}: missing sha256 in lock (run fetch-fixtures --write-lock)",
            asset.path
        );
    }
    if actual != asset.sha256 {
        bail!(
            "{}: sha256 mismatch (expected {}, got {})",
            asset.path,
            asset.sha256,
            actual
        );
    }
    Ok(())
}

fn fetch_asset(root: &Path, asset: &LockAsset) -> Result<()> {
    let dest = root.join(&asset.path);
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).with_context(|| format!("mkdir {}", parent.display()))?;
    }
    let response =
        reqwest::blocking::get(&asset.url).with_context(|| format!("fetch {}", asset.url))?;
    if !response.status().is_success() {
        bail!("fetch {}: HTTP {}", asset.url, response.status());
    }
    let bytes = response
        .bytes()
        .with_context(|| format!("read body from {}", asset.url))?;
    if bytes.starts_with(b"404:") || bytes.starts_with(b"404 ") {
        bail!("fetch {}: response looks like a 404 page", asset.url);
    }
    fs::write(&dest, &bytes).with_context(|| format!("write {}", dest.display()))?;
    eprintln!(
        "example-fixtures: wrote {} (sha256 {})",
        dest.display(),
        hex_sha256(&bytes)
    );
    Ok(())
}

fn hex_sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

/// Upstream [open-rpc/examples](https://github.com/open-rpc/examples) still ships
/// `1.0.0-rc1` on several documents; bump to `1.4.0` so parser fixtures match ADR 0019.
fn normalize_openrpc_version_bumps(root: &Path) -> Result<()> {
    for rel in [
        "tests/fixtures/upstream/petstore-expanded-1.4/openrpc.json",
        "tests/fixtures/upstream/link-example-1.4/openrpc.json",
    ] {
        let path = root.join(rel);
        let text = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        if text.contains("\"openrpc\": \"1.0.0-rc1\"") {
            let updated = text.replace("\"openrpc\": \"1.0.0-rc1\"", "\"openrpc\": \"1.4.0\"");
            fs::write(&path, updated).with_context(|| format!("write {}", path.display()))?;
        }
    }
    Ok(())
}
