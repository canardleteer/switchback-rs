//! Fixture path helpers for integration tests and workspace examples.

use std::path::{Path, PathBuf};

use crate::load::{LoadArgs, load};

pub const MICRO_ACME_ROOT: &str = "micro/acme";

pub const EXAMPLE_ACME_INPUTS: &[&str] = &[
    "v1/asyncapi.yaml",
    "v2/asyncapi.yaml",
    "v3alpha1/asyncapi.yaml",
];

/// Path to `tests/fixtures/` in this crate.
pub fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

pub fn fixture_path(relative: &str) -> PathBuf {
    fixtures_dir().join(relative)
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
            "missing fixture {}",
            entry.display()
        )));
    }
    load_fixture_at(&entry)
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
