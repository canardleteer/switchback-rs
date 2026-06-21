#![forbid(unsafe_code)]
#![allow(missing_docs)]

//! JSON Schema document parser layer and standalone catalog loader.
//!
//! See the crate README for the public API surface.

pub mod category;
pub mod companion;
pub mod contract;
pub mod envelope;
pub mod examples;
pub mod family;
pub mod link;
pub mod load;
pub mod loader;
pub mod manual;
pub mod paths;
pub mod populate;
pub mod resolver;
pub mod schema;

#[cfg(feature = "validate")]
pub mod validate;

#[cfg(feature = "url-refs")]
pub mod url_refs;

pub use category::JsonSchemaCategory;
pub use contract::JsonSchemaContract;
pub use envelope::{
    Components, Envelope, ExternalDocs, Info, Reference as EnvelopeReference, Server, Tag,
};
pub use family::JsonSchemaFamily;
pub use link::JsonSchemaLinkExtractor;
pub use load::{LoadArgs, load, resolve_inputs};
pub use loader::{Doc, Loader, Resolved};
pub use manual::restore_sources;
pub use resolver::{NodeRef, RefIndex, RefResolver};
pub use schema::{Composite, JsonType, Schema, SchemaObject, populate_schema_body};
