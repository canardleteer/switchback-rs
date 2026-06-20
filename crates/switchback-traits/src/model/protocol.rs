//! Protocol attachment envelope on contract and entity nodes.

/// Opaque protocol binding stored on contract and entity IR nodes.
///
/// The `payload` bytes encode exactly one arm of a protocol-specific top-level
/// oneof (for example `HttpPayload` or `GrpcPayload`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProtocolAttachment {
    /// Stable protocol slug (for example `"http"`, `"grpc"`).
    pub protocol_id: String,
    /// Encoded protocol-specific payload message.
    pub payload: Vec<u8>,
}
