mod common;

use std::path::Path;

use switchback_openapi::meta_schemas;

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
    assert!(meta_schemas::asset_by_path("oas/3.1/schema/2025-11-23").is_some());
    assert!(
        !meta_schemas::read(&meta_schemas::OAS_3_1_SCHEMA_2025_11_23)
            .expect("read")
            .is_empty()
    );
}
