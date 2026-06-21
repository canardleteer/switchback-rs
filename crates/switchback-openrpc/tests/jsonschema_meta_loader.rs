//! Ensure the shared JSON Schema loader can ingest OpenRPC meta-schema fixtures.

use switchback_jsonschema::{LoadArgs, load};
use switchback_openrpc::meta_schemas as openrpc_meta;

#[test]
fn jsonschema_loader_loads_openrpc_meta_schema_without_hang() {
    let module_root = openrpc_meta::manifest_dir().to_path_buf();
    let relative_path = "spec/1.4/schema.json";
    let manual = load(&LoadArgs {
        module_root: module_root.clone(),
        inputs: vec![module_root.join(relative_path)],
        search_roots: Vec::new(),
        title: None,
    })
    .unwrap_or_else(|e| panic!("load openrpc meta-schema {relative_path}: {e}"));
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
