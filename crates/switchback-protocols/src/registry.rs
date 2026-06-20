//! Protocol registry for encode/decode of attachments.

use switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::GrpcPayload;
use switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::__buffa::oneof::grpc_payload::Kind as GrpcKind;
use switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::HttpPayload;
use switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::__buffa::oneof::http_payload::Kind as HttpKind;
use switchback_traits::{ProtocolAttachment, Result, SwitchbackError};

use crate::grpc::GrpcProtocol;
use crate::http::HttpProtocol;
use crate::wire::decode_message;

/// Decoded HTTP payload oneof arm.
#[derive(Clone, Debug, PartialEq)]
pub enum HttpPayloadKind {
    /// Contract-level metadata.
    Contract(
        switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::HttpContractMeta,
    ),
    /// Operation invocation metadata.
    Operation(
        switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::HttpOperationMeta,
    ),
    /// Success response metadata.
    Response(
        switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::HttpResponseMeta,
    ),
    /// Error response metadata.
    Error(switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::HttpErrorMeta),
    /// Parameter metadata.
    Parameter(
        switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::HttpParameterMeta,
    ),
}

/// Decoded gRPC payload oneof arm.
#[derive(Clone, Debug, PartialEq)]
pub enum GrpcPayloadKind {
    /// Contract-level metadata.
    Contract(
        switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::GrpcContractMeta,
    ),
    /// Operation invocation metadata.
    Operation(
        switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::GrpcOperationMeta,
    ),
    /// Success status metadata.
    Status(switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::GrpcStatusMeta),
    /// Error metadata.
    Error(switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::GrpcErrorMeta),
    /// Metadata key.
    Metadata(
        switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::GrpcMetadataMeta,
    ),
}

/// Result of decoding a [`ProtocolAttachment`].
#[derive(Clone, Debug, PartialEq)]
pub enum DecodedAttachment {
    /// Known HTTP payload arm.
    Http(HttpPayloadKind),
    /// Known gRPC payload arm.
    Grpc(GrpcPayloadKind),
    /// Unknown or custom protocol; bytes round-trip opaquely.
    Opaque {
        /// Protocol slug from the attachment envelope.
        protocol_id: String,
        /// Opaque payload bytes.
        payload: Vec<u8>,
    },
}

/// Registry of built-in protocol decoders.
#[derive(Clone, Debug, Default)]
pub struct ProtocolRegistry {
    http: HttpProtocol,
    grpc: GrpcProtocol,
}

impl ProtocolRegistry {
    /// Built-in registry with `http` and `grpc` registered.
    pub fn with_builtins() -> Self {
        Self::default()
    }

    /// HTTP protocol implementation.
    pub fn http(&self) -> &HttpProtocol {
        &self.http
    }

    /// gRPC protocol implementation.
    pub fn grpc(&self) -> &GrpcProtocol {
        &self.grpc
    }

    /// Decode a protocol attachment envelope.
    ///
    /// Built-in ids `"http"` and `"grpc"` deserialize to [`DecodedAttachment::Http`]
    /// or [`DecodedAttachment::Grpc`]; other ids return
    /// [`DecodedAttachment::Opaque`] with bytes unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::HttpOperationMeta;
    /// use switchback_protocols::{DecodedAttachment, HttpPayloadKind, ProtocolRegistry};
    ///
    /// let registry = ProtocolRegistry::with_builtins();
    /// let attachment = registry.http().attach_operation(&HttpOperationMeta {
    ///     method: "GET".into(),
    ///     path_template: "/pets".into(),
    ///     ..Default::default()
    /// });
    /// match registry.decode_attachment(&attachment).unwrap() {
    ///     DecodedAttachment::Http(HttpPayloadKind::Operation(m)) => assert_eq!(m.method, "GET"),
    ///     _ => panic!("expected operation meta"),
    /// }
    /// ```
    pub fn decode_attachment(&self, attachment: &ProtocolAttachment) -> Result<DecodedAttachment> {
        match attachment.protocol_id.as_str() {
            "http" => decode_http(&attachment.payload).map(DecodedAttachment::Http),
            "grpc" => decode_grpc(&attachment.payload).map(DecodedAttachment::Grpc),
            other => Ok(DecodedAttachment::Opaque {
                protocol_id: other.to_string(),
                payload: attachment.payload.clone(),
            }),
        }
    }

