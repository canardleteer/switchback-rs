//! Markdown link check gate for switchback-mdbook renderer output.

use crate::workspace::WORKSPACE_ROOT;
use anyhow::{bail, Context, Result};
use std::process::Command;

pub fn link_check() -> Result<()> {
    let status = Command::new("cargo")
        .current_dir(WORKSPACE_ROOT)
        .args([
            "test",
            "-p",
            "switchback-mdbook",
            "--test",
            "link_check",
            "--",
            "--nocapture",
        ])
        .status()
        .context("spawn cargo test link_check")?;
    if status.success() {
        Ok(())
    } else {
        bail!("link-check failed");
    }
}
