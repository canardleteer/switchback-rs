//! gRPC protocol implementation.

use switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::{
    GrpcContractMeta, GrpcErrorMeta, GrpcMetadataMeta, GrpcOperationMeta, GrpcPayload,
    GrpcStatusMeta,
};
use switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::__buffa::oneof::grpc_payload::Kind;
use switchback_traits::{ProtocolAttachment, ResponseSeverity, Result};

use crate::severity::{grpc_status_name_severity, grpc_status_severity};
use crate::traits::{
    ErrorProtocol, FieldCarrierProtocol, OperationProtocol, Protocol, ProtocolWire,
    ResponseProtocol,
};
use crate::wire::{decode_message, encode_message};

/// Built-in gRPC protocol (`"grpc"`).
#[derive(Clone, Copy, Debug, Default)]
pub struct GrpcProtocol;

impl Protocol for GrpcProtocol {
    fn id(&self) -> &'static str {
        "grpc"
    }
}

impl OperationProtocol for GrpcProtocol {
    type OperationMeta = GrpcOperationMeta;

    fn format_signature(&self, meta: &Self::OperationMeta) -> String {
        let streaming = match (meta.client_streaming, meta.server_streaming) {
            (true, true) => "stream ",
            (true, false) => "stream ",
            (false, true) => "",
            (false, false) => "",
        };
        let out_stream = if meta.server_streaming { "stream " } else { "" };
        format!(
            "{} ( {streaming}… ) returns ( {out_stream}… )",
            meta.rpc_name
        )
    }
}

impl ResponseProtocol for GrpcProtocol {
    fn response_severity(&self, status_key: &str) -> ResponseSeverity {
        grpc_status_name_severity(status_key)
    }
}

impl ErrorProtocol for GrpcProtocol {
    type ErrorMeta = GrpcErrorMeta;

    fn error_severity(&self, error_key: &str) -> ResponseSeverity {
        grpc_status_name_severity(error_key)
    }

    fn format_error_label(&self, error_key: &str) -> String {
        format!("gRPC {error_key}")
    }
}

impl FieldCarrierProtocol for GrpcProtocol {
    fn field_carrier_kinds(&self) -> &'static [&'static str] {
        &["metadata"]
    }

    fn valid_parameter_locations(&self) -> &'static [&'static str] {
        &["metadata"]
    }
}

impl GrpcProtocol {
    /// Attach contract-level gRPC metadata.
    pub fn attach_contract(&self, meta: &GrpcContractMeta) -> ProtocolAttachment {
        attachment_from_payload(GrpcPayload {
            kind: Some(Kind::Contract(Box::new(meta.clone()))),
            ..Default::default()
        })
    }

    /// Attach operation-level gRPC metadata.
    pub fn attach_operation(&self, meta: &GrpcOperationMeta) -> ProtocolAttachment {
        attachment_from_payload(GrpcPayload {
            kind: Some(Kind::Operation(Box::new(meta.clone()))),
            ..Default::default()
        })
    }

    /// Attach success status gRPC metadata.
    pub fn attach_status(&self, meta: &GrpcStatusMeta) -> ProtocolAttachment {
        attachment_from_payload(GrpcPayload {
            kind: Some(Kind::Status(Box::new(meta.clone()))),
            ..Default::default()
        })
    }

    /// Attach error gRPC metadata.
    pub fn attach_error(&self, meta: &GrpcErrorMeta) -> ProtocolAttachment {
        attachment_from_payload(GrpcPayload {
            kind: Some(Kind::Error(Box::new(meta.clone()))),
            ..Default::default()
        })
    }

    /// Attach metadata key gRPC metadata.
    pub fn attach_metadata(&self, meta: &GrpcMetadataMeta) -> ProtocolAttachment {
        attachment_from_payload(GrpcPayload {
            kind: Some(Kind::Metadata(Box::new(meta.clone()))),
            ..Default::default()
        })
    }

