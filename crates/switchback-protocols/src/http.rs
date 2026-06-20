//! HTTP protocol implementation.

use switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::{
    HttpContractMeta, HttpErrorMeta, HttpOperationMeta, HttpParameterMeta, HttpPayload,
    HttpResponseMeta,
};
use switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::__buffa::oneof::http_payload::Kind;
use switchback_traits::{ProtocolAttachment, ResponseSeverity, Result};

use crate::severity::{http_status_code_severity, http_status_severity};
use crate::traits::{
    ErrorProtocol, FieldCarrierProtocol, OperationProtocol, Protocol, ProtocolWire,
    ResponseProtocol,
};
use crate::wire::{decode_message, encode_message};

/// Built-in HTTP protocol (`"http"`).
#[derive(Clone, Copy, Debug, Default)]
pub struct HttpProtocol;

impl Protocol for HttpProtocol {
    fn id(&self) -> &'static str {
        "http"
    }
}

impl OperationProtocol for HttpProtocol {
    type OperationMeta = HttpOperationMeta;

    fn format_signature(&self, meta: &Self::OperationMeta) -> String {
        format!(
            "{} {}",
            meta.method.to_ascii_uppercase(),
            meta.path_template
        )
    }
}

impl ResponseProtocol for HttpProtocol {
    fn response_severity(&self, status_key: &str) -> ResponseSeverity {
        http_status_severity(status_key)
    }
}

impl ErrorProtocol for HttpProtocol {
    type ErrorMeta = HttpErrorMeta;

    fn error_severity(&self, error_key: &str) -> ResponseSeverity {
        http_status_severity(error_key)
    }

    fn format_error_label(&self, error_key: &str) -> String {
        format!("HTTP {error_key}")
    }
}

impl FieldCarrierProtocol for HttpProtocol {
    fn field_carrier_kinds(&self) -> &'static [&'static str] {
        &["header", "trailer", "cookie"]
    }

    fn valid_parameter_locations(&self) -> &'static [&'static str] {
        &["query", "path", "header", "cookie"]
    }
}

impl HttpProtocol {
    /// Attach contract-level HTTP metadata.
    pub fn attach_contract(&self, meta: &HttpContractMeta) -> ProtocolAttachment {
        attachment_from_payload(HttpPayload {
            kind: Some(Kind::Contract(Box::new(meta.clone()))),
            ..Default::default()
        })
    }

    /// Attach operation-level HTTP metadata.
    pub fn attach_operation(&self, meta: &HttpOperationMeta) -> ProtocolAttachment {
        attachment_from_payload(HttpPayload {
            kind: Some(Kind::Operation(Box::new(meta.clone()))),
            ..Default::default()
        })
    }

    /// Attach success response HTTP metadata.
    pub fn attach_response(&self, meta: &HttpResponseMeta) -> ProtocolAttachment {
        attachment_from_payload(HttpPayload {
            kind: Some(Kind::Response(Box::new(meta.clone()))),
            ..Default::default()
        })
    }

    /// Attach error response HTTP metadata.
    pub fn attach_error(&self, meta: &HttpErrorMeta) -> ProtocolAttachment {
        attachment_from_payload(HttpPayload {
            kind: Some(Kind::Error(Box::new(meta.clone()))),
            ..Default::default()
        })
    }

    /// Attach parameter HTTP metadata.
    pub fn attach_parameter(&self, meta: &HttpParameterMeta) -> ProtocolAttachment {
        attachment_from_payload(HttpPayload {
            kind: Some(Kind::Parameter(Box::new(meta.clone()))),
            ..Default::default()
        })
    }

    /// Build [`HttpOperationMeta`] from method, path, and streaming flags.
    pub fn operation_meta(
        method: &str,
        path: &str,
        request_streaming: bool,
        response_streaming: bool,
    ) -> HttpOperationMeta {
        HttpOperationMeta {
            method: method.to_ascii_uppercase(),
            path_template: path.to_string(),
            request_streaming,
            response_streaming,
            ..Default::default()
        }
    }

    /// Build [`HttpResponseMeta`] from a status key and optional media type.
    pub fn response_meta(status: &str, media_type: &str) -> HttpResponseMeta {
        let status_code = status.parse::<u32>().unwrap_or(0);
        HttpResponseMeta {
            status_code,
            media_type: media_type.to_string(),
            ..Default::default()
        }
    }

    /// Build [`HttpErrorMeta`] from a status key.
    pub fn error_meta(status: &str, description: &str) -> HttpErrorMeta {
        let status_code = status.parse::<u32>().unwrap_or(0);
        HttpErrorMeta {
            status_code,
            detail: description.to_string(),
            ..Default::default()
        }
    }

    /// Build [`HttpParameterMeta`] from OpenAPI parameter fields.
    pub fn parameter_meta(name: &str, location: &str, required: bool) -> HttpParameterMeta {
        HttpParameterMeta {
            name: name.to_string(),
            location: location.to_string(),
            required,
            ..Default::default()
        }
    }

    /// Build [`HttpContractMeta`] from server URLs.
    pub fn contract_meta(server_urls: &[String], default: Option<&str>) -> HttpContractMeta {
        HttpContractMeta {
            server_urls: server_urls.to_vec(),
            default_server_url: default.unwrap_or_default().to_string(),
            ..Default::default()
        }
    }

    /// Classify numeric HTTP status for populate.
    pub fn severity_for_status_code(code: u16) -> ResponseSeverity {
        http_status_code_severity(code)
    }

    /// True when the status code represents an error outcome.
    pub fn is_error_status(status: &str) -> bool {
        matches!(
            http_status_severity(status),
            ResponseSeverity::ClientError | ResponseSeverity::ServerError
        )
    }
}

impl ProtocolWire for HttpOperationMeta {
    const PROTOCOL_ID: &'static str = "http";

    fn encode_to_vec(&self) -> Vec<u8> {
        encode_message(&HttpPayload {
            kind: Some(Kind::Operation(Box::new(self.clone()))),
            ..Default::default()
        })
    }

    fn decode_from_bytes(bytes: &[u8]) -> Result<Self> {
        let payload: HttpPayload = decode_message(bytes)?;
        match payload.kind {
            Some(Kind::Operation(meta)) => Ok(*meta),
            _ => Err(switchback_traits::SwitchbackError::codec(
                "expected HttpOperationMeta payload",
            )),
        }
    }
}

impl ProtocolWire for HttpErrorMeta {
    const PROTOCOL_ID: &'static str = "http";

    fn encode_to_vec(&self) -> Vec<u8> {
        encode_message(&HttpPayload {
            kind: Some(Kind::Error(Box::new(self.clone()))),
            ..Default::default()
        })
    }

    fn decode_from_bytes(bytes: &[u8]) -> Result<Self> {
        let payload: HttpPayload = decode_message(bytes)?;
        match payload.kind {
            Some(Kind::Error(meta)) => Ok(*meta),
            _ => Err(switchback_traits::SwitchbackError::codec(
                "expected HttpErrorMeta payload",
            )),
        }
    }
}

fn attachment_from_payload(payload: HttpPayload) -> ProtocolAttachment {
    let protocol = HttpProtocol;
    ProtocolAttachment {
        protocol_id: protocol.id().to_string(),
        payload: encode_message(&payload),
    }
}
