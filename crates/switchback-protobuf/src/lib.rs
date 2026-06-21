#![forbid(unsafe_code)]
#![allow(missing_docs)] // lifted protobuf-mdbook internals; public API documented in README

//! Protobuf → switchback parser.
//!
//! `switchback-protobuf` implements [`ContractFamily`](switchback_traits::ContractFamily) and
//! [`Contract`](switchback_traits::Contract) for protobuf and turns a set of `.proto` files
//! into a [`ReferenceManual`](switchback_traits::ReferenceManual). Its parser strategy is
//! "compile to descriptors": it runs `buf build` / `protoc` to produce a
//! `FileDescriptorSet`, then populates the switchback from it.
//!
//! # Example
//!
//! ```no_run
//! use std::path::PathBuf;
//! use switchback_protobuf::{load, input::Compiler, LoadArgs, examples::{fixtures_proto_dir, EXAMPLE_PROTO_INPUTS}};
//!
//! let export = switchback_protobuf::default_proto_deps_export();
//! let _ = switchback_protobuf::ensure_test_proto_deps(&fixtures_proto_dir(), None);
//! let args = LoadArgs {
//!     compiler: Compiler::Protoc,
//!     module_root: fixtures_proto_dir(),
//!     inputs: EXAMPLE_PROTO_INPUTS.iter().map(|p| PathBuf::from(*p)).collect(),
//!     proto_paths: vec![fixtures_proto_dir()],
//!     protoc_path: None,
//!     buf_path: None,
//!     proto_deps_export: Some(export),
//!     title: None,
//! };
//! let manual = load(&args).expect("load protos");
//! assert!(!manual.modules.is_empty());
//! ```

pub mod category;
pub mod companion;
pub mod contract;
pub mod descriptor;
pub mod descriptor_util;
pub mod examples;
pub mod family;
pub mod input;
pub mod link;
pub mod load;
pub mod manual;
pub mod paths;
pub mod populate;
pub mod proto_deps;

pub use category::ProtobufCategory;
pub use contract::ProtobufContract;
pub use family::ProtobufFamily;
pub use input::Compiler;
pub use link::{ProtobufFqnLinkExtractor, ProtobufLinkExtractor};
pub use load::{LoadArgs, default_proto_deps_export, ensure_test_proto_deps, load};
pub use manual::restore_sources;
