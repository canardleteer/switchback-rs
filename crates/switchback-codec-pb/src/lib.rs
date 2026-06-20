#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![allow(async_fn_in_trait)]

//! Reference binary codec for the switchback artifact.
//!
//! `switchback-codec-pb` implements [`SwitchbackCodec`] and
//! [`SyncSwitchbackCodec`] from `switchback-traits` using
//! [buffa](https://github.com/anthropics/buffa)-generated types compiled from
//! [`switchback.proto`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-codec-pb/proto/canardleteer/switchback/v1alpha1/switchback.proto)
//! (`canardleteer.switchback.v1alpha1`). See [ADR 0003](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0003-protobuf-switchback-wire-format-with-buffa-in-switchback-codec-pb.md)
//! for wire-format policy.
//!
//! # Example
//!
//! ```
//! use switchback_codec_pb::ProtobufCodec;
//! use switchback_traits::{ReferenceManual, SyncSwitchbackCodec};
//!
//! let manual = ReferenceManual {
//!     switchback_version: "v1alpha1".into(),
//!     title: "Example".into(),
//!     ..Default::default()
//! };
//! let codec = ProtobufCodec;
//! let bytes = codec.serialize(&manual).unwrap();
//! let round_trip = codec.deserialize(&bytes).unwrap();
//! assert_eq!(round_trip.title, "Example");
//! ```
//!
//! Wire message types live under [`canardleteer::switchback::v1alpha1`].

mod codec;
mod protobuf;

/// Low-level protobuf conversion between seam model types and wire messages.
pub mod convert;

/// Buffa-generated protobuf types, namespaced by protobuf package path.
pub use protobuf::canardleteer;

/// Codec-internal alias for the active protobuf schema; prefer
/// [`canardleteer::switchback::v1alpha1`] in external tools.
pub(crate) mod pb {
    pub use crate::protobuf::generated::canardleteer::switchback::v1alpha1::*;
}

pub use codec::{ProtobufCodec, DEFAULT_SWITCHBACK_FILENAME};
pub use convert::WIRE_VERSION;
