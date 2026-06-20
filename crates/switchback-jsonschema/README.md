# switchback-jsonschema

Shared JSON-Schema-document parser layer and standalone catalog loader for the
switchback-rs toolchain.

`switchback-jsonschema` loads YAML/JSON documents, resolves `$ref`s, models the
shared OpenAPI/AsyncAPI/OpenRPC **envelope**, and produces
**schema entity bodies** for downstream family parsers. In **catalog mode** it
implements
[`ContractFamily`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-traits/src/traits/contract_family.rs)
and turns a directory of JSON Schema files into a
[`ReferenceManual`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-traits/src/model/manual.rs).

Library-only in this pass: CLI, mdBook renderer, and `xtask parse` remain
deferred.

## Usage

```rust
use std::path::PathBuf;
use switchback_jsonschema::{
    examples::{fixtures_catalog_dir, EXAMPLE_CATALOG_INPUTS},
    load, LoadArgs,
};

let module_root = fixtures_catalog_dir();
let args = LoadArgs {
    module_root: module_root.clone(),
    inputs: EXAMPLE_CATALOG_INPUTS
        .iter()
        .map(|p| module_root.join(p))
        .collect(),
    search_roots: vec![module_root.clone()],
    title: None,
};

let manual = load(&args)?;
```

Serialize with
[`switchback-codec-pb`](https://github.com/canardleteer/switchback-rs/tree/main/crates/switchback-codec-pb)
(`ProtobufCodec`, default filename `switchback.binpb`).

## Shared-layer re-exports

OpenAPI, AsyncAPI, and OpenRPC family parsers (`switchback-openapi`,
`switchback-asyncapi`, `switchback-openrpc`) can depend on these without
duplicating:

- `loader::{Loader, Doc, Resolved}`
- `resolver::{RefResolver, RefIndex, NodeRef}`
- `envelope::{Envelope, Info, Server, Components, Tag, ExternalDocs, ...}`
- `schema::{Schema, SchemaObject, populate_schema_body}`

## Cargo features

Default features are **empty**; CI and pass-through tests run with no network.

| Feature | Dependency | Purpose |
| --- | --- | --- |
| `url-refs` | `reqwest` | HTTPS `$ref` fetch + cache (stretch; off by default) |
| `validate` | `jsonschema` | Optional meta-schema validation hook (not wired to CLI yet) |

## Partial implementation

`JsonSchemaLinkExtractor` implements the trait but returns empty
`intra_links`; prose `[Name](…)` extraction is deferred. Structural `$ref`
cross-links populate `StoredEntity.refs`.

See
[ADR 0006](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0006-json-schema-parser-loader-and-catalog-in-switchback-jsonschema.md).

## Fixtures

Integration tests use a catalog under `tests/fixtures/catalog/`
(external/internal `$ref`, cyclic chain, companion markdown), mirroring
[`switchback-protobuf`](https://github.com/canardleteer/switchback-rs/tree/main/crates/switchback-protobuf)'s
in-crate fixture layout. Curated vendored meta-schema paths from
`switchback-openapi`, `switchback-openrpc` (see ADR 0005).

Tests assert loader resolution, `ProtobufCodec` round-trip, directory-faithful
source restoration, and structural smoke on `$ref` indexes.

## Architecture

See
[ADR 0006](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0006-json-schema-parser-loader-and-catalog-in-switchback-jsonschema.md).
