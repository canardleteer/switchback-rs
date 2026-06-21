//! Loader smoke tests for vendored upstream OpenRPC example fixtures.

use std::path::PathBuf;

use switchback_jsonschema::resolve_inputs;

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/upstream")
}

fn load_upstream(relative: &str) {
    let path = fixtures_dir().join(relative);
    let module_root = path.parent().expect("parent").to_path_buf();
    let input = PathBuf::from(path.file_name().expect("file name"));
    resolve_inputs(&module_root, &[input], std::slice::from_ref(&module_root))
        .unwrap_or_else(|e| panic!("resolve {relative}: {e}"));
}

#[test]
fn upstream_metrics_1_3_resolves() {
    load_upstream("metrics-1.3/openrpc.json");
}

#[test]
fn upstream_petstore_expanded_1_4_resolves() {
    load_upstream("petstore-expanded-1.4/openrpc.json");
}

#[test]
fn upstream_link_example_1_4_resolves() {
    load_upstream("link-example-1.4/openrpc.json");
}
