//! Switchback serialization seam (async primary + sync secondary).

use crate::{ReferenceManual, Result};

/// Async codec for I/O-backed serialize/deserialize.
pub trait SwitchbackCodec: Send + Sync {
    async fn serialize(&self, manual: &ReferenceManual) -> Result<Vec<u8>>;
    async fn deserialize(&self, bytes: &[u8]) -> Result<ReferenceManual>;
}

/// Synchronous compatibility API for callers that cannot wrap [`SwitchbackCodec`].
pub trait SyncSwitchbackCodec: Send + Sync {
    fn serialize(&self, manual: &ReferenceManual) -> Result<Vec<u8>>;
    fn deserialize(&self, bytes: &[u8]) -> Result<ReferenceManual>;
}
