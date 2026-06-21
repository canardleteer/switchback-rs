//! Fixture path helpers for integration tests and workspace examples.

use std::path::{Path, PathBuf};

use crate::load::{LoadArgs, load};

pub const MICRO_ACME_ROOT: &str = "micro/acme";
pub const MICRO_MINIMAL: &str = "micro/minimal/asyncapi.yaml";

pub const UPSTREAM_STREETLIGHTS: &str = "upstream/streetlights-kafka/asyncapi.yaml";

pub const EXAMPLE_ACME_INPUTS: &[&str] = &[
    "v1/asyncapi.yaml",
    "v2/asyncapi.yaml",
    "v3alpha1/asyncapi.yaml",
];

/// One AsyncAPI example corpus used by tests and `examples/mdbook-asyncapi`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExampleFixture {
    /// Stable directory name for rendered book output.
    pub id: &'static str,
    /// Path relative to [`fixtures_dir`].
    pub relative: &'static str,
    /// Short label for logs and optional title override.
    pub label: &'static str,
    /// `micro` (hand-maintained) or `upstream` (vendored).
    pub tier: ExampleTier,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExampleTier {
    /// Hand-maintained micro fixture.
    Micro,
    /// Vendored upstream fixture (requires `fetch-fixtures`).
    Upstream,
}

impl ExampleTier {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Micro => "micro",
            Self::Upstream => "upstream",
        }
    }
}

/// All example corpora (upstream streetlights + micro Acme + minimal).
pub const EXAMPLE_FIXTURES: &[ExampleFixture] = &[
    ExampleFixture {
        id: "streetlights-kafka",
        relative: UPSTREAM_STREETLIGHTS,
        label: "Streetlights Kafka (AsyncAPI 2.6)",
        tier: ExampleTier::Upstream,
    },
    ExampleFixture {
        id: "acme-api",
        relative: MICRO_ACME_ROOT,
        label: "Acme Events (micro, 3 versions)",
        tier: ExampleTier::Micro,
    },
    ExampleFixture {
        id: "minimal",
        relative: MICRO_MINIMAL,
        label: "Minimal AsyncAPI (micro)",
        tier: ExampleTier::Micro,
    },
];

/// Default example corpus (Acme three-version events API).
pub fn default_example_fixtures() -> impl Iterator<Item = &'static ExampleFixture> {
    std::iter::once(example_fixture("acme-api").expect("acme-api fixture registered"))
}

/// Path to `tests/fixtures/` in this crate.
pub fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

pub fn fixture_path(relative: &str) -> PathBuf {
    fixtures_dir().join(relative)
}

pub fn example_fixture(id: &str) -> Option<&'static ExampleFixture> {
    EXAMPLE_FIXTURES.iter().find(|f| f.id == id)
}

pub fn fixtures_for_tier(tier: ExampleTier) -> impl Iterator<Item = &'static ExampleFixture> {
    EXAMPLE_FIXTURES.iter().filter(move |f| f.tier == tier)
}

/// Load the Acme three-version AsyncAPI corpus.
pub fn load_acme_example() -> switchback_traits::Result<switchback_traits::ReferenceManual> {
    let module_root = fixtures_dir().join(MICRO_ACME_ROOT);
    load(&LoadArgs {
        module_root: module_root.clone(),
        inputs: EXAMPLE_ACME_INPUTS
            .iter()
            .map(|p| PathBuf::from(*p))
            .collect(),
        search_roots: vec![module_root],
        title: Some("Acme Events".into()),
    })
}

/// Load one fixture by relative path to its entry file.
pub fn load_fixture_relative(
    relative: &str,
) -> switchback_traits::Result<switchback_traits::ReferenceManual> {
    let entry = fixture_path(relative);
    if !entry.is_file() {
        return Err(switchback_traits::SwitchbackError::load(format!(
            "missing fixture {} (upstream: run cargo xtask spec-vendor fetch-fixtures --family asyncapi)",
            entry.display()
        )));
    }
    if relative == MICRO_ACME_ROOT || relative.starts_with("micro/acme/") && entry.is_dir() {
        return load_acme_example();
    }
    load_fixture_at(&entry)
}

/// Load one catalogued example fixture.
pub fn load_example(
    fixture: &ExampleFixture,
) -> switchback_traits::Result<switchback_traits::ReferenceManual> {
    if fixture.id == "acme-api" {
        return load_acme_example();
    }
    load_fixture_relative(fixture.relative)
}

fn load_fixture_at(entry: &Path) -> switchback_traits::Result<switchback_traits::ReferenceManual> {
    let module_root = entry
        .parent()
        .expect("fixture parent directory")
        .to_path_buf();
    let input = PathBuf::from(entry.file_name().expect("fixture file name"));
    load(&LoadArgs {
        module_root: module_root.clone(),
        inputs: vec![input],
        search_roots: vec![module_root],
        title: None,
    })
}
