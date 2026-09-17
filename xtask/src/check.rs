//! Canonically ordered repository checks.

use std::path::Path;

use anyhow::{Result, ensure};

use crate::ci;
use crate::example_fixtures;
use crate::highlight;
use crate::link_check;
use crate::process::{CommandSpec, require, run as run_command};
use crate::render;
use crate::spec_vendor;
use crate::workspace;
use crate::{CheckStep, FeatureArgs, policy};

const RUSTFMT_INSTALL: &str = "Install it with `rustup component add rustfmt`.";
const CLIPPY_INSTALL: &str = "Install it with `rustup component add clippy`.";

pub fn select_steps(only: &[CheckStep], exclude: &[CheckStep]) -> Result<Vec<CheckStep>> {
    let selected: Vec<_> = policy::CHECK_ORDER
        .iter()
        .copied()
        .filter(|step| {
            if only.is_empty() {
                !exclude.contains(step)
            } else {
                only.contains(step)
            }
        })
        .collect();
    ensure!(
        !selected.is_empty(),
        "the check selection is empty; choose at least one registered step"
    );
    Ok(selected)
}

pub fn run(root: &Path, steps: &[CheckStep], features: &FeatureArgs) -> Result<()> {
    require(
        root,
        "Cargo",
        &CommandSpec::new("cargo").arg("--version"),
        "Install a Rust toolchain from https://rustup.rs/.",
    )?;
    if steps.contains(&CheckStep::Fmt) {
        require(
            root,
            "rustfmt",
            &CommandSpec::new("cargo").args(["fmt", "--version"]),
            RUSTFMT_INSTALL,
        )?;
    }
    if steps.contains(&CheckStep::Clippy) {
        require(
            root,
            "Clippy",
            &CommandSpec::new("cargo").args(["clippy", "--version"]),
            CLIPPY_INSTALL,
        )?;
    }

    for step in steps {
        workspace::run(&step_label(*step), || run_step(root, *step, features))?;
    }
    Ok(())
}

fn step_label(step: CheckStep) -> String {
    match step {
        CheckStep::Toolchain => "check-toolchain".into(),
        CheckStep::Fmt => "fmt-check".into(),
        CheckStep::Check => "check".into(),
        CheckStep::Clippy => "clippy".into(),
        CheckStep::Test => "test".into(),
        CheckStep::Render => "render mdbook".into(),
        CheckStep::LinkCheck => "link-check".into(),
        CheckStep::Highlight => "check-highlight-rust".into(),
        CheckStep::SpecVendor => "spec-vendor validate".into(),
        CheckStep::ExampleFixtures => "example-fixtures validate".into(),
    }
}

fn run_step(root: &Path, step: CheckStep, features: &FeatureArgs) -> Result<()> {
    match step {
        CheckStep::Toolchain => ci::check_toolchain(true),
        CheckStep::Fmt => ci::fmt_check(),
        CheckStep::Check => run_command(root, &cargo_quality("check", features, false)),
        CheckStep::Clippy => run_command(root, &cargo_quality("clippy", features, true)),
        CheckStep::Test => run_command(root, &cargo_quality("test", features, false)),
        CheckStep::Render => render::render_mdbook(),
        CheckStep::LinkCheck => link_check::link_check(),
        CheckStep::Highlight => highlight::check_highlight_rust(),
        CheckStep::SpecVendor => spec_vendor::validate(spec_vendor::Family::All),
        CheckStep::ExampleFixtures => {
            example_fixtures::validate_openapi()?;
            example_fixtures::validate_asyncapi()?;
            example_fixtures::validate_openrpc()
        }
    }
}

fn cargo_quality(subcommand: &str, features: &FeatureArgs, deny_warnings: bool) -> CommandSpec {
    let mut command =
        CommandSpec::new("cargo").args([subcommand, "--locked", "--workspace", "--all-targets"]);
    command.args.extend(features.cargo_args());
    if deny_warnings {
        command
            .args
            .extend(["--".into(), "-D".into(), "warnings".into()]);
    }
    command
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_is_deduplicated_and_canonical() {
        let selected = select_steps(&[CheckStep::Test, CheckStep::Fmt, CheckStep::Test], &[])
            .expect("selection should work");
        assert_eq!(selected, vec![CheckStep::Fmt, CheckStep::Test]);
    }

    #[test]
    fn exclusion_preserves_registry_order() {
        let selected = select_steps(&[], &[CheckStep::Clippy]).expect("selection should work");
        assert!(!selected.contains(&CheckStep::Clippy));
        assert_eq!(selected.first(), Some(&CheckStep::Toolchain));
        assert!(selected.contains(&CheckStep::Fmt));
        assert!(selected.contains(&CheckStep::Test));
    }

    #[test]
    fn excluding_every_step_is_an_error() {
        assert!(select_steps(&[], &policy::CHECK_ORDER).is_err());
    }

    #[test]
    fn fmt_does_not_receive_feature_arguments() {
        assert_ne!(CheckStep::Fmt, CheckStep::Check);
    }

    #[test]
    fn cargo_quality_forwards_explicit_features() {
        let features = FeatureArgs {
            all_features: false,
            features: vec!["buf".into()],
            no_default_features: true,
        };
        let command = cargo_quality("check", &features, false);
        assert!(command.args.contains(&"--no-default-features".into()));
        assert!(command.args.contains(&"--features".into()));
        assert!(command.args.contains(&"buf".into()));
        assert!(!command.args.contains(&"--all-features".into()));
    }

    #[test]
    fn cargo_quality_defaults_to_all_features() {
        let command = cargo_quality("test", &FeatureArgs::default(), false);
        assert!(command.args.contains(&"--all-features".into()));
        assert!(command.args.contains(&"--locked".into()));
    }
}
