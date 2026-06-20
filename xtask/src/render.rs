//! Renderer golden gates for `switchback-mdbook`.

use crate::workspace::WORKSPACE_ROOT;
use anyhow::{bail, Context, Result};
use std::process::Command;

/// Run mdBook renderer golden regression (`switchback-mdbook` output_regression).
pub fn render_mdbook() -> Result<()> {
    let status = Command::new("cargo")
        .current_dir(WORKSPACE_ROOT)
        .args([
            "test",
            "-p",
            "switchback-mdbook",
            "output_regression",
            "--",
            "--nocapture",
        ])
        .status()
        .context("spawn cargo test output_regression")?;
    if status.success() {
        Ok(())
    } else {
        bail!("render --renderer mdbook failed");
    }
}

/// Refresh checked-in golden fixtures under `crates/switchback-mdbook/tests/fixtures/golden/`.
pub fn update_golden() -> Result<()> {
    let status = Command::new("cargo")
        .current_dir(WORKSPACE_ROOT)
        .env("UPDATE_GOLDEN", "1")
        .args([
            "test",
            "-p",
            "switchback-mdbook",
            "output_regression",
            "--",
            "--nocapture",
        ])
        .status()
        .context("spawn cargo test output_regression (UPDATE_GOLDEN=1)")?;
    if status.success() {
        Ok(())
    } else {
        bail!("update-golden failed");
    }
}
