//! Pre-bootstrap publishability checks (no crates.io upload).

use crate::workspace::{WORKSPACE_ROOT, cargo};
use anyhow::{Context, Result, bail};
use std::process::Command;

/// Publishable workspace crates in dependency order (matches `publish-crate.yml`).
const PUBLISHABLE_CRATES: &[&str] = &[
    "switchback-traits",
    "switchback-codec-pb",
    "switchback-protocols",
    "switchback-jsonschema",
    "switchback-openapi",
    "switchback-asyncapi",
    "switchback-openrpc",
    "switchback-protobuf",
    "switchback-mdbook",
    "switchback-assemble",
];

/// Validate packaging metadata and the leaf-crate publish path without uploading.
pub fn publish_check() -> Result<()> {
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
        "xtask: publish-check ok ({} crate(s); full registry chain checked at publish time)",
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