    /// Find the first HTTP operation meta on an operation body's attachments.
    pub fn http_operation_from_attachments(
        &self,
        protocols: &[ProtocolAttachment],
    ) -> Option<
        switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::HttpOperationMeta,
    > {
        for attachment in protocols {
            if let Ok(DecodedAttachment::Http(HttpPayloadKind::Operation(meta))) =
                self.decode_attachment(attachment)
            {
                return Some(meta);
            }
        }
        None
    }

    /// Find the first gRPC operation meta on an operation body's attachments.
    pub fn grpc_operation_from_attachments(
        &self,
        protocols: &[ProtocolAttachment],
    ) -> Option<
        switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::GrpcOperationMeta,
    > {
        for attachment in protocols {
            if let Ok(DecodedAttachment::Grpc(GrpcPayloadKind::Operation(meta))) =
                self.decode_attachment(attachment)
            {
                return Some(meta);
            }
        }
        None
    }
}

fn decode_http(bytes: &[u8]) -> Result<HttpPayloadKind> {
    let payload: HttpPayload = decode_message(bytes)?;
    match payload.kind {
        Some(HttpKind::Contract(v)) => Ok(HttpPayloadKind::Contract(*v)),
        Some(HttpKind::Operation(v)) => Ok(HttpPayloadKind::Operation(*v)),
        Some(HttpKind::Response(v)) => Ok(HttpPayloadKind::Response(*v)),
        Some(HttpKind::Error(v)) => Ok(HttpPayloadKind::Error(*v)),
        Some(HttpKind::Parameter(v)) => Ok(HttpPayloadKind::Parameter(*v)),
        None => Err(SwitchbackError::codec("empty HttpPayload")),
    }
}

fn decode_grpc(bytes: &[u8]) -> Result<GrpcPayloadKind> {
    let payload: GrpcPayload = decode_message(bytes)?;
    match payload.kind {
        Some(GrpcKind::Contract(v)) => Ok(GrpcPayloadKind::Contract(*v)),
        Some(GrpcKind::Operation(v)) => Ok(GrpcPayloadKind::Operation(*v)),
        Some(GrpcKind::Status(v)) => Ok(GrpcPayloadKind::Status(*v)),
        Some(GrpcKind::Error(v)) => Ok(GrpcPayloadKind::Error(*v)),
        Some(GrpcKind::Metadata(v)) => Ok(GrpcPayloadKind::Metadata(*v)),
        None => Err(SwitchbackError::codec("empty GrpcPayload")),
    }
}

#[cfg(test)]
mod coverage_matrix {
    use super::*;
    use switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::{
        GrpcContractMeta, GrpcErrorMeta, GrpcMetadataMeta, GrpcOperationMeta, GrpcStatusMeta,
    };
    use switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::{
        HttpContractMeta, HttpErrorMeta, HttpOperationMeta, HttpParameterMeta, HttpResponseMeta,
    };

