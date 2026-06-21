//! Built-in protocol implementations and registry for switchback attachments.
//!
//! Transport semantics are orthogonal to [contract family](https://github.com/canardleteer/switchback-rs/blob/main/docs/GLOSSARY.md#contract-family).
//! Parsers attach [`ProtocolAttachment`](switchback_traits::ProtocolAttachment)
//! envelopes on contract and entity nodes; this crate provides `HttpProtocol`,
//! `GrpcProtocol`, and [`ProtocolRegistry`] to encode and decode built-in payload
//! schemas.
//!
//! # Entity attachment matrix
//!
//! | IR node | `http` payload arm | `grpc` payload arm |
//! | --- | --- | --- |
//! | Contract | `HttpContractMeta` | `GrpcContractMeta` |
//! | Operation | `HttpOperationMeta` | `GrpcOperationMeta` |
//! | Response ref/body | `HttpResponseMeta` / `HttpErrorMeta` | `GrpcStatusMeta` / `GrpcErrorMeta` |
//! | Parameter ref/body | `HttpParameterMeta` | `GrpcMetadataMeta` |
//!
//! Decode: read `protocol_id`, deserialize `payload` as `HttpPayload` or
//! `GrpcPayload`, inspect the oneof kind. See
//! [ADR 0011](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0011-protocol-layer-and-contract-family-binding.md)
//! and [ADR 0012](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0012-http-streaming-inference-and-grpc-metadata-from-protobuf-options.md).

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod amqp;
mod grpc;
mod http;
mod kafka;
mod mqtt;
mod registry;
mod severity;
mod traits;
mod wire;

pub use amqp::AmqpProtocol;
pub use grpc::GrpcProtocol;
pub use http::HttpProtocol;
pub use kafka::KafkaProtocol;
pub use mqtt::MqttProtocol;
pub use registry::{
    AmqpPayloadKind, DecodedAttachment, GrpcPayloadKind, HttpPayloadKind, KafkaPayloadKind,
    MqttPayloadKind, ProtocolRegistry,
};
pub use severity::{
    grpc_status_name_severity, grpc_status_severity, http_status_code_severity,
    http_status_severity,
};
pub use traits::{
    ErrorProtocol, FieldCarrierProtocol, OperationProtocol, Protocol, ProtocolWire,
    ResponseProtocol,
};
