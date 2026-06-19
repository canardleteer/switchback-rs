//! Buffa-generated protobuf types for `switchback.v1alpha1`.
//!
//! Types are produced at build time from [`switchback.proto`](https://github.com/canardleteer/switchback-rs/blob/main/proto/switchback.proto).
//! Generated items inherit buffa/rustdoc from the `.proto` file; this module suppresses
//! `missing_docs` on that output.

#[allow(
    dead_code,
    missing_docs,
    unused_imports,
    clippy::all,
    clippy::pedantic,
    clippy::nursery
)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/_include.rs"));
}

pub use generated::switchback::v1alpha1::*;
