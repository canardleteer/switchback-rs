mod common;

use common::{codec_roundtrip, count_entities, count_refs, fixtures_dir, load_fixture, normalize};
use switchback_openrpc::{
    examples::{
        MICRO_COMPANION, MICRO_MULTIFILE, MICRO_TAG_GROUPS, UPSTREAM_LINK_1_4,
        UPSTREAM_METRICS_1_3, UPSTREAM_PETSTORE_1_4,
    },
    restore_sources,
};
use switchback_traits::{EntityBody, ResponseSeverity, SyncSwitchbackCodec};
use tempfile::tempdir;

#[test]
fn micro_acme_multi_entry_loads() {
    let manual = switchback_openrpc::load_acme_example().expect("load acme-api");
    let contract = &manual.modules[0].contracts[0];
    assert_eq!(contract.family, "openrpc");
    let group_ids: Vec<_> = contract.groups.iter().map(|g| g.id.as_str()).collect();
    assert!(group_ids.contains(&"acme.example.v1"));
    assert!(group_ids.contains(&"acme.example.v2"));
    assert!(group_ids.contains(&"acme.example.v3alpha1"));
    assert!(count_entities(&manual) > 20);
    assert!(!contract.companions.is_empty());
}

#[test]
fn micro_acme_operation_fields() {
    let manual = switchback_openrpc::load_acme_example().expect("load acme-api");
    let contract = &manual.modules[0].contracts[0];

    let echo_unary = contract
        .groups
        .iter()
        .find(|g| g.id.as_str() == "acme.example.v1")
        .and_then(|g| g.entities.iter().find(|e| e.name == "echoUnary"))
        .expect("echoUnary operation");
    let EntityBody::Operation(body) = &echo_unary.body else {
        panic!("expected operation body");
    };
    assert!(body.signature.contains("**echoUnary**"));
    assert!(body.signature.contains("EchoUnaryResponse"));
    let request = body
        .parameters
        .iter()
        .find(|p| p.name == "request")
        .expect("request parameter ref");
    assert!(request.required);
    let auth = body
        .parameters
        .iter()
        .find(|p| p.name == "Authorization")
        .expect("Authorization parameter ref");
    assert!(!auth.required);
    assert_eq!(body.responses.len(), 1);
    assert_eq!(body.responses[0].severity, ResponseSeverity::Unspecified);
}

#[test]
fn micro_acme_cross_entry_ref_resolves() {
    let manual = switchback_openrpc::load_acme_example().expect("load acme-api");
    let contract = &manual.modules[0].contracts[0];
    let list_catalog = contract
        .groups
        .iter()
        .find(|g| g.id.as_str() == "acme.example.v1")
        .and_then(|g| g.entities.iter().find(|e| e.name == "listCatalogProducts"))
        .expect("listCatalogProducts operation");
    let EntityBody::Operation(body) = &list_catalog.body else {
        panic!("expected operation body");
    };
    let response = &body.responses[0];
    assert_eq!(
        response.schema_ref.target.group, "acme.example.v2",
        "expected cross-entry result ref into v2 group: {:?}",
        response.schema_ref.target
    );
    assert_eq!(response.schema_ref.target.name, "ListProductsResponse");
}

#[test]
fn micro_tag_groups_loads() {
    let manual = load_fixture(MICRO_TAG_GROUPS);
    let contract = &manual.modules[0].contracts[0];
    assert_eq!(contract.family, "openrpc");
    let group_ids: Vec<_> = contract.groups.iter().map(|g| g.id.as_str()).collect();
    assert!(group_ids.contains(&"pets"));
    assert!(group_ids.contains(&"store"));
    assert!(group_ids.contains(&"components"));
}

#[test]
fn micro_multifile_refs() {
    let manual = load_fixture(MICRO_MULTIFILE);
    assert!(count_entities(&manual) > 0);
    assert!(count_refs(&manual) > 0);
}

#[test]
fn micro_companion_beside() {
    let manual = load_fixture(MICRO_COMPANION);
    let contract = &manual.modules[0].contracts[0];
    assert!(
        !contract.companions.is_empty(),
        "expected beside companion markdown"
    );
}

#[test]
fn codec_roundtrip_micro_tag_groups() {
    let manual = normalize(load_fixture(MICRO_TAG_GROUPS));
    let restored = normalize(codec_roundtrip(&manual));
    assert_eq!(manual, restored, "codec round-trip");
}

#[test]
fn source_restore_micro_multifile() {
    let fixture_dir = fixtures_dir().join("micro/multifile");
    let manual = normalize(load_fixture(MICRO_MULTIFILE));
    let temp = tempdir().expect("tempdir");
    restore_sources(&manual, temp.path()).expect("restore");
    for rel in ["openrpc.json", "shared/schemas.json"] {
        let expected = std::fs::read(fixture_dir.join(rel)).expect("read fixture");
        let got = std::fs::read(temp.path().join(rel)).expect("read restored");
        assert_eq!(expected, got, "restored bytes for {rel}");
    }
}

#[test]
fn upstream_metrics_1_3_loads() {
    let path = fixtures_dir().join(UPSTREAM_METRICS_1_3);
    if !path.is_file() {
        eprintln!(
            "skip upstream_metrics_1_3: run cargo xtask spec-vendor fetch-fixtures --family openrpc"
        );
        return;
    }
    let manual = load_fixture(UPSTREAM_METRICS_1_3);
    assert_eq!(manual.modules[0].contracts[0].version.as_str(), "1.3");
    assert!(count_entities(&manual) > 0);
}

#[test]
fn upstream_petstore_expanded_1_4_loads() {
    let path = fixtures_dir().join(UPSTREAM_PETSTORE_1_4);
    if !path.is_file() {
        eprintln!(
            "skip upstream_petstore_expanded_1_4: run cargo xtask spec-vendor fetch-fixtures --family openrpc"
        );
        return;
    }
    let manual = load_fixture(UPSTREAM_PETSTORE_1_4);
    assert_eq!(manual.modules[0].contracts[0].version.as_str(), "1.4");
    assert!(count_entities(&manual) > 0);
    assert!(count_refs(&manual) > 0);
}

#[test]
fn upstream_link_example_1_4_loads() {
    let path = fixtures_dir().join(UPSTREAM_LINK_1_4);
    if !path.is_file() {
        eprintln!(
            "skip upstream_link_example_1_4: run cargo xtask spec-vendor fetch-fixtures --family openrpc"
        );
        return;
    }
    let manual = load_fixture(UPSTREAM_LINK_1_4);
    assert_eq!(manual.modules[0].contracts[0].version.as_str(), "1.4");
    assert!(count_entities(&manual) > 0);
    assert!(count_refs(&manual) > 0);
}

#[test]
fn wire_policy_encode_succeeds() {
    let manual = load_fixture(MICRO_TAG_GROUPS);
    let codec = switchback_codec_pb::ProtobufCodec;
    SyncSwitchbackCodec::serialize(&codec, &manual).expect("serialize");
}
