mod common;

use std::path::Path;

use switchback_openrpc::meta_schemas;

#[test]
fn meta_schema_integrity() {
    assert!(!meta_schemas::ALL.is_empty());
    for asset in meta_schemas::ALL {
        let path = meta_schemas::resolve_path(asset);
        assert!(path.is_file(), "missing {}", path.display());
        assert!(!meta_schemas::read(asset).expect("read").is_empty());
    }
    common::assert_lock_integrity(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        meta_schemas::ALL.len(),
    );
}

#[test]
fn highlight_assets_present() {
    assert!(!meta_schemas::read(&meta_schemas::SCHEMA_1_4)
        .expect("read")
        .is_empty());
    assert!(!meta_schemas::read(&meta_schemas::SCHEMA_1_3)
        .expect("read")
        .is_empty());
}
