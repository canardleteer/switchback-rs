#![forbid(unsafe_code)]

//! AsyncAPI -> switchback parser.
//!
//! `switchback-asyncapi` implements `ContractFamily` and `Contract` for
//! AsyncAPI and turns an AsyncAPI document (2.x and 3.x) into a switchback. It
//! preserves the contract's version faithfully: 2.x channel-embedded
//! `subscribe`/`publish` operations stay embedded; 3.x top-level `operations`
//! stay top-level. The parser does not synthesize 3.x from 2.x.
//!
//! Entity categories: `channel`, `operation`, `message`, `schema`. Grouping
//! uses the application `id` (or `info.title` slug) with `tags` for
//! sub-grouping. It reuses the loader, `$ref` resolver, envelope, and schema
//! renderer from `switchback-jsonschema`. Mermaid sequence diagrams are
//! generated from operations and replies (an AsyncAPI-specific feature with no
//! OpenAPI analog).
