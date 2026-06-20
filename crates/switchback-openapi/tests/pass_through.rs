mod common;

use common::{codec_roundtrip, count_entities, count_refs, fixtures_dir, load_fixture, normalize};
use switchback_openapi::{
    examples::{
        MICRO_COMPANION, MICRO_MULTIFILE, MICRO_NULLABLE_3_0, MICRO_TAG_GROUPS, UPSTREAM_HIGH_3_0,
        UPSTREAM_HIGH_3_1_WEBHOOK, UPSTREAM_LOW_3_0, UPSTREAM_LOW_3_1,
    },
    restore_sources,
};
use switchback_traits::{EntityBody, SyncSwitchbackCodec};
use tempfile::tempdir;

#[test]
fn micro_tag_groups_loads() {
    let manual = load_fixture(MICRO_TAG_GROUPS);
    let contract = &manual.modules[0].contracts[0];
    assert_eq!(contract.family, "openapi");
    let group_ids: Vec<_> = contract.groups.iter().map(|g| g.id.as_str()).collect();
    assert!(group_ids.contains(&"pets"));
    assert!(group_ids.contains(&"store"));
    assert!(group_ids.contains(&"components"));
}

#[test]
fn micro_nullable_3_0_preserves_nullable() {
    let manual = load_fixture(MICRO_NULLABLE_3_0);
    assert_eq!(manual.modules[0].contracts[0].version.as_str(), "3.0.3");
    let schema = manual.modules[0].contracts[0]
        .groups
        .iter()
        .find(|g| g.id.as_str() == "components")
        .and_then(|g| g.entities.iter().find(|e| e.name == "NullablePet"))
        .expect("NullablePet schema");
    if let switchback_traits::EntityBody::Schema(body) = &schema.body {
        assert!(body.fence_body.contains("nullable"));
    } else {
        panic!("expected schema body");
    }
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
    for rel in ["openapi.yaml", "schemas/Pet.yaml"] {
        let expected = std::fs::read(fixture_dir.join(rel)).expect("read fixture");
        let got = std::fs::read(temp.path().join(rel)).expect("read restored");
        assert_eq!(expected, got, "restored bytes for {rel}");
    }
}

#[test]
fn upstream_low_3_0_loads() {
    let path = fixtures_dir().join(UPSTREAM_LOW_3_0);
    if !path.is_file() {
        eprintln!(
            "skip upstream_low_3_0: run cargo xtask spec-vendor fetch-fixtures --family openapi"
        );
        return;
    }
    let manual = load_fixture(UPSTREAM_LOW_3_0);
    assert!(count_entities(&manual) > 0);
}

#[test]
fn upstream_low_3_1_loads() {
    let path = fixtures_dir().join(UPSTREAM_LOW_3_1);
    if !path.is_file() {
        eprintln!(
            "skip upstream_low_3_1: run cargo xtask spec-vendor fetch-fixtures --family openapi"
        );
        return;
    }
    let manual = load_fixture(UPSTREAM_LOW_3_1);
    assert!(count_entities(&manual) > 0);
}

#[test]
fn upstream_high_3_0_loads() {
    let path = fixtures_dir().join(UPSTREAM_HIGH_3_0);
    if !path.is_file() {
        eprintln!("skip upstream_high_3_0");
        return;
    }
    let manual = load_fixture(UPSTREAM_HIGH_3_0);
    assert!(count_entities(&manual) > 0);
    assert!(count_refs(&manual) > 0);
}

#[test]
fn upstream_high_3_1_loads() {
    for rel in [UPSTREAM_HIGH_3_1_WEBHOOK, UPSTREAM_LOW_3_1] {
        let path = fixtures_dir().join(rel);
        if !path.is_file() {
            eprintln!("skip upstream_high_3_1: {rel}");
            return;
        }
        let manual = load_fixture(rel);
        assert!(count_entities(&manual) > 0);
    }
}

#[test]
fn upstream_tictactoe_operation_fields() {
    let path = fixtures_dir().join(UPSTREAM_LOW_3_1);
    if !path.is_file() {
        eprintln!("skip upstream_tictactoe_operation_fields");
        return;
    }
    let manual = load_fixture(UPSTREAM_LOW_3_1);
    let put = manual.modules[0].contracts[0]
        .groups
        .iter()
        .find(|g| g.id.as_str() == "gameplay")
        .and_then(|g| {
            g.entities
                .iter()
                .find(|e| e.name == "PUT /board/{row}/{column}")
        })
        .expect("PUT operation");
    let EntityBody::Operation(body) = &put.body else {
        panic!("expected operation body");
    };
    let progress = body
        .parameters
        .iter()
        .find(|p| p.name == "progressUrl")
        .expect("progressUrl parameter");
    assert_eq!(progress.type_label, "string");
    assert_eq!(progress.location, "header");
    assert!(body.request_body.is_some());
    let row = body
        .parameters
        .iter()
        .find(|p| p.name == "rowParam")
        .expect("rowParam");
    assert_eq!(row.location, "path");
    assert!(row.required);
    assert_eq!(row.type_label, "coordinate");
    let ok = body
        .responses
        .iter()
        .find(|r| r.status == "200")
        .expect("200 response");
    assert_eq!(ok.severity, switchback_traits::ResponseSeverity::Success);
    let bad = body
        .responses
        .iter()
        .find(|r| r.status == "400")
        .expect("400 response");
    assert_eq!(
        bad.severity,
        switchback_traits::ResponseSeverity::ClientError
    );
}

#[test]
fn wire_policy_encode_succeeds() {
    let manual = load_fixture(MICRO_TAG_GROUPS);
    let codec = switchback_codec_pb::ProtobufCodec;
    SyncSwitchbackCodec::serialize(&codec, &manual).expect("serialize");
}
