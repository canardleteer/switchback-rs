//! Built-in protocol implementations and registry for switchback attachments.
//!
//! See [ADR 0011](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0011-protocol-layer-and-contract-family-binding.md).

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod grpc;
mod http;
mod registry;
mod severity;
mod traits;
mod wire;

pub use grpc::GrpcProtocol;
pub use http::HttpProtocol;
pub use registry::{DecodedAttachment, GrpcPayloadKind, HttpPayloadKind, ProtocolRegistry};
pub use severity::{
    grpc_status_name_severity, grpc_status_severity, http_status_code_severity,
    http_status_severity,
};
pub use traits::{
    ErrorProtocol, FieldCarrierProtocol, OperationProtocol, Protocol, ProtocolWire,
    ResponseProtocol,
};
