//! Shared helpers for `tests/fixtures/proto/` used by integration tests.

use std::path::PathBuf;

/// Canonical example `.proto` inputs (excludes vendored `buf/validate/validate.proto`).
pub const EXAMPLE_PROTO_INPUTS: &[&str] = &[
    "acme/example/v1/echo.proto",
    "acme/example/v1/gateway.proto",
    "acme/example/v2/types.proto",
    "acme/example/v2/catalog.proto",
    "acme/example/v2/services.proto",
    "acme/example/v3alpha1/types.proto",
    "acme/example/v3alpha1/pipeline.proto",
    "acme/example/v3alpha1/services.proto",
];

/// Path to the Buf module under `tests/fixtures/proto/`.
pub fn fixtures_proto_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/proto")
}
