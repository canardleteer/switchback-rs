//! Ensure the shared JSON Schema loader can ingest OpenAPI meta-schema fixtures.

use switchback_jsonschema::{LoadArgs, load};
use switchback_openapi::meta_schemas as openapi_meta;

#[test]
fn jsonschema_loader_loads_openapi_meta_schema_without_hang() {
    let module_root = openapi_meta::manifest_dir().to_path_buf();
    let relative_path = "oas/3.1/schema/2025-09-15";
    let manual = load(&LoadArgs {
        module_root: module_root.clone(),
        inputs: vec![module_root.join(relative_path)],
        search_roots: Vec::new(),
        title: None,
    })
    .unwrap_or_else(|e| panic!("load openapi meta-schema {relative_path}: {e}"));
    assert!(!manual.modules.is_empty());
    let entity_count: usize = manual
        .modules
        .iter()
        .flat_map(|m| &m.contracts)
        .flat_map(|c| &c.groups)
        .map(|g| g.entities.len())
        .sum();
    assert!(entity_count > 0, "{relative_path} should yield entities");
}
