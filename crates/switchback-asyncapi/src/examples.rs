//! Fixture path helpers (skeleton).

use std::path::PathBuf;

/// Path to `tests/fixtures/` in this crate.
pub fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}
