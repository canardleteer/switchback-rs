#![forbid(unsafe_code)]

//! The shared JSON-Schema-document parser layer.
//!
//! `switchback-jsonschema` is the off-the-shelf parts bin for any
//! JSON-Schema-based contract family. It layers, on top of
//! `switchback-traits`:
//!
//! - a document `Loader` and `$ref` resolver (internal JSON Pointer, external
//!   file refs, cycle detection; URL refs behind a feature flag),
//! - the shared API-description envelope (`info`/`servers`/`components`/
//!   `security`/`tags`/`externalDocs`) shared by OpenAPI, AsyncAPI, and
//!   OpenRPC,
//! - a schema entity-body producer (fences, property listing, `$ref`
//!   cross-links, examples) that populates the switchback rather than
//!   rendering.
//!
//! `switchback-openapi`, `switchback-asyncapi`, and `switchback-openrpc` build
//! on it. Renderers never see this crate; they consume the switchback. It also
//! ships a standalone CLI that parses a directory of JSON Schema files into a
//! switchback binary file.
