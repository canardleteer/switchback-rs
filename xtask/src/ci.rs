//! CI helpers: fmt, clippy, test, toolchain, audit, rumdl, ryl.

use crate::workspace::{cargo, external, WORKSPACE_ROOT};
use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Command;

const CARGO_AUDIT_INSTALL: &str = "cargo install cargo-audit --locked";
const RUMDL_INSTALL: &str = "cargo install rumdl --locked";
const RYL_INSTALL: &str = "cargo install ryl --locked";

pub fn fmt() -> Result<()> {
    cargo_fmt(&[])?;
    rumdl_fmt()
}

pub fn fmt_check() -> Result<()> {
    cargo_fmt(&["--check"])
}

fn cargo_fmt(extra: &[&str]) -> Result<()> {
    let mut args = vec!["fmt", "--all", "--"];
    args.extend_from_slice(extra);
    cargo(&args)
}

pub fn check() -> Result<()> {
    cargo(&["check", "--workspace", "--all-targets"])
}

pub fn clippy() -> Result<()> {
    cargo(&[
        "clippy",
        "--workspace",
        "--all-targets",
        "--",
        "-D",
        "warnings",
    ])
}

pub fn test() -> Result<()> {
    cargo(&["test", "--workspace"])
}

pub fn audit() -> Result<()> {
    external("cargo", &["audit"], CARGO_AUDIT_INSTALL)
}

pub fn rumdl_check() -> Result<()> {
    external(
        "rumdl",
        &["check", "--respect-gitignore", "."],
        RUMDL_INSTALL,
    )
}

pub fn rumdl_fmt() -> Result<()> {
    external("rumdl", &["fmt", "--respect-gitignore", "."], RUMDL_INSTALL)
}

pub fn ryl_check() -> Result<()> {
    external("ryl", &["."], RYL_INSTALL)
}

pub fn ryl_fix() -> Result<()> {
    external("ryl", &["--fix", "."], RYL_INSTALL)
}

#[derive(Debug)]
struct ToolchainPin {
    channel: String,
    components: Vec<String>,
}

fn read_toolchain_pin() -> Result<ToolchainPin> {
    let path = Path::new(WORKSPACE_ROOT).join("rust-toolchain.toml");
    let content =
        std::fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    let mut channel = None;
    let mut components = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if let Some(v) = line.strip_prefix("channel = ") {
            channel = Some(trim_toml_string(v));
        } else if let Some(v) = line.strip_prefix("components = ") {
            components = parse_toml_string_array(v);
        }
    }
    let channel = channel.context("rust-toolchain.toml missing channel")?;
    Ok(ToolchainPin {
        channel,
        components,
    })
}

fn trim_toml_string(raw: &str) -> String {
    raw.trim().trim_matches('"').trim_matches('\'').to_string()
}

fn parse_toml_string_array(raw: &str) -> Vec<String> {
    raw.trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| trim_toml_string(s.trim()))
        .filter(|s| !s.is_empty())
        .collect()
}

fn command_output(bin: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(bin)
        .args(args)
        .output()
        .with_context(|| format!("{bin} {}", args.join(" ")))?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    } else {
        bail!(
            "{bin} {} failed ({:?}): {}",
            args.join(" "),
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        )
    }
}

pub fn check_toolchain(strict: bool) -> Result<()> {
    let pin = read_toolchain_pin()?;
    let active_channel = active_toolchain_channel()?;
    let rustc_v = command_output("rustc", &["-V"])?;
    let active_version = rustc_v
        .split_whitespace()
        .nth(1)
        .context("parse rustc -V")?;

    let mut divergent = false;
    if active_channel != pin.channel {
        divergent = true;
        eprintln!(
            "xtask: warning: active rustup channel ({active_channel}) does not match rust-toolchain.toml channel ({})",
            pin.channel
        );
    }

    let installed = command_output("rustup", &["component", "list", "--installed"])?;
    for component in &pin.components {
        if !installed.lines().any(|line| line.starts_with(component)) {
            divergent = true;
            eprintln!(
                "xtask: warning: rustup component {component:?} from rust-toolchain.toml is not installed"
            );
        }
    }

    if divergent {
        if strict {
            bail!(
                "toolchain diverges from rust-toolchain.toml (channel={}, components={:?}); \
                 run `rustup toolchain install` in the repo root",
                pin.channel,
                pin.components
            );
        }
        eprintln!("xtask: check-toolchain: divergent (non-strict mode; use --strict to fail)");
    } else {
        eprintln!(
            "xtask: check-toolchain ok (channel={}, rustc {active_version}, components={:?})",
            pin.channel, pin.components
        );
    }
    Ok(())
}

fn active_toolchain_channel() -> Result<String> {
    let line = command_output("rustup", &["show", "active-toolchain"])?;
    let name = line
        .split_whitespace()
        .next()
        .context("parse rustup show active-toolchain")?;
    name.split('-')
        .next()
        .map(str::to_string)
        .context("parse toolchain channel from active-toolchain name")
}
