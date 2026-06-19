//! Shared workspace paths and subprocess helpers.

use anyhow::{bail, Context, Result};
use std::io::ErrorKind;
use std::process::Command;

pub const WORKSPACE_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");

/// Log and run an xtask step.
pub fn run(name: &str, f: impl FnOnce() -> Result<()>) -> Result<()> {
    eprintln!("xtask: {name}");
    f()
}

/// Run `cargo` from the workspace root.
pub fn cargo(args: &[&str]) -> Result<()> {
    let status = Command::new("cargo")
        .args(args)
        .current_dir(WORKSPACE_ROOT)
        .status()
        .with_context(|| format!("cargo {}", args.join(" ")))?;
    if status.success() {
        Ok(())
    } else {
        bail!("cargo {} failed ({status})", args.join(" "));
    }
}

/// Run an external binary from the workspace root; fail with an install hint if it is missing.
pub fn external(bin: &str, args: &[&str], install_hint: &str) -> Result<()> {
    let status = Command::new(bin)
        .args(args)
        .current_dir(WORKSPACE_ROOT)
        .status()
        .map_err(|err| match err.kind() {
            ErrorKind::NotFound => {
                anyhow::anyhow!("{bin} not found on PATH; install with: {install_hint}")
            }
            _ => err.into(),
        })
        .with_context(|| format!("{bin} {}", args.join(" ")))?;
    if status.success() {
        Ok(())
    } else {
        bail!("{bin} {} failed ({status})", args.join(" "));
    }
}
