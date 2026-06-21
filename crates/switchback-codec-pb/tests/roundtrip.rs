//! Round-trip integration tests for [`ProtobufCodec`].

use std::path::PathBuf;

use buffa::Message;
use static_assertions::assert_impl_all;
use switchback_codec_pb::{DEFAULT_SWITCHBACK_FILENAME, ProtobufCodec, WIRE_VERSION, convert};
use switchback_traits::{
    Anchor, Companion, Document, EntityBody, EntityRef, ExtensionBody, ExternalUrl, Group, GroupId,
    GroupRef, IntraLink, LinkTarget, ManualContract, Module, ModuleId, OperationBody, ParameterRef,
    ProtocolAttachment, RefKind, Reference, ReferenceManual, ResponseRef, SchemaBody, Source,
    SourceRef, SpecVersion, StoredEntity, SwitchbackCodec, SyncSwitchbackCodec,
};

fn fixture_manual() -> ReferenceManual {
    let schema_ref = Reference {
        target: EntityRef {
            module: "Acme".into(),
            group: "acme.user.v1".into(),
            category: "schema".into(),
            name: "User".into(),
        },
        kind: RefKind::Internal,
    };

    ReferenceManual {
        switchback_version: WIRE_VERSION.into(),
        title: "Acme API".into(),
        sources: vec![Document {
            source_ref: SourceRef {
                uri: "repo://openapi.yaml".into(),
                commit: "abc123".into(),
                content_hash: "deadbeef".into(),
            },
            media_type: "application/yaml".into(),
            content: b"openapi: 3.1.0\n".to_vec(),
        }],
        modules: vec![Module {
            id: ModuleId::from("Acme"),
            title: "Acme".into(),
            overview: "Overview text.".into(),
            contracts: vec![ManualContract {
                family: "openapi".into(),
                version: SpecVersion::from("3.1.0"),
                groups: vec![Group {
                    id: GroupId::from("acme.user.v1"),
                    dir: "acme/user/v1".into(),
                    title: "User API".into(),
                    overview: Some("Group overview.".into()),
                    source: Some(Source {
                        file: "repo://openapi.yaml".into(),
                        span: None,
                    }),
                    entities: vec![
                        StoredEntity {
                            name: "getUser".into(),
                            category: "operation".into(),
                            title: "Get user".into(),
                            doc: Some("Fetch a user by id.".into()),
                            source: None,
                            refs: vec![schema_ref.clone()],
                            intra_links: vec![
                                IntraLink {
                                    anchor: Anchor {
                                        field: "doc".into(),
                                        byte_start: 0,
                                        byte_end: 4,
                                    },
                                    target: LinkTarget::Entity(EntityRef {
                                        module: "Acme".into(),
                                        group: "acme.user.v1".into(),
                                        category: "schema".into(),
                                        name: "User".into(),
                                    }),
                                    raw: "[User]".into(),
                                },
                                IntraLink {
                                    anchor: Anchor {
                                        field: "doc".into(),
                                        byte_start: 10,
                                        byte_end: 20,
                                    },
                                    target: LinkTarget::Group(GroupRef {
                                        module: "Acme".into(),
                                        group: "acme.user.v1".into(),
                                    }),
                                    raw: "user group".into(),
                                },
                                IntraLink {
                                    anchor: Anchor {
                                        field: "doc".into(),
                                        byte_start: 30,
                                        byte_end: 40,
                                    },
                                    target: LinkTarget::External(ExternalUrl {
                                        url: "https://example.com".into(),
                                    }),
                                    raw: "example".into(),
                                },
                            ],
                            body: EntityBody::Operation(OperationBody {
                                signature: "**GET** `/users/{id}`".into(),
                                fence_language: "yaml".into(),
                                fence_body: "operationId: getUser\n".into(),
                                parameters: vec![ParameterRef {
                                    name: "id".into(),
                                    location: "path".into(),
                                    required: true,
                                    schema_ref: schema_ref.clone(),
                                    type_label: "string".into(),
                                    description: String::new(),
                                    protocols: Vec::new(),
                                }],
                                responses: vec![ResponseRef {
                                    status: "200".into(),
                                    severity: switchback_traits::ResponseSeverity::Success,
                                    media_type: "application/json".into(),
                                    schema_ref: schema_ref.clone(),
                                    description: "OK".into(),
                                    protocols: Vec::new(),
                                }],
                                request_body: None,
                                protocols: Vec::new(),
                            }),
                        },
                        StoredEntity {
                            name: "User".into(),
                            category: "schema".into(),
                            title: "User".into(),
                            doc: None,
                            source: None,
                            refs: vec![],
                            intra_links: vec![],
                            body: EntityBody::Schema(SchemaBody {
                                fence_language: "json".into(),
                                fence_body: r#"{"type":"object"}"#.into(),
                                payload_format: "json-schema".into(),
                                properties: vec![],
                            }),
                        },
                        StoredEntity {
                            name: "Webhook".into(),
                            category: "extension".into(),
                            title: "Webhook".into(),
                            doc: None,
                            source: None,
                            refs: vec![],
                            intra_links: vec![],
                            body: EntityBody::Extension(ExtensionBody {
                                extension_type: "asyncapi.webhook".into(),
                                payload: br#"{"hook":true}"#.to_vec(),
                                fence_language: Some("json".into()),
                                fence_body: Some("{}".into()),
                            }),
                        },
                    ],
                    source_path: PathBuf::from("openapi/acme/user/v1"),
                }],
                companions: vec![Companion {
                    output_name: "acme.user.v1.overview.md".into(),
                    bytes: b"# Overview".to_vec(),
                    media_type: "text/markdown".into(),
                    title: "Overview".into(),
                    source_dir: "acme/user/v1".into(),
                    stem: "overview".into(),
                }],
                protocols: Vec::new(),
            }],
        }],
    }
}

