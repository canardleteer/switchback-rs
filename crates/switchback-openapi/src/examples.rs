//! Fixture path helpers for integration tests and workspace examples.

use std::path::{Path, PathBuf};

use crate::load::{LoadArgs, load};

pub const MICRO_TAG_GROUPS: &str = "micro/tag-groups/openapi.yaml";
pub const MICRO_NULLABLE_3_0: &str = "micro/nullable-3.0/openapi.yaml";
pub const MICRO_COMPANION: &str = "micro/companion/openapi.yaml";
pub const MICRO_MULTIFILE: &str = "micro/multifile/openapi.yaml";
pub const MICRO_STREAMING: &str = "micro/streaming/openapi.yaml";
pub const MICRO_ACME_ROOT: &str = "micro/acme";

pub const EXAMPLE_ACME_INPUTS: &[&str] = &[
    "v1/openapi.yaml",
    "v2/openapi.yaml",
    "v3alpha1/openapi.yaml",
];

pub const UPSTREAM_LOW_3_0: &str = "upstream/oas3.0-petstore/petstore.yaml";
pub const UPSTREAM_HIGH_3_0: &str = "upstream/oas3.0-link-example/link-example.yaml";
pub const UPSTREAM_LOW_3_1: &str = "upstream/oas3.1-tictactoe/tictactoe.yaml";
pub const UPSTREAM_HIGH_3_1_WEBHOOK: &str = "upstream/oas3.1-webhook/webhook-example.yaml";

/// One OpenAPI example corpus used by tests and `examples/mdbook-openapi`.
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

/// All example corpora (four upstream + four micro).
pub const EXAMPLE_FIXTURES: &[ExampleFixture] = &[
    ExampleFixture {
        id: "petstore-3.0",
        relative: UPSTREAM_LOW_3_0,
        label: "Petstore (OpenAPI 3.0)",
        tier: ExampleTier::Upstream,
    },
    ExampleFixture {
        id: "link-example-3.0",
        relative: UPSTREAM_HIGH_3_0,
        label: "Link example (OpenAPI 3.0)",
        tier: ExampleTier::Upstream,
    },
    ExampleFixture {
        id: "tictactoe-3.1",
        relative: UPSTREAM_LOW_3_1,
        label: "Tic-tac-toe (OpenAPI 3.1)",
        tier: ExampleTier::Upstream,
    },
    ExampleFixture {
        id: "webhook-3.1",
        relative: UPSTREAM_HIGH_3_1_WEBHOOK,
        label: "Webhook example (OpenAPI 3.1)",
        tier: ExampleTier::Upstream,
    },
    ExampleFixture {
        id: "tag-groups",
        relative: MICRO_TAG_GROUPS,
        label: "x-tagGroups (micro)",
        tier: ExampleTier::Micro,
    },
    ExampleFixture {
        id: "nullable-3.0",
        relative: MICRO_NULLABLE_3_0,
        label: "nullable 3.0 (micro)",
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
        id: "streaming",
        relative: MICRO_STREAMING,
        label: "HTTP streaming (micro)",
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

/// Load the Acme three-version OpenAPI corpus.
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
            "missing fixture {} (upstream: run cargo xtask spec-vendor fetch-fixtures --family openapi)",
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
