#![forbid(unsafe_code)]

//! Protobuf -> switchback parser.
//!
//! `switchback-protobuf` implements `ContractFamily` and `Contract` for
//! protobuf and turns a set of `.proto` files into a switchback. Its parser
//! strategy is "compile to descriptors": it runs `buf build` / `protoc` /
//! reads a prebuilt `--descriptor-set` to produce a `FileDescriptorSet`, then
//! populates the switchback from it. This is the protobuf-specific periphery
//! of `protobuf-mdbook` (plugin protocol, input pipeline, `SourceCodeInfo`
//! span extraction, fence synthesis, CEL/Protovalidate extraction) rewritten
//! as the parser side of the seam.
//!
//! Entity categories: `service`, `schema` (messages + enums), `operation`.
//! Grouping uses the protobuf package. It depends only on
//! `switchback-traits`; the protoc-plugin stdin/stdout entry is kept as an
//! optional convenience.
