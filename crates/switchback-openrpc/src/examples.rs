//! Fixture path helpers for integration tests and workspace examples.

use std::path::{Path, PathBuf};

use crate::load::{LoadArgs, load};

pub const MICRO_TAG_GROUPS: &str = "micro/tag-groups/openrpc.json";
pub const MICRO_COMPANION: &str = "micro/companion/openrpc.json";
pub const MICRO_MULTIFILE: &str = "micro/multifile/openrpc.json";
pub const MICRO_ACME_ROOT: &str = "micro/acme";

pub const EXAMPLE_ACME_INPUTS: &[&str] = &[
    "v1/openrpc.json",
    "v2/openrpc.json",
    "v3alpha1/openrpc.json",
];

pub const UPSTREAM_METRICS_1_3: &str = "upstream/metrics-1.3/openrpc.json";
pub const UPSTREAM_PETSTORE_1_4: &str = "upstream/petstore-expanded-1.4/openrpc.json";
pub const UPSTREAM_LINK_1_4: &str = "upstream/link-example-1.4/openrpc.json";

/// One OpenRPC example corpus used by tests and workspace examples.
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
    /// Upstream (vendored).
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

/// All example corpora (three upstream + four micro).
pub const EXAMPLE_FIXTURES: &[ExampleFixture] = &[
    ExampleFixture {
        id: "metrics-1.3",
        relative: UPSTREAM_METRICS_1_3,
        label: "Metrics (OpenRPC 1.3)",
        tier: ExampleTier::Upstream,
    },
    ExampleFixture {
        id: "petstore-1.4",
        relative: UPSTREAM_PETSTORE_1_4,
        label: "Petstore expanded (OpenRPC 1.4)",
        tier: ExampleTier::Upstream,
    },
    ExampleFixture {
        id: "link-example-1.4",
        relative: UPSTREAM_LINK_1_4,
        label: "Link example (OpenRPC 1.4)",
        tier: ExampleTier::Upstream,
    },
    ExampleFixture {
        id: "tag-groups",
        relative: MICRO_TAG_GROUPS,
        label: "x-tagGroup (micro)",
        tier: ExampleTier::Micro,
    },
    ExampleFixture {
        id: "companion",
        relative: MICRO_COMPANION,
        label: "Beside companion (micro)",
        tier: ExampleTier::Micro,
    },
    ExampleFixture {
        id: "multifile",
        relative: MICRO_MULTIFILE,
        label: "External $ref (micro)",
        tier: ExampleTier::Micro,
    },
    ExampleFixture {
        id: "acme-api",
        relative: MICRO_ACME_ROOT,
        label: "Acme APIs (micro, 3 versions)",
        tier: ExampleTier::Micro,
    },
];

/// Default example corpus (Acme three-version API).
pub fn default_example_fixtures() -> impl Iterator<Item = &'static ExampleFixture> {
    std::iter::once(example_fixture("acme-api").expect("acme-api fixture registered"))
}

/// Load the Acme three-version OpenRPC corpus.
pub fn load_acme_example() -> switchback_traits::Result<switchback_traits::ReferenceManual> {
    let module_root = fixtures_dir().join(MICRO_ACME_ROOT);
    load(&LoadArgs {
        module_root: module_root.clone(),
        inputs: EXAMPLE_ACME_INPUTS
            .iter()
            .map(|p| PathBuf::from(*p))
            .collect(),
        search_roots: vec![module_root],
        title: Some("Acme APIs".into()),
    })
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

/// Load one fixture by its [`ExampleFixture::relative`] path.
pub fn load_fixture_relative(
    relative: &str,
) -> switchback_traits::Result<switchback_traits::ReferenceManual> {
    let entry = fixture_path(relative);
    if !entry.is_file() {
        return Err(switchback_traits::SwitchbackError::load(format!(
            "missing fixture {} (upstream: run cargo xtask spec-vendor fetch-fixtures --family openrpc)",
            entry.display()
        )));
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
