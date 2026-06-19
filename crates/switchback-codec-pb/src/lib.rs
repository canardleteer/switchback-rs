#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![allow(async_fn_in_trait)]

//! Reference binary codec for the switchback artifact.
//!
//! `switchback-codec-pb` implements [`SwitchbackCodec`] and
//! [`SyncSwitchbackCodec`] from `switchback-traits` using
//! [buffa](https://github.com/anthropics/buffa)-generated types compiled from
//! [`switchback.proto`](https://github.com/canardleteer/switchback-rs/blob/main/proto/switchback.proto)
//! (`switchback.v1alpha1`). See [ADR 0003](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0003-protobuf-switchback-wire-format-with-buffa-in-switchback-codec-pb.md)
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

mod codec;

/// Low-level protobuf conversion between seam model types and wire messages.
pub mod convert;

/// Buffa-generated `switchback.v1alpha1` protobuf types (build output).
pub mod pb;

pub use codec::{ProtobufCodec, DEFAULT_SWITCHBACK_FILENAME};
pub use convert::WIRE_VERSION;
