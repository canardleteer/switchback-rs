#![forbid(unsafe_code)]

//! The reference binary codec for the switchback.
//!
//! `switchback-codec-pb` implements the `SwitchbackCodec` trait from
//! `switchback-traits` using `buffa`-generated types compiled from
//! `proto/switchback.proto` (`switchback.v1alpha1`). A switchback binary file is the canonical,
//! deterministic, cacheable serialized form of a reference manual; every parser
//! emits it and every renderer reads it.
//!
//! The codec is a binary IDL format. Protobuf is the reference implementation.
