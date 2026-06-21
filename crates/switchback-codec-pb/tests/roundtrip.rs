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

#[test]
fn protocol_attachments_roundtrip_all_arms() {
    use switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::{
        GrpcContractMeta, GrpcErrorMeta, GrpcMetadataMeta, GrpcOperationMeta, GrpcStatusMeta,
    };
    use switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::{
        HttpContractMeta, HttpErrorMeta, HttpOperationMeta, HttpParameterMeta, HttpResponseMeta,
    };
    use switchback_protocols::{GrpcProtocol, HttpProtocol, ProtocolRegistry};
    use switchback_traits::{ParameterBody, ResponseBody};

    let http = HttpProtocol;
    let grpc = GrpcProtocol;
    let registry = ProtocolRegistry::with_builtins();

    let http_attachments = [
        http.attach_contract(&HttpContractMeta {
            default_server_url: "https://example.com".into(),
            ..Default::default()
        }),
        http.attach_operation(&HttpOperationMeta {
            method: "GET".into(),
            path_template: "/pets".into(),
            ..Default::default()
        }),
        http.attach_response(&HttpResponseMeta {
            status_code: 200,
            ..Default::default()
        }),
        http.attach_error(&HttpErrorMeta {
            status_code: 404,
            ..Default::default()
        }),
        http.attach_parameter(&HttpParameterMeta {
            name: "id".into(),
            location: "path".into(),
            required: true,
            ..Default::default()
        }),
    ];

    let grpc_attachments = [
        grpc.attach_contract(&GrpcContractMeta {
            package_name: "acme.v1".into(),
            ..Default::default()
        }),
        grpc.attach_operation(&GrpcOperationMeta {
            rpc_name: "GetPet".into(),
            ..Default::default()
        }),
        grpc.attach_status(&GrpcStatusMeta {
            code: 0,
            message: "OK".into(),
            ..Default::default()
        }),
        grpc.attach_error(&GrpcErrorMeta {
            code: 5,
            message: "not found".into(),
            ..Default::default()
        }),
        grpc.attach_metadata(&GrpcMetadataMeta {
            key: "x-request-id".into(),
            ..Default::default()
        }),
    ];

    let opaque = ProtocolAttachment {
        protocol_id: "acme/kafka".into(),
        payload: vec![9, 8, 7],
    };

    let schema_ref = Reference {
        target: EntityRef {
            module: "M".into(),
            group: "g".into(),
            category: "schema".into(),
            name: "Pet".into(),
        },
        kind: RefKind::Internal,
    };

    let manual = ReferenceManual {
        switchback_version: WIRE_VERSION.into(),
        title: "Protocol matrix".into(),
        sources: Vec::new(),
        modules: vec![Module {
            id: ModuleId::from("M"),
            title: "M".into(),
            overview: String::new(),
            contracts: vec![ManualContract {
                family: "openapi".into(),
                version: SpecVersion::from("3.1.0"),
                groups: vec![Group {
                    id: GroupId::from("g"),
                    dir: "g".into(),
                    title: "G".into(),
                    overview: None,
                    source: None,
                    entities: vec![
                        StoredEntity {
                            name: "op".into(),
                            category: "operation".into(),
                            title: "op".into(),
                            doc: None,
                            source: None,
                            refs: vec![],
                            intra_links: vec![],
                            body: EntityBody::Operation(OperationBody {
                                signature: "GET /pets".into(),
                                fence_language: "yaml".into(),
                                fence_body: String::new(),
                                parameters: vec![ParameterRef {
                                    name: "id".into(),
                                    location: "path".into(),
                                    required: true,
                                    schema_ref: schema_ref.clone(),
                                    type_label: "string".into(),
                                    description: String::new(),
                                    protocols: vec![http_attachments[4].clone()],
                                }],
                                responses: vec![ResponseRef {
                                    status: "404".into(),
                                    severity: switchback_traits::ResponseSeverity::ClientError,
                                    schema_ref: schema_ref.clone(),
                                    media_type: String::new(),
                                    description: String::new(),
                                    protocols: vec![http_attachments[3].clone()],
                                }],
                                request_body: None,
                                protocols: vec![http_attachments[1].clone()],
                            }),
                        },
                        StoredEntity {
                            name: "param".into(),
                            category: "parameter".into(),
                            title: "param".into(),
                            doc: None,
                            source: None,
                            refs: vec![],
                            intra_links: vec![],
                            body: EntityBody::Parameter(ParameterBody {
                                name: "id".into(),
                                location: "path".into(),
                                required: true,
                                fence_language: "yaml".into(),
                                fence_body: String::new(),
                                protocols: vec![http_attachments[4].clone()],
                            }),
                        },
                        StoredEntity {
                            name: "resp".into(),
                            category: "response".into(),
                            title: "resp".into(),
                            doc: None,
                            source: None,
                            refs: vec![],
                            intra_links: vec![],
                            body: EntityBody::Response(ResponseBody {
                                status: "200".into(),
                                severity: switchback_traits::ResponseSeverity::Success,
                                media_type: String::new(),
                                fence_language: "yaml".into(),
                                fence_body: String::new(),
                                protocols: vec![http_attachments[2].clone()],
                            }),
                        },
                    ],
                    source_path: PathBuf::new(),
                }],
                companions: vec![],
                protocols: vec![
                    http_attachments[0].clone(),
                    grpc_attachments[0].clone(),
                    opaque.clone(),
                ],
            }],
        }],
    };

    let round_trip = SyncSwitchbackCodec::deserialize(
        &ProtobufCodec,
        &SyncSwitchbackCodec::serialize(&ProtobufCodec, &manual).unwrap(),
    )
    .unwrap();

    let contract = &round_trip.modules[0].contracts[0];
    assert_eq!(contract.protocols.len(), 3);
    assert!(registry.decode_attachment(&contract.protocols[0]).is_ok());
    assert!(registry.decode_attachment(&contract.protocols[1]).is_ok());
    match registry.decode_attachment(&contract.protocols[2]).unwrap() {
        switchback_protocols::DecodedAttachment::Opaque {
            protocol_id,
            payload,
        } => {
            assert_eq!(protocol_id, "acme/kafka");
            assert_eq!(payload, vec![9, 8, 7]);
        }
        other => panic!("expected opaque, got {other:?}"),
    }

    if let EntityBody::Operation(op) = &contract.groups[0].entities[0].body {
        assert_eq!(op.protocols.len(), 1);
        assert!(registry.decode_attachment(&op.protocols[0]).is_ok());
        assert_eq!(op.parameters[0].protocols.len(), 1);
        assert_eq!(op.responses[0].protocols.len(), 1);
    } else {
        panic!("expected operation");
    }

    let _ = grpc_attachments;
}
