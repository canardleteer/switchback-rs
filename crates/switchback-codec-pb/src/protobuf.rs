//! Buffa-generated protobuf types, exposed by protobuf package path.
//!
//! Types are produced at build time from [`switchback.proto`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-codec-pb/proto/canardleteer/switchback/v1alpha1/switchback.proto).
//! Generated items inherit buffa/rustdoc from the `.proto` file; this module suppresses
//! `missing_docs` on that output.
//!
//! External tools should depend on the layered path, e.g.
//! `switchback_codec_pb::canardleteer::switchback::v1alpha1::ReferenceManual`, so
//! additional package versions can ship alongside without a breaking rename.

#[allow(
    dead_code,
    missing_docs,
    unused_imports,
    clippy::all,
    clippy::pedantic,
    clippy::nursery
)]
pub(crate) mod generated {
    include!(concat!(env!("OUT_DIR"), "/_include.rs"));
}

/// Protobuf package prefix `canardleteer.*`.
pub mod canardleteer {
    /// Protobuf package `canardleteer.switchback.*`.
    pub mod switchback {
        /// Built-in protocol packages under `canardleteer.switchback.protocol.*`.
        pub mod protocol {
            /// HTTP protocol schema `canardleteer.switchback.protocol.http.v1alpha1`.
            pub mod http {
                /// Unstable HTTP protocol schema.
                pub mod v1alpha1 {
                    pub use crate::protobuf::generated::canardleteer::switchback::protocol::http::v1alpha1::*;
                }
            }
            /// gRPC protocol schema `canardleteer.switchback.protocol.grpc.v1alpha1`.
            pub mod grpc {
                /// Unstable gRPC protocol schema.
                pub mod v1alpha1 {
                    pub use crate::protobuf::generated::canardleteer::switchback::protocol::grpc::v1alpha1::*;
                }
            }
        }

        /// Unstable schema `canardleteer.switchback.v1alpha1`.
        pub mod v1alpha1 {
            pub use crate::protobuf::generated::canardleteer::switchback::v1alpha1::*;
        }
    }
}
