//! Protocol trait seams.

use switchback_traits::{ResponseSeverity, Result};

/// Stable protocol slug plus metadata.
pub trait Protocol: Send + Sync + 'static {
    /// Protocol slug (for example `"http"`, `"grpc"`).
    fn id(&self) -> &'static str;
}

/// Encode/decode the typed payload inside [`ProtocolAttachment`](switchback_traits::ProtocolAttachment).
pub trait ProtocolWire: Sized {
    /// Protocol slug this wire type belongs to.
    const PROTOCOL_ID: &'static str;

    /// Serialize this payload to bytes for attachment storage.
    fn encode_to_vec(&self) -> Vec<u8>;

    /// Deserialize bytes produced by [`Self::encode_to_vec`].
    fn decode_from_bytes(bytes: &[u8]) -> Result<Self>;
}

/// Operation documentation aspects.
pub trait OperationProtocol: Protocol {
    /// Typed operation metadata payload.
    type OperationMeta: ProtocolWire;

    /// Human-facing operation signature line.
    fn format_signature(&self, meta: &Self::OperationMeta) -> String;

    /// Optional title hint derived from operation metadata.
    fn operation_title_hint(&self, meta: &Self::OperationMeta) -> Option<String> {
        let _ = meta;
        None
    }
}

/// Response outcome mapping.
pub trait ResponseProtocol: Protocol {
    /// Map a family-specific status key to [`ResponseSeverity`].
    fn response_severity(&self, status_key: &str) -> ResponseSeverity;
}

/// Error / fault documentation aspects.
pub trait ErrorProtocol: Protocol {
    /// Typed error metadata payload.
    type ErrorMeta: ProtocolWire;

    /// Map an error key to [`ResponseSeverity`].
    fn error_severity(&self, error_key: &str) -> ResponseSeverity;

    /// Human-readable error label for documentation.
    fn format_error_label(&self, error_key: &str) -> String;
}

/// Named fields carried outside the message body.
pub trait FieldCarrierProtocol: Protocol {
    /// Valid carrier kinds for this protocol.
    fn field_carrier_kinds(&self) -> &'static [&'static str];

    /// Valid parameter locations for this protocol.
    fn valid_parameter_locations(&self) -> &'static [&'static str];
}
