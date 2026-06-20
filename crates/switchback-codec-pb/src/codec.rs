//! [`ProtobufCodec`] — buffa wire encoding for the switchback artifact.

use buffa::Message;
use switchback_traits::{ReferenceManual, Result, SwitchbackCodec, SyncSwitchbackCodec};

use crate::convert;
use crate::pb;

/// Default on-disk filename for a switchback binary file.
pub const DEFAULT_SWITCHBACK_FILENAME: &str = "switchback.binpb";

/// Reference protobuf codec using buffa-generated types from `canardleteer.switchback.v1alpha1`.
///
/// Implements [`SwitchbackCodec`] (async-primary) and [`SyncSwitchbackCodec`]
/// per [ADR 0002](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0002-async-first-traits-with-synchronous-secondary-apis-in-switchback-traits.md).
/// Encode and decode are in-memory and synchronous; the async trait methods
/// delegate to the same helpers without blocking on I/O.
#[derive(Debug, Default, Clone, Copy)]
pub struct ProtobufCodec;

impl ProtobufCodec {
    /// Serialize a [`ReferenceManual`] to protobuf wire bytes.
    pub fn serialize_sync(&self, manual: &ReferenceManual) -> Result<Vec<u8>> {
        let proto = convert::to_proto(manual)?;
        Ok(proto.encode_to_vec())
    }

    /// Deserialize protobuf wire bytes into a [`ReferenceManual`].
    pub fn deserialize_sync(&self, bytes: &[u8]) -> Result<ReferenceManual> {
        let proto = pb::ReferenceManual::decode_from_slice(bytes)
            .map_err(|err| switchback_traits::SwitchbackError::codec(err.to_string()))?;
        convert::from_proto(proto)
    }
}

impl SwitchbackCodec for ProtobufCodec {
    async fn serialize(&self, manual: &ReferenceManual) -> Result<Vec<u8>> {
        self.serialize_sync(manual)
    }

    async fn deserialize(&self, bytes: &[u8]) -> Result<ReferenceManual> {
        self.deserialize_sync(bytes)
    }
}

impl SyncSwitchbackCodec for ProtobufCodec {
    fn serialize(&self, manual: &ReferenceManual) -> Result<Vec<u8>> {
        self.serialize_sync(manual)
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<ReferenceManual> {
        self.deserialize_sync(bytes)
    }
}
