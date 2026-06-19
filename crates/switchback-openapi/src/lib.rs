#![forbid(unsafe_code)]

//! OpenAPI -> switchback parser.
//!
//! `switchback-openapi` implements `ContractFamily` and `Contract` for OpenAPI
//! and turns an OpenAPI Description (3.0.x and 3.1.x) into a switchback. It
//! preserves the contract's version faithfully: 3.0 `nullable` stays
//! `nullable`; 3.1 `type: [T, "null"]` stays a type array. The parser does not
//! normalize or upgrade versions.
//!
//! Entity categories: `schema`, `operation`, `parameter`, `response`,
//! `request-body`, `security-scheme`. Grouping uses `tags` / `x-tagGroups`.
//! It reuses the loader, `$ref` resolver, envelope, and schema renderer from
//! `switchback-jsonschema`.
