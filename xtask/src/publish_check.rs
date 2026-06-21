//! Pre-release publishability checks (no crates.io upload).

use crate::workspace::{WORKSPACE_ROOT, cargo};
use anyhow::{Context, Result, bail};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Publishable workspace crates in dependency order (runtime deps, then dev-deps).
///
/// Matches [`publish-crate.yml`](../../.github/workflows/publish-crate.yml) and
/// `release-plz` publish order requirements.
const PUBLISHABLE_CRATES: &[&str] = &[
    "switchback-traits",
    "switchback-codec-pb",
    "switchback-protocols",
    "switchback-jsonschema",
    "switchback-avro",
    "switchback-asyncapi",
    "switchback-openrpc",
    "switchback-openapi",
    "switchback-protobuf",
    "switchback-assemble",
    "switchback-mdbook",
];

fn crate_manifest(name: &str) -> PathBuf {
    Path::new(WORKSPACE_ROOT)
        .join("crates")
        .join(name)
        .join("Cargo.toml")
}

fn switchback_dev_dependencies(name: &str) -> Result<Vec<String>> {
    let manifest = std::fs::read_to_string(crate_manifest(name))
        .with_context(|| format!("read {}", crate_manifest(name).display()))?;
    let mut in_dev = false;
    let mut deps = Vec::new();
    for line in manifest.lines() {
        if line.trim() == "[dev-dependencies]" {
            in_dev = true;
            continue;
        }
        if line.starts_with('[') {
            in_dev = false;
            continue;
        }
        if !in_dev {
            continue;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let mut crate_name = trimmed.split('=').next().unwrap_or(trimmed).trim();
        if let Some(base) = crate_name.split('.').next() {
            crate_name = base;
        }
        if crate_name.starts_with("switchback-") {
            deps.push(crate_name.to_string());
        }
    }
    Ok(deps)
}

/// `cargo publish` resolves dev-dependencies from crates.io. Any workspace
/// `switchback-*` dev-dep must refer to a crate published **earlier** in
/// [`PUBLISHABLE_CRATES`] (no cycles, no forward references).
fn assert_publishable_dev_dependency_order() -> Result<()> {
    for (idx, &name) in PUBLISHABLE_CRATES.iter().enumerate() {
        let earlier: HashSet<&str> = PUBLISHABLE_CRATES[..idx].iter().copied().collect();
        for dep in switchback_dev_dependencies(name)? {
            if !earlier.contains(dep.as_str()) {
                bail!(
                    "{name} dev-depends on {dep}, but {dep} is not published before {name} \
                     in the workspace publish order"
                );
            }
        }
    }
    Ok(())
}

/// Validate packaging metadata and the leaf-crate publish path without uploading.
pub fn publish_check() -> Result<()> {
    assert_publishable_dev_dependency_order()?;

    for &name in PUBLISHABLE_CRATES {
        eprintln!("xtask: publish-check: cargo package --list -p {name}");
        let status = Command::new("cargo")
            .args(["package", "--list", "-p", name])
            .current_dir(WORKSPACE_ROOT)
            .status()
            .with_context(|| format!("cargo package --list -p {name}"))?;
        if !status.success() {
            bail!("cargo package --list -p {name} failed ({status})");
        }
    }

    eprintln!("xtask: publish-check: cargo publish -p switchback-traits --dry-run --allow-dirty");
    cargo(&[
        "publish",
        "-p",
        "switchback-traits",
        "--dry-run",
        "--allow-dirty",
    ])?;

    eprintln!(
        "xtask: publish-check ok ({} crate(s); dev-dependency publish order valid)",
        PUBLISHABLE_CRATES.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn publishable_crates_match_release_plz() {
        let root = Path::new(WORKSPACE_ROOT);
        let release_plz =
            std::fs::read_to_string(root.join("release-plz.toml")).expect("read release-plz.toml");
        for &name in PUBLISHABLE_CRATES {
            assert!(
                release_plz.contains(&format!("name = \"{name}\"")),
                "missing {name} in release-plz.toml"
            );
        }
    }
}
