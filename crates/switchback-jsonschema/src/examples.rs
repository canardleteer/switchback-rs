//! Shared helpers for `tests/fixtures/catalog/` used by integration tests.

use std::path::PathBuf;

/// Canonical example JSON Schema catalog inputs.
pub const EXAMPLE_CATALOG_INPUTS: &[&str] = &[
    "schemas/common.yaml",
    "schemas/foo.yaml",
    "schemas/internal_defs.yaml",
    "schemas/cyclic.yaml",
];

/// Path to the JSON Schema catalog under `tests/fixtures/catalog/`.
pub fn fixtures_catalog_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/catalog")
}
