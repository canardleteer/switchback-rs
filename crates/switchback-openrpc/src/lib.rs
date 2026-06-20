#![forbid(unsafe_code)]

//! OpenRPC -> switchback parser.
//!
//! `switchback-openrpc` is a thin behavior layer over JSON Schema content descriptors. An OpenRPC document has
//! `info`, `servers`, `methods`, and `components`; a method has `params`
//! (`ContentDescriptor[]`) and `result` (a `ContentDescriptor`). The parser
//! maps `methods` -> `Operation` entities and content descriptors / component
//! schemas -> `Schema` entities, reusing the loader, `$ref` resolver,
//! envelope, and schema renderer from `switchback-jsonschema` verbatim.
//!
//! Entity categories: `operation`, `schema`, `parameter`. Grouping uses
//! `x-tagGroup`; the module is the service the OpenRPC document describes. The
//! whole family-specific surface is roughly two trait impls plus a
//! method-to-operation mapper.

pub mod meta_schemas;
