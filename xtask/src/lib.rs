//! Workspace task runner for `switchback-rs`.

mod align_versions;
mod check;
mod ci;
mod coverage;
mod example_fixtures;
mod highlight;
mod link_check;
mod policy;
mod process;
mod profile;
mod publish_check;
mod render;
mod spec_vendor;
mod workspace;

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use align_versions::{AlignVersionsArgs, align_workspace_versions};
use anyhow::{Result, bail};
use clap::{Args, Parser, Subcommand, ValueEnum};

/// The repository development command-line interface.
#[derive(Debug, Parser)]
#[command(
    name = "cargo xtask",
    bin_name = "cargo xtask",
    about = "Run repository development tasks"
)]
pub struct Cli {
    #[command(subcommand)]
    task: Task,
}

/// A repository development task.
#[derive(Debug, Subcommand)]
enum Task {
    /// Run all or a selected subset of repository checks.
    #[command(visible_alias = "ci")]
    Check(CheckArgs),
    /// Generate a fresh coverage report and optionally open it.
    Coverage(CoverageArgs),
    /// Generate a fresh coverage report and open it.
    CoverageOpen(CoverageSelection),
    /// Record a fresh profile and optionally open it.
    Profile(ProfileArgs),
    /// Record a fresh profile and open it.
    ProfileOpen,
    /// `cargo fmt --all` plus `rumdl fmt` and `ryl --fix`.
    Fmt,
    /// `cargo fmt --all --check` plus wire-schema `buf lint` / `buf format --diff`.
    FmtCheck,
    Clippy,
    Test,
    Audit,
    RumdlCheck,
    RumdlFmt,
    Ryl,
    /// Sync `[workspace.package].version` with `switchback-*` workspace deps.
    AlignWorkspaceVersions(AlignVersionsArgs),
    /// Validate crates.io packaging without uploading (package --list + traits dry-run).
    PublishCheck,
    CheckToolchain {
        #[arg(long)]
        strict: bool,
    },
    /// Not implemented yet.
    Parse {
        #[arg(long)]
        parser: String,
    },
    /// Render a documentation target (`mdbook`).
    Render {
        #[arg(long)]
        renderer: String,
    },
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

/// Selectable check steps. Absence of a selector means all steps.
#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum CheckStep {
    Toolchain,
    Fmt,
    Check,
    Clippy,
    Test,
    Render,
    LinkCheck,
    Highlight,
    SpecVendor,
    ExampleFixtures,
}

#[derive(Debug, Args)]
struct CheckArgs {
    /// Run only these comma-separated checks.
    #[arg(long, value_delimiter = ',', num_args = 1.., conflicts_with = "exclude")]
    only: Vec<CheckStep>,
    /// Run all checks except these comma-separated checks.
    #[arg(long, value_delimiter = ',', num_args = 1.., conflicts_with = "only")]
    exclude: Vec<CheckStep>,
    #[command(flatten)]
    features: FeatureArgs,
}

/// Cargo feature-selection arguments shared by checks and coverage.
#[derive(Clone, Debug, Default, Args)]
pub struct FeatureArgs {
    /// Build with all features (also the default when no feature option is set).
    #[arg(
        long,
        conflicts_with = "features",
        conflicts_with = "no_default_features"
    )]
    pub(crate) all_features: bool,
    /// Comma-separated Cargo features to enable.
    #[arg(
        long,
        value_delimiter = ',',
        num_args = 1..,
        value_parser = parse_nonempty,
        conflicts_with = "all_features"
    )]
    pub(crate) features: Vec<String>,
    /// Disable default features; may be combined with --features.
    #[arg(long, conflicts_with = "all_features")]
    pub(crate) no_default_features: bool,
}

impl FeatureArgs {
    pub(crate) fn cargo_args(&self) -> Vec<OsString> {
        if !self.all_features && self.features.is_empty() && !self.no_default_features {
            return vec!["--all-features".into()];
        }
        let mut args = Vec::new();
        if self.all_features {
            args.push("--all-features".into());
        }
        if self.no_default_features {
            args.push("--no-default-features".into());
        }
        if !self.features.is_empty() {
            args.push("--features".into());
            args.push(self.features.join(",").into());
        }
        args
    }
}

fn parse_nonempty(value: &str) -> std::result::Result<String, String> {
    let value = value.trim();
    if value.is_empty() {
        Err("feature names cannot be empty".to_string())
    } else {
        Ok(value.to_string())
    }
}

/// Supported coverage engines.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
pub enum CoverageEngine {
    #[default]
    LlvmCov,
    Tarpaulin,
}

#[derive(Clone, Debug, Args)]
struct CoverageSelection {
    /// Coverage engine.
    #[arg(long, value_enum, default_value_t)]
    engine: CoverageEngine,
    #[command(flatten)]
    features: FeatureArgs,
}

#[derive(Debug, Args)]
struct CoverageArgs {
    #[command(flatten)]
    selection: CoverageSelection,
    /// Open the newly generated report.
    #[arg(long)]
    open: bool,
}

#[derive(Debug, Args)]
struct ProfileArgs {
    /// Open the newly recorded profile.
    #[arg(long)]
    open: bool,
}

#[derive(Subcommand, Debug)]
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

