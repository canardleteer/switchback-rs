#![forbid(unsafe_code)]

//! Workspace task runner for `switchback-rs`.
//!
//! Commands: `ci`, `fmt`, `fmt-check`, `clippy`, `test`, `parse`
//! (`--parser <family>`), `render` (`--renderer <target>`), `link-check`,
//! `update-golden`, and `check-toolchain`. Also supports `--parser`/`--renderer`
//! flags and optional `schema-lint`/`yamlfmt`/`prettier` for example YAML.

mod ci;
mod workspace;

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use workspace::run;

#[derive(Parser)]
#[command(name = "xtask")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Always-on gate: toolchain, fmt-check, check, clippy, test, audit, rumdl, ryl.
    Ci,
    /// `cargo fmt --all` plus `rumdl fmt` and `ryl --fix`.
    Fmt,
    /// `cargo fmt --all --check` plus `rumdl check`.
    FmtCheck,
    Check,
    Clippy,
    Test,
    Audit,
    RumdlCheck,
    RumdlFmt,
    Ryl,
    CheckToolchain {
        #[arg(long)]
        strict: bool,
    },
    /// Not implemented yet.
    Parse {
        #[arg(long)]
        parser: String,
    },
    /// Not implemented yet.
    Render {
        #[arg(long)]
        renderer: String,
    },
    /// Not implemented yet.
    LinkCheck,
    /// Not implemented yet.
    UpdateGolden,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Ci => {
            run("check-toolchain", || ci::check_toolchain(true))?;
            run("fmt-check", ci::fmt_check)?;
            run("check", ci::check)?;
            run("clippy", ci::clippy)?;
            run("test", ci::test)?;
            run("audit", ci::audit)?;
            run("rumdl check", ci::rumdl_check)?;
            run("ryl", ci::ryl_check)?;
            Ok(())
        }
        Cmd::Fmt => {
            ci::fmt()?;
            ci::ryl_fix()
        }
        Cmd::FmtCheck => ci::fmt_check(),
        Cmd::Check => ci::check(),
        Cmd::Clippy => ci::clippy(),
        Cmd::Test => ci::test(),
        Cmd::Audit => ci::audit(),
        Cmd::RumdlCheck => ci::rumdl_check(),
        Cmd::RumdlFmt => ci::rumdl_fmt(),
        Cmd::Ryl => ci::ryl_check(),
        Cmd::CheckToolchain { strict } => ci::check_toolchain(strict),
        Cmd::Parse { parser } => bail!("parse --parser {parser}: not implemented yet"),
        Cmd::Render { renderer } => bail!("render --renderer {renderer}: not implemented yet"),
        Cmd::LinkCheck => bail!("link-check: not implemented yet"),
        Cmd::UpdateGolden => bail!("update-golden: not implemented yet"),
    }
}