    #[test]
    fn http_matrix_roundtrips() {
        let registry = ProtocolRegistry::with_builtins();
        let http = registry.http();

        let cases: Vec<(HttpPayloadKind, ProtocolAttachment)> = vec![
            (
                HttpPayloadKind::Contract(HttpContractMeta {
                    default_server_url: "https://api.example.com".into(),
                    ..Default::default()
                }),
                http.attach_contract(&HttpContractMeta {
                    default_server_url: "https://api.example.com".into(),
                    ..Default::default()
                }),
            ),
            (
                HttpPayloadKind::Operation(HttpOperationMeta {
                    method: "GET".into(),
                    path_template: "/pets".into(),
                    ..Default::default()
                }),
                http.attach_operation(&HttpOperationMeta {
                    method: "GET".into(),
                    path_template: "/pets".into(),
                    ..Default::default()
                }),
            ),
            (
                HttpPayloadKind::Response(HttpResponseMeta {
                    status_code: 200,
                    ..Default::default()
                }),
                http.attach_response(&HttpResponseMeta {
                    status_code: 200,
                    ..Default::default()
                }),
            ),
            (
                HttpPayloadKind::Error(HttpErrorMeta {
                    status_code: 404,
                    ..Default::default()
                }),
                http.attach_error(&HttpErrorMeta {
                    status_code: 404,
                    ..Default::default()
                }),
            ),
            (
                HttpPayloadKind::Parameter(HttpParameterMeta {
                    name: "id".into(),
                    location: "path".into(),
                    required: true,
                    ..Default::default()
                }),
                http.attach_parameter(&HttpParameterMeta {
                    name: "id".into(),
                    location: "path".into(),
                    required: true,
                    ..Default::default()
                }),
            ),
        ];

        for (expected_kind, attachment) in cases {
            match registry.decode_attachment(&attachment).unwrap() {
                DecodedAttachment::Http(kind) => assert_eq!(kind, expected_kind),
                other => panic!("expected http decode, got {other:?}"),
            }
        }
    }

    #[test]
    fn grpc_matrix_roundtrips() {
        let registry = ProtocolRegistry::with_builtins();
        let grpc = registry.grpc();

        let cases: Vec<(GrpcPayloadKind, ProtocolAttachment)> = vec![
            (
                GrpcPayloadKind::Contract(GrpcContractMeta {
                    package_name: "acme.v1".into(),
                    ..Default::default()
                }),
                grpc.attach_contract(&GrpcContractMeta {
                    package_name: "acme.v1".into(),
                    ..Default::default()
                }),
            ),
            (
                GrpcPayloadKind::Operation(GrpcOperationMeta {
                    rpc_name: "GetPet".into(),
                    ..Default::default()
                }),
                grpc.attach_operation(&GrpcOperationMeta {
                    rpc_name: "GetPet".into(),
                    ..Default::default()
                }),
            ),
            (
                GrpcPayloadKind::Status(GrpcStatusMeta {
                    code: 0,
                    message: "OK".into(),
                    ..Default::default()
                }),
                grpc.attach_status(&GrpcStatusMeta {
                    code: 0,
                    message: "OK".into(),
                    ..Default::default()
                }),
            ),
            (
                GrpcPayloadKind::Error(GrpcErrorMeta {
                    code: 5,
                    message: "not found".into(),
                    ..Default::default()
                }),
                grpc.attach_error(&GrpcErrorMeta {
                    code: 5,
                    message: "not found".into(),
                    ..Default::default()
                }),
            ),
            (
                GrpcPayloadKind::Metadata(GrpcMetadataMeta {
                    key: "x-request-id".into(),
                    required: false,
                    ..Default::default()
                }),
                grpc.attach_metadata(&GrpcMetadataMeta {
                    key: "x-request-id".into(),
                    required: false,
                    ..Default::default()
                }),
            ),
        ];

        for (expected_kind, attachment) in cases {
            match registry.decode_attachment(&attachment).unwrap() {
                DecodedAttachment::Grpc(kind) => assert_eq!(kind, expected_kind),
                other => panic!("expected grpc decode, got {other:?}"),
            }
        }
    }

    #[test]
    fn opaque_custom_protocol_passthrough() {
        let registry = ProtocolRegistry::with_builtins();
        let attachment = ProtocolAttachment {
            protocol_id: "acme/kafka".into(),
            payload: vec![1, 2, 3],
        };
        match registry.decode_attachment(&attachment).unwrap() {
            DecodedAttachment::Opaque {
                protocol_id,
                payload,
            } => {
                assert_eq!(protocol_id, "acme/kafka");
                assert_eq!(payload, vec![1, 2, 3]);
            }
            other => panic!("expected opaque, got {other:?}"),
        }
    }
}