/// Dispatch a parsed repository task.
pub fn run(cli: Cli) -> Result<()> {
    let root = workspace_root();
    match cli.task {
        Task::Check(args) => {
            let steps = check::select_steps(&args.only, &args.exclude)?;
            check::run(&root, &steps, &args.features)
        }
        Task::Coverage(args) => coverage::run(
            &root,
            args.selection.engine,
            &args.selection.features,
            args.open,
        ),
        Task::CoverageOpen(args) => coverage::run(&root, args.engine, &args.features, true),
        Task::Profile(args) => profile::run(&root, args.open),
        Task::ProfileOpen => profile::run(&root, true),
        Task::Fmt => {
            ci::fmt()?;
            ci::ryl_fix()
        }
        Task::FmtCheck => check::run(&root, &[CheckStep::Fmt], &FeatureArgs::default()),
        Task::Clippy => check::run(&root, &[CheckStep::Clippy], &FeatureArgs::default()),
        Task::Test => check::run(&root, &[CheckStep::Test], &FeatureArgs::default()),
        Task::Audit => ci::audit(),
        Task::RumdlCheck => ci::rumdl_check(),
        Task::RumdlFmt => ci::rumdl_fmt(),
        Task::Ryl => ci::ryl_check(),
        Task::AlignWorkspaceVersions(args) => align_workspace_versions(args),
        Task::PublishCheck => publish_check::publish_check(),
        Task::CheckToolchain { strict } => ci::check_toolchain(strict),
        Task::Parse { parser } => bail!("parse --parser {parser}: not implemented yet"),
        Task::Render { renderer } => {
            if renderer == "mdbook" {
                render::render_mdbook()
            } else {
                bail!("render --renderer {renderer}: unknown renderer (supported: mdbook)")
            }
        }
        Task::LinkCheck => link_check::link_check(),
        Task::CheckHighlightRust => highlight::check_highlight_rust(),
        Task::UpdateHighlightGolden => highlight::update_highlight_golden(),
        Task::UpdateGolden => render::update_golden(),
        Task::SpecVendor { cmd } => match cmd {
            SpecVendorCmd::Validate { family } => {
                spec_vendor::validate(spec_vendor::Family::from_str(&family)?)
            }
            SpecVendorCmd::Fetch { family, write_lock } => {
                spec_vendor::fetch(spec_vendor::Family::from_str(&family)?, write_lock)
            }
            SpecVendorCmd::ValidateFixtures { family } => match family.as_str() {
                "openapi" => example_fixtures::validate_openapi(),
                "asyncapi" => example_fixtures::validate_asyncapi(),
                "openrpc" => example_fixtures::validate_openrpc(),
                "all" => {
                    example_fixtures::validate_openapi()?;
                    example_fixtures::validate_asyncapi()?;
                    example_fixtures::validate_openrpc()
                }
                other => bail!(
                    "validate-fixtures --family {other}: use openapi, asyncapi, openrpc, or all"
                ),
            },
            SpecVendorCmd::FetchFixtures { family, write_lock } => match family.as_str() {
                "openapi" => example_fixtures::fetch_openapi(write_lock),
                "asyncapi" => example_fixtures::fetch_asyncapi(write_lock),
                "openrpc" => example_fixtures::fetch_openrpc(write_lock),
                "all" => {
                    example_fixtures::fetch_openapi(write_lock)?;
                    example_fixtures::fetch_asyncapi(write_lock)?;
                    example_fixtures::fetch_openrpc(write_lock)
                }
                other => {
                    bail!("fetch-fixtures --family {other}: use openapi, asyncapi, openrpc, or all")
                }
            },
        },
    }
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must remain directly below the workspace root")
        .to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_builds() {
        Cli::command().debug_assert();
    }

    #[test]
    fn ci_is_the_same_parsed_variant_as_check() {
        let cli = Cli::try_parse_from(["cargo xtask", "ci", "--only", "test,fmt"])
            .expect("ci alias should parse");
        let Task::Check(args) = cli.task else {
            panic!("ci must parse as check");
        };
        assert_eq!(args.only, vec![CheckStep::Test, CheckStep::Fmt]);
    }

    #[test]
    fn selectors_conflict_at_the_parser_boundary() {
        assert!(
            Cli::try_parse_from(["cargo xtask", "check", "--only", "fmt", "--exclude", "test"])
                .is_err()
        );
    }

    #[test]
    fn feature_defaults_and_explicit_combination_match_cargo() {
        assert_eq!(
            FeatureArgs::default().cargo_args(),
            vec![OsString::from("--all-features")]
        );
        let explicit = FeatureArgs {
            all_features: false,
            features: vec!["alpha".into(), "beta".into()],
            no_default_features: true,
        };
        assert_eq!(
            explicit.cargo_args(),
            vec![
                OsString::from("--no-default-features"),
                OsString::from("--features"),
                OsString::from("alpha,beta")
            ]
        );
    }

    #[test]
    fn open_aliases_accept_the_same_coverage_selection() {
        let cli = Cli::try_parse_from([
            "cargo xtask",
            "coverage-open",
            "--engine",
            "tarpaulin",
            "--features",
            "alpha",
        ])
        .expect("coverage-open should parse");
        let Task::CoverageOpen(args) = cli.task else {
            panic!("expected coverage-open");
        };
        assert_eq!(args.engine, CoverageEngine::Tarpaulin);
        assert_eq!(args.features.features, vec!["alpha"]);
    }

    #[test]
    fn empty_only_selector_is_rejected() {
        assert!(Cli::try_parse_from(["cargo xtask", "check", "--only", ""]).is_err());
    }
}
