//! Shared helpers for repo-root `examples/jsonschema/`.

use std::path::PathBuf;

/// Canonical example JSON Schema catalog inputs.
pub const EXAMPLE_CATALOG_INPUTS: &[&str] = &[
    "schemas/common.yaml",
    "schemas/foo.yaml",
    "schemas/internal_defs.yaml",
    "schemas/cyclic.yaml",
];

/// Path to the JSON Schema catalog under `examples/jsonschema/`.
pub fn examples_catalog_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/jsonschema")
}
