//! HTTP protocol attachment helpers for OpenAPI populate.

use serde_json::Value;
use switchback_protocols::{ErrorProtocol, HttpProtocol, OperationProtocol, ResponseProtocol};
use switchback_traits::{ProtocolAttachment, ResponseRef};

const STREAMING_RESPONSE_MEDIA: &[&str] = &[
    "text/event-stream",
    "application/stream+json",
    "application/x-ndjson",
];
const STREAMING_REQUEST_MEDIA: &[&str] = &["application/octet-stream"];

/// Build contract-level HTTP attachments from OpenAPI `servers`.
pub fn contract_attachments(root: &Value) -> Vec<ProtocolAttachment> {
    let http = HttpProtocol;
    let servers: Vec<String> = root
        .get("servers")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|s| s.get("url").and_then(|u| u.as_str()).map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    if servers.is_empty() {
        return Vec::new();
    }
    let default = servers.first().map(String::as_str);
    vec![http.attach_contract(&HttpProtocol::contract_meta(&servers, default))]
}

/// Infer HTTP streaming flags from OpenAPI operation content media types (ADR 0012).
pub fn infer_streaming_flags(op_value: &Value) -> (bool, bool) {
    let request_streaming = op_value
        .get("requestBody")
        .and_then(|body| body.get("content"))
        .and_then(|content| content.as_object())
        .is_some_and(|content| {
            content
                .keys()
                .any(|media| STREAMING_REQUEST_MEDIA.contains(&media.as_str()))
        });

    let response_streaming = op_value
        .get("responses")
        .and_then(|responses| responses.as_object())
        .is_some_and(|responses| {
            responses.iter().any(|(status, response)| {
                if status.starts_with('x') {
                    return false;
                }
                response
                    .get("content")
                    .and_then(|content| content.as_object())
                    .is_some_and(|content| {
                        content
                            .keys()
                            .any(|media| STREAMING_RESPONSE_MEDIA.contains(&media.as_str()))
                    })
            })
        });

    (request_streaming, response_streaming)
}

/// Attach HTTP operation metadata and format signature.
pub fn populate_operation(
    method: &str,
    path: &str,
    op_value: &Value,
) -> (String, Vec<ProtocolAttachment>) {
    let http = HttpProtocol;
    let (request_streaming, response_streaming) = infer_streaming_flags(op_value);
    let meta = HttpProtocol::operation_meta(method, path, request_streaming, response_streaming);
    let signature = http.format_signature(&meta);
    let protocols = vec![http.attach_operation(&meta)];
    (signature, protocols)
}

/// Attach HTTP parameter metadata.
pub fn parameter_attachment(name: &str, location: &str, required: bool) -> ProtocolAttachment {
    let http = HttpProtocol;
    http.attach_parameter(&HttpProtocol::parameter_meta(name, location, required))
}

/// Build a [`ResponseRef`] with HTTP protocol attachments and severity.
pub fn response_ref(
    status: &str,
    description: &str,
    media_type: &str,
    schema_ref: switchback_traits::Reference,
) -> ResponseRef {
    let http = HttpProtocol;
    let severity = if HttpProtocol::is_error_status(status) {
        http.error_severity(status)
    } else {
        http.response_severity(status)
    };
    let protocols = if HttpProtocol::is_error_status(status) {
        vec![http.attach_error(&HttpProtocol::error_meta(status, description))]
    } else {
        vec![http.attach_response(&HttpProtocol::response_meta(status, media_type))]
    };
    ResponseRef {
        status: status.to_string(),
        severity,
        schema_ref,
        media_type: media_type.to_string(),
        description: description.to_string(),
        protocols,
    }
}

/// Attach HTTP parameter metadata on a standalone parameter body.
pub fn parameter_body_protocols(
    name: &str,
    location: &str,
    required: bool,
) -> Vec<ProtocolAttachment> {
    vec![parameter_attachment(name, location, required)]
}

/// Attach HTTP response metadata on a standalone response body.
pub fn response_body_protocols(
    status: &str,
    description: &str,
    media_type: &str,
) -> (switchback_traits::ResponseSeverity, Vec<ProtocolAttachment>) {
    let http = HttpProtocol;
    let severity = if HttpProtocol::is_error_status(status) {
        http.error_severity(status)
    } else {
        http.response_severity(status)
    };
    let protocols = if HttpProtocol::is_error_status(status) {
        vec![http.attach_error(&HttpProtocol::error_meta(status, description))]
    } else {
        vec![http.attach_response(&HttpProtocol::response_meta(status, media_type))]
    };
    (severity, protocols)
}
