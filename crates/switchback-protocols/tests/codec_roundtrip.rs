//! Protocol attachment round-trip through [`switchback_codec_pb::ProtobufCodec`].
//!
//! Lives in `switchback-protocols` (not `switchback-codec-pb`) so `codec-pb` can
//! publish without a dev-dependency cycle on this crate.

use std::path::PathBuf;

use switchback_codec_pb::{ProtobufCodec, WIRE_VERSION};
use switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::{
    GrpcContractMeta, GrpcErrorMeta, GrpcMetadataMeta, GrpcOperationMeta, GrpcStatusMeta,
};
use switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::{
    HttpContractMeta, HttpErrorMeta, HttpOperationMeta, HttpParameterMeta, HttpResponseMeta,
};
use switchback_protocols::{GrpcProtocol, HttpProtocol, ProtocolRegistry};
use switchback_traits::{
    EntityBody, EntityRef, Group, GroupId, ManualContract, Module, ModuleId, OperationBody,
    ParameterBody, ParameterRef, ProtocolAttachment, RefKind, Reference, ReferenceManual,
    ResponseBody, ResponseRef, SpecVersion, StoredEntity, SyncSwitchbackCodec,
};

#[test]
fn protocol_attachments_roundtrip_all_arms() {
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
