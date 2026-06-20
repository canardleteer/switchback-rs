//! Sync `[workspace.package].version` with `switchback-*` entries in
//! `[workspace.dependencies]`.

use crate::workspace::WORKSPACE_ROOT;
use anyhow::{bail, Context, Result};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use toml_edit::{DocumentMut, Item, Value};

#[derive(Parser)]
pub struct AlignVersionsArgs {
    /// Target semver; default is the current `[workspace.package].version`.
    #[arg(long)]
    version: Option<String>,
    /// Verify alignment only; exit non-zero on drift.
    #[arg(long)]
    check: bool,
}

pub fn align_workspace_versions(args: AlignVersionsArgs) -> Result<()> {
    let root = Path::new(WORKSPACE_ROOT);
    let manifest_path = root.join("Cargo.toml");
    let content = fs::read_to_string(&manifest_path)
        .with_context(|| format!("read {}", manifest_path.display()))?;

    let mut doc = content
        .parse::<DocumentMut>()
        .context("parse root Cargo.toml")?;

    let workspace_version = read_workspace_version(&doc)?;
    let target = args
        .version
        .as_deref()
        .unwrap_or(workspace_version.as_str())
        .to_string();

    verify_member_inheritance(root)?;

    let drift = collect_drift(&doc, &target)?;
    if args.check {
        if drift.is_empty() {
            eprintln!("xtask: align-workspace-versions: ok ({target})");
            return Ok(());
        }
        for line in &drift {
            eprintln!("xtask: align-workspace-versions: drift: {line}");
        }
        bail!("workspace version alignment drift detected (expected {target})");
    }

    if drift.is_empty() {
        eprintln!("xtask: align-workspace-versions: already aligned at {target}");
        return Ok(());
    }

    write_workspace_version(&mut doc, &target)?;
    align_switchback_dependency_versions(&mut doc, &target)?;

    fs::write(&manifest_path, doc.to_string())
        .with_context(|| format!("write {}", manifest_path.display()))?;

    eprintln!("xtask: align-workspace-versions: set workspace version to {target}");
    for line in drift {
        eprintln!("xtask: align-workspace-versions: updated {line}");
    }
    eprintln!("xtask: align-workspace-versions: run `cargo generate-lockfile` next");
    Ok(())
}

fn read_workspace_version(doc: &DocumentMut) -> Result<String> {
    let workspace = doc
        .get("workspace")
        .and_then(|item| item.as_table())
        .context("Cargo.toml missing [workspace]")?;
    let package = workspace
        .get("package")
        .and_then(|item| item.as_table())
        .context("Cargo.toml missing [workspace.package]")?;
    package
        .get("version")
        .and_then(|item| item.as_value())
        .and_then(Value::as_str)
        .map(str::to_string)
        .context("[workspace.package].version missing or not a string")
}

fn write_workspace_version(doc: &mut DocumentMut, version: &str) -> Result<()> {
    let workspace = doc
        .get_mut("workspace")
        .and_then(|item| item.as_table_mut())
        .context("Cargo.toml missing [workspace]")?;
    let package = workspace
        .get_mut("package")
        .and_then(|item| item.as_table_mut())
        .context("Cargo.toml missing [workspace.package]")?;
    package.insert("version", Item::Value(Value::from(version)));
    Ok(())
}

fn align_switchback_dependency_versions(doc: &mut DocumentMut, version: &str) -> Result<()> {
    let workspace = doc
        .get_mut("workspace")
        .and_then(|item| item.as_table_mut())
        .context("Cargo.toml missing [workspace]")?;
    let deps = workspace
        .get_mut("dependencies")
        .and_then(|item| item.as_table_mut())
        .context("Cargo.toml missing [workspace.dependencies]")?;

    for (name, item) in deps.iter_mut() {
        if !name.starts_with("switchback-") {
            continue;
        }
        if let Some(table) = item.as_inline_table_mut() {
            if !table.contains_key("path") {
                continue;
            }
            table.insert("version", Value::from(version));
            continue;
        }
        if let Some(table) = item.as_table_mut() {
            if !table.contains_key("path") {
                continue;
            }
            table.insert("version", Item::Value(Value::from(version)));
        }
    }
    Ok(())
}

fn collect_drift(doc: &DocumentMut, target: &str) -> Result<Vec<String>> {
    let mut drift = Vec::new();
    let workspace_version = read_workspace_version(doc)?;
    if workspace_version != target {
        drift.push(format!(
            "[workspace.package].version: {workspace_version} -> {target}"
        ));
    }

    let workspace = doc
        .get("workspace")
        .and_then(|item| item.as_table())
        .context("Cargo.toml missing [workspace]")?;
    let deps = workspace
        .get("dependencies")
        .and_then(|item| item.as_table())
        .context("Cargo.toml missing [workspace.dependencies]")?;

    for (name, item) in deps.iter() {
        if !name.starts_with("switchback-") {
            continue;
        }
        let Some(has_path) = dependency_has_path(item) else {
            continue;
        };
        if !has_path {
            continue;
        }
        let current = dependency_version(item).unwrap_or("<missing>");
        if current != target {
            drift.push(format!(
                "[workspace.dependencies].{name}.version: {current} -> {target}"
            ));
        }
    }
    Ok(drift)
}

fn dependency_has_path(item: &Item) -> Option<bool> {
    if let Some(table) = item.as_inline_table() {
        return Some(table.contains_key("path"));
    }
    Some(item.as_table()?.contains_key("path"))
}

fn dependency_version(item: &Item) -> Option<&str> {
    if let Some(table) = item.as_inline_table() {
        return table.get("version").and_then(|v| v.as_str());
    }
    item.as_table()?.get("version")?.as_str()
}

fn verify_member_inheritance(root: &Path) -> Result<()> {
    let crates_dir = root.join("crates");
    for entry in
        fs::read_dir(&crates_dir).with_context(|| format!("read {}", crates_dir.display()))?
    {
        let entry = entry?;
        let manifest = member_manifest_path(&entry.path());
        if !manifest.is_file() {
            continue;
        }
        let content = fs::read_to_string(&manifest)
            .with_context(|| format!("read {}", manifest.display()))?;
        if content.contains("version.workspace = true") {
            continue;
        }
        if content.contains("\nversion = ") || content.contains("\r\nversion = ") {
            bail!(
                "{} must use `version.workspace = true` (inline package.version is not allowed)",
                manifest.display()
            );
        }
        bail!("{} missing `version.workspace = true`", manifest.display());
    }
    Ok(())
}

fn member_manifest_path(crate_dir: &Path) -> PathBuf {
    crate_dir.join("Cargo.toml")
}
