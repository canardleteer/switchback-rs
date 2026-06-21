mod common;

use common::{
    META_SCHEMA_FIXTURES, assert_sources_match_inputs, codec_roundtrip, count_entities, count_refs,
    fixtures_catalog_dir, load_catalog, load_meta_schema_fixture, normalize, restore_sources_map,
};
use switchback_jsonschema::examples::EXAMPLE_CATALOG_INPUTS;
use switchback_jsonschema::{LoadArgs, load, resolve_inputs};
use switchback_traits::SyncSwitchbackCodec;
use tempfile::tempdir;

#[test]
fn loader_internal_and_external_refs() {
    let module_root = fixtures_catalog_dir();
    let inputs = vec![module_root.join("schemas/foo.yaml")];
    let resolved = resolve_inputs(&module_root, &inputs, std::slice::from_ref(&module_root))
        .expect("resolve foo.yaml");
    assert!(
        resolved.docs.len() >= 2,
        "foo.yaml should pull in common.yaml"
    );
    assert!(!resolved.index.nodes.is_empty());
}

#[test]
fn loader_cycle_terminates() {
    let module_root = fixtures_catalog_dir();
    let inputs = vec![module_root.join("schemas/cyclic.yaml")];
    let resolved = resolve_inputs(&module_root, &inputs, std::slice::from_ref(&module_root))
        .expect("resolve cyclic.yaml");
    assert!(!resolved.index.ref_targets.is_empty());
}

#[test]
fn examples_catalog_load_codec_roundtrip_and_source_restore() {
    let fixture_dir = fixtures_catalog_dir();
    let manual = normalize(load_catalog());
    assert_sources_match_inputs(&manual, &fixture_dir, EXAMPLE_CATALOG_INPUTS);

    let restored = normalize(codec_roundtrip(&manual));
    assert_eq!(manual, restored, "codec round-trip");

    let temp = tempdir().expect("tempdir");
    restore_sources_map(&restored, temp.path());
    for input in EXAMPLE_CATALOG_INPUTS {
        let expected = std::fs::read(fixture_dir.join(input))
            .unwrap_or_else(|e| panic!("read fixture {input}: {e}"));
        let got = std::fs::read(temp.path().join(input))
            .unwrap_or_else(|e| panic!("read restored {input}: {e}"));
        assert_eq!(expected, got, "restored bytes for {input}");
    }
}

#[test]
fn meta_schema_slice_loads_without_hang() {
    for (family, path) in META_SCHEMA_FIXTURES {
        let manual = load_meta_schema_fixture(family, path);
        assert!(!manual.modules.is_empty());
        assert!(
            count_entities(&manual) > 0,
            "{family}/{path} should yield entities"
        );
    }
}

#[test]
fn structural_smoke_examples_catalog() {
    let manual = load_catalog();
    let contract = &manual.modules[0].contracts[0];
    assert_eq!(contract.family, "jsonschema");
    assert!(!contract.groups.is_empty());
    assert!(count_entities(&manual) > 0);
    assert!(count_refs(&manual) > 0, "expected structural $ref index");
}

#[test]
fn wire_policy_encode_succeeds() {
    let manual = load_catalog();
    let codec = switchback_codec_pb::ProtobufCodec;
    SyncSwitchbackCodec::serialize(&codec, &manual)
        .expect("encode must succeed without LinkTarget::Unresolved");
}

#[test]
fn companion_markdown_discovered() {
    let manual = load_catalog();
    let contract = &manual.modules[0].contracts[0];
    assert!(
        contract
            .companions
            .iter()
            .any(|c| c.output_name.contains("foo")),
        "expected foo companion markdown"
    );
}

#[test]
fn directory_input_expands_to_all_schemas() {
    let module_root = fixtures_catalog_dir();
    let args = LoadArgs {
        module_root: module_root.clone(),
        inputs: vec![module_root.join("schemas")],
        search_roots: vec![module_root.clone()],
        title: None,
    };
    let manual = load(&args).expect("load schemas directory");
    assert!(manual.modules[0].contracts[0].groups.len() >= EXAMPLE_CATALOG_INPUTS.len());
}
