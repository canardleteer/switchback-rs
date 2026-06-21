mod common;

use common::{codec_roundtrip, count_entities, count_refs, normalize};
use switchback_asyncapi::load_acme_example;
use switchback_protocols::ProtocolRegistry;
use switchback_traits::{EntityBody, SyncSwitchbackCodec};

#[test]
fn micro_acme_multi_entry_loads() {
    let manual = load_acme_example().expect("load acme events");
    let contract = &manual.modules[0].contracts[0];
    assert_eq!(contract.family, "asyncapi");
    let group_ids: Vec<_> = contract.groups.iter().map(|g| g.id.as_str()).collect();
    assert!(group_ids.contains(&"acme.example.v1"));
    assert!(group_ids.contains(&"acme.example.v2"));
    assert!(group_ids.contains(&"acme.example.v3alpha1"));
    assert!(count_entities(&manual) > 10);
    assert!(!contract.companions.is_empty());
}

#[test]
fn micro_acme_v1_kafka_operation_attachment() {
    let manual = load_acme_example().expect("load acme events");
    let contract = &manual.modules[0].contracts[0];
    let registry = ProtocolRegistry::with_builtins();

    let publish = contract
        .groups
        .iter()
        .find(|g| g.id.as_str() == "acme.example.v1")
        .and_then(|g| g.entities.iter().find(|e| e.name == "publishEchoUnary"))
        .expect("publishEchoUnary operation");
    let EntityBody::Operation(body) = &publish.body else {
        panic!("expected operation body");
    };
    assert!(!body.protocols.is_empty());
    assert!(
        body.protocols.iter().any(|p| p.protocol_id == "kafka")
            || body.protocols.iter().any(|p| p.protocol_id == "mqtt")
    );
    let _ = registry;
}

#[test]
fn micro_acme_v3alpha1_avro_schema_payload() {
    let manual = load_acme_example().expect("load acme events");
    let contract = &manual.modules[0].contracts[0];
    let schema = contract
        .groups
        .iter()
        .find(|g| g.id.as_str() == "acme.example.v3alpha1")
        .and_then(|g| g.entities.iter().find(|e| e.name == "PipelineStarted"))
        .expect("PipelineStarted message/schema entity");
    if let EntityBody::Schema(body) = &schema.body {
        assert!(body.payload_format.contains("avro"));
        assert!(body.fence_body.contains("PipelineStarted"));
    } else if let EntityBody::Message(body) = &schema.body {
        assert!(body.fence_body.contains("avro") || body.fence_body.contains("PipelineStarted"));
    } else {
        panic!("expected schema or message body for Avro payload");
    }
}

#[test]
fn micro_acme_v2_tagged_catalog_entities() {
    let manual = load_acme_example().expect("load acme events");
    let contract = &manual.modules[0].contracts[0];
    let v2 = contract
        .groups
        .iter()
        .find(|g| g.id.as_str() == "acme.example.v2")
        .expect("v2 group");
    assert!(
        v2.entities
            .iter()
            .any(|e| e.name == "publishProductCreated")
    );
    assert!(count_refs(&manual) > 0);
}

#[test]
fn codec_roundtrip_acme_multi_entry() {
    let manual = normalize(load_acme_example().expect("load acme events"));
    let restored = normalize(codec_roundtrip(&manual));
    assert_eq!(manual, restored, "codec round-trip");
}

#[test]
fn wire_policy_encode_succeeds() {
    let manual = load_acme_example().expect("load acme events");
    let codec = switchback_codec_pb::ProtobufCodec;
    SyncSwitchbackCodec::serialize(&codec, &manual).expect("serialize");
}
