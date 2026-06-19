//! Switchback serialization seam (async primary + sync secondary).

use crate::{ReferenceManual, Result};

/// Async codec for I/O-backed serialize/deserialize of [`ReferenceManual`] artifacts.
///
/// Primary API per [ADR 0002](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0002-async-first-traits-with-synchronous-secondary-apis-in-switchback-traits.md).
/// Implementations live in codec crates (e.g. protobuf switchback wire format).
pub trait SwitchbackCodec: Send + Sync {
    /// Serializes a reference manual to switchback wire bytes.
    async fn serialize(&self, manual: &ReferenceManual) -> Result<Vec<u8>>;

    /// Deserializes switchback wire bytes into a reference manual.
    async fn deserialize(&self, bytes: &[u8]) -> Result<ReferenceManual>;
}

/// Synchronous compatibility API for callers that cannot wrap [`SwitchbackCodec`].
///
/// Secondary API per ADR 0002. Prefer [`SwitchbackCodec`] for service-side pipelines.
pub trait SyncSwitchbackCodec: Send + Sync {
    /// Serializes a reference manual to switchback wire bytes.
    fn serialize(&self, manual: &ReferenceManual) -> Result<Vec<u8>>;

    /// Deserializes switchback wire bytes into a reference manual.
    fn deserialize(&self, bytes: &[u8]) -> Result<ReferenceManual>;
}
