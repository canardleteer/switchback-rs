#![forbid(unsafe_code)]

//! Workspace task runner for `switchback-rs`.
//!
//! Commands: `ci`, `ci-post`, `fmt`, `fmt-check`, `clippy`, `test`, `align-workspace-versions`,
//! `spec-vendor`, `parse` (`--parser <family>`), `render` (`--renderer <target>`), `link-check`,
//! `check-highlight-rust`, `update-highlight-golden`, `update-golden`, and `check-toolchain`.

mod align_versions;
mod ci;
mod example_fixtures;
mod highlight;
mod link_check;
mod render;
mod spec_vendor;
mod workspace;

use align_versions::{align_workspace_versions, AlignVersionsArgs};
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
    /// Rust/parser gate: toolchain, fmt-check, check, clippy, test, render,
    /// link-check, check-highlight-rust, spec-vendor, example-fixtures.
    Ci,
    /// CI matrix integration: test, render, link-check, check-highlight-rust,
    /// spec-vendor, example-fixtures (no check/clippy).
    CiPost,
    /// `cargo fmt --all` plus `rumdl fmt` and `ryl --fix`.
    Fmt,
    /// `cargo fmt --all --check` plus wire-schema `buf lint` / `buf format --diff`.
    FmtCheck,
    Check,
    Clippy,
    Test,
    Audit,
    RumdlCheck,
    RumdlFmt,
    Ryl,
    /// Sync `[workspace.package].version` with `switchback-*` workspace deps.
    AlignWorkspaceVersions(AlignVersionsArgs),
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
    /// Golden HTML parity for protobuf / CEL highlighter.
    CheckHighlightRust,
    /// Refresh highlight HTML golden fixtures.
    UpdateHighlightGolden,
    /// Refresh mdBook renderer golden fixtures.
    UpdateGolden,
    /// Validate vendored meta-schema SHA-256 locks.
    SpecVendor {
        #[command(subcommand)]
        cmd: SpecVendorCmd,
    },
}

#[derive(Subcommand)]
enum SpecVendorCmd {
    /// Recompute SHA-256 of vendored files and compare to lock (no network).
    Validate {
        #[arg(long, default_value = "all")]
        family: String,
    },
    /// Redownload vendored files from upstream URLs in the lock file.
    Fetch {
        #[arg(long, default_value = "all")]
        family: String,
        /// Bootstrap lock file and download all assets (one-time).
        #[arg(long)]
        write_lock: bool,
    },
    /// Validate SHA-256 of vendored OpenAPI example fixtures.
    ValidateFixtures {
        #[arg(long, default_value = "openapi")]
        family: String,
    },
    /// Fetch OpenAPI example API descriptions from upstream.
    FetchFixtures {
        #[arg(long, default_value = "openapi")]
        family: String,
        #[arg(long)]
        write_lock: bool,
    },
}

fn run_ci_post() -> Result<()> {
    run("test", ci::test)?;
    run("render mdbook", render::render_mdbook)?;
    run("link-check", link_check::link_check)?;
    run("check-highlight-rust", highlight::check_highlight_rust)?;
    run("spec-vendor validate", || {
        spec_vendor::validate(spec_vendor::Family::All)
    })?;
    run(
        "example-fixtures validate",
        example_fixtures::validate_openapi,
    )
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Ci => {
            run("check-toolchain", || ci::check_toolchain(true))?;
            run("fmt-check", ci::fmt_check)?;
            run("check", ci::check)?;
            run("clippy", ci::clippy)?;
            run_ci_post()
        }
        Cmd::CiPost => run_ci_post(),
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
        Cmd::AlignWorkspaceVersions(args) => align_workspace_versions(args),
        Cmd::CheckToolchain { strict } => ci::check_toolchain(strict),
        Cmd::Parse { parser } => bail!("parse --parser {parser}: not implemented yet"),
        Cmd::Render { renderer } => {
            if renderer == "mdbook" {
                render::render_mdbook()
            } else {
                bail!("render --renderer {renderer}: unknown renderer (supported: mdbook)")
            }
        }
        Cmd::LinkCheck => link_check::link_check(),
        Cmd::CheckHighlightRust => highlight::check_highlight_rust(),
        Cmd::UpdateHighlightGolden => highlight::update_highlight_golden(),
        Cmd::UpdateGolden => render::update_golden(),
        Cmd::SpecVendor { cmd } => match cmd {
            SpecVendorCmd::Validate { family } => {
                spec_vendor::validate(spec_vendor::Family::from_str(&family)?)
            }
            SpecVendorCmd::Fetch { family, write_lock } => {
                spec_vendor::fetch(spec_vendor::Family::from_str(&family)?, write_lock)
            }
            SpecVendorCmd::ValidateFixtures { family } => match family.as_str() {
                "openapi" => example_fixtures::validate_openapi(),
                other => bail!("validate-fixtures --family {other}: only openapi supported"),
            },
            SpecVendorCmd::FetchFixtures { family, write_lock } => match family.as_str() {
                "openapi" => example_fixtures::fetch_openapi(write_lock),
                other => bail!("fetch-fixtures --family {other}: only openapi supported"),
            },
        },
    }
}