#[test]
fn protobuf_codec_is_send_sync() {
    assert_impl_all!(ProtobufCodec: Send, Sync, SwitchbackCodec, SyncSwitchbackCodec);
}

#[test]
fn default_filename_is_switchback_binpb() {
    assert_eq!(DEFAULT_SWITCHBACK_FILENAME, "switchback.binpb");
}

#[test]
fn sync_roundtrip_preserves_manual() {
    let manual = fixture_manual();
    let codec = ProtobufCodec;

    let bytes = SyncSwitchbackCodec::serialize(&codec, &manual).expect("serialize");
    assert!(!bytes.is_empty());

    let round_trip = SyncSwitchbackCodec::deserialize(&codec, &bytes).expect("deserialize");
    assert_eq!(round_trip.switchback_version, WIRE_VERSION);
    assert_eq!(round_trip.title, manual.title);
    assert_eq!(round_trip.sources, manual.sources);
    assert_eq!(round_trip.modules.len(), 1);
    assert_eq!(
        round_trip.modules[0].contracts[0].groups[0].entities.len(),
        3
    );
    assert_eq!(
        round_trip.modules[0].contracts[0].companions,
        manual.modules[0].contracts[0].companions
    );
}

#[tokio::test]
async fn async_roundtrip_preserves_manual() {
    let manual = fixture_manual();
    let codec = ProtobufCodec;

    let bytes = SwitchbackCodec::serialize(&codec, &manual)
        .await
        .expect("serialize");
    let round_trip = SwitchbackCodec::deserialize(&codec, &bytes)
        .await
        .expect("deserialize");
    assert_eq!(round_trip.title, manual.title);
}

#[test]
fn serialize_rejects_unresolved_intra_link() {
    let mut manual = fixture_manual();
    manual.modules[0].contracts[0].groups[0].entities[0]
        .intra_links
        .push(IntraLink {
            anchor: Anchor {
                field: "doc".into(),
                byte_start: 0,
                byte_end: 1,
            },
            target: LinkTarget::Unresolved,
            raw: "?".into(),
        });

    let err = SyncSwitchbackCodec::serialize(&ProtobufCodec, &manual)
        .expect_err("unresolved link should fail");
    assert!(err.to_string().contains("unresolved"));
}

#[test]
fn deserialize_rejects_incompatible_version() {
    let manual = fixture_manual();
    let mut proto = convert::to_proto(&manual).expect("to_proto");
    proto.switchback_version = "v9".into();
    let bad_bytes = proto.encode_to_vec();

    let err = SyncSwitchbackCodec::deserialize(&ProtobufCodec, &bad_bytes)
        .expect_err("incompatible version");
    assert!(err.to_string().contains("switchback_version"));
}

#[test]
fn source_path_is_not_round_tripped() {
    let manual = fixture_manual();
    assert!(
        !manual.modules[0].contracts[0].groups[0]
            .source_path
            .as_os_str()
            .is_empty()
    );

    let round_trip = SyncSwitchbackCodec::deserialize(
        &ProtobufCodec,
        &SyncSwitchbackCodec::serialize(&ProtobufCodec, &manual).unwrap(),
    )
    .unwrap();
    assert!(
        round_trip.modules[0].contracts[0].groups[0]
            .source_path
            .as_os_str()
            .is_empty()
    );
}