    /// Build [`GrpcOperationMeta`] from RPC descriptor fields.
    pub fn operation_meta(
        rpc_name: &str,
        client_streaming: bool,
        server_streaming: bool,
    ) -> GrpcOperationMeta {
        GrpcOperationMeta {
            rpc_name: rpc_name.to_string(),
            client_streaming,
            server_streaming,
            ..Default::default()
        }
    }

    /// Build [`GrpcStatusMeta`] for OK responses.
    pub fn status_meta_ok() -> GrpcStatusMeta {
        GrpcStatusMeta {
            code: 0,
            message: "OK".to_string(),
            ..Default::default()
        }
    }

    /// Build [`GrpcErrorMeta`] from a status name.
    pub fn error_meta(code_name: &str, message: &str) -> GrpcErrorMeta {
        let code = grpc_code_from_name(code_name);
        GrpcErrorMeta {
            code,
            message: message.to_string(),
            ..Default::default()
        }
    }

    /// Build [`GrpcMetadataMeta`].
    pub fn metadata_meta(key: &str, required: bool) -> GrpcMetadataMeta {
        GrpcMetadataMeta {
            key: key.to_string(),
            required,
            ..Default::default()
        }
    }

    /// Build [`GrpcContractMeta`].
    pub fn contract_meta(package_name: &str) -> GrpcContractMeta {
        GrpcContractMeta {
            package_name: package_name.to_string(),
            ..Default::default()
        }
    }

    /// Classify gRPC status code for populate.
    pub fn severity_for_code(code: i32) -> ResponseSeverity {
        grpc_status_severity(code)
    }
}

impl ProtocolWire for GrpcOperationMeta {
    const PROTOCOL_ID: &'static str = "grpc";

    fn encode_to_vec(&self) -> Vec<u8> {
        encode_message(&GrpcPayload {
            kind: Some(Kind::Operation(Box::new(self.clone()))),
            ..Default::default()
        })
    }

    fn decode_from_bytes(bytes: &[u8]) -> Result<Self> {
        let payload: GrpcPayload = decode_message(bytes)?;
        match payload.kind {
            Some(Kind::Operation(meta)) => Ok(*meta),
            _ => Err(switchback_traits::SwitchbackError::codec(
                "expected GrpcOperationMeta payload",
            )),
        }
    }
}

impl ProtocolWire for GrpcErrorMeta {
    const PROTOCOL_ID: &'static str = "grpc";

    fn encode_to_vec(&self) -> Vec<u8> {
        encode_message(&GrpcPayload {
            kind: Some(Kind::Error(Box::new(self.clone()))),
            ..Default::default()
        })
    }

    fn decode_from_bytes(bytes: &[u8]) -> Result<Self> {
        let payload: GrpcPayload = decode_message(bytes)?;
        match payload.kind {
            Some(Kind::Error(meta)) => Ok(*meta),
            _ => Err(switchback_traits::SwitchbackError::codec(
                "expected GrpcErrorMeta payload",
            )),
        }
    }
}

fn attachment_from_payload(payload: GrpcPayload) -> ProtocolAttachment {
    let protocol = GrpcProtocol;
    ProtocolAttachment {
        protocol_id: protocol.id().to_string(),
        payload: encode_message(&payload),
    }
}

fn grpc_code_from_name(name: &str) -> i32 {
    match name.trim().to_ascii_uppercase().as_str() {
        "OK" => 0,
        "CANCELLED" => 1,
        "UNKNOWN" => 2,
        "INVALID_ARGUMENT" => 3,
        "DEADLINE_EXCEEDED" => 4,
        "NOT_FOUND" => 5,
        "ALREADY_EXISTS" => 6,
        "PERMISSION_DENIED" => 7,
        "RESOURCE_EXHAUSTED" => 8,
        "FAILED_PRECONDITION" => 9,
        "ABORTED" => 10,
        "OUT_OF_RANGE" => 11,
        "UNIMPLEMENTED" => 12,
        "INTERNAL" => 13,
        "UNAVAILABLE" => 14,
        "DATA_LOSS" => 15,
        "UNAUTHENTICATED" => 16,
        _ => 2,
    }
}
