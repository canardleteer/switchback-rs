# 6. JSON Schema parser loader and catalog in switchback-jsonschema

Date: 2026-06-20

## Status

Proposed

Relates to
[7. Parser library conventions for switchback family crates](0007-parser-library-conventions-for-switchback-family-crates.md)

Relates to
[8. Per-family grouping rules and jsonschema shared-layer boundary](0008-per-family-grouping-rules-and-jsonschema-shared-layer-boundary.md)

## Context

Phase 1 of spread-it-out requires a shared JSON-Schema-document parser layer
(`switchback-jsonschema`) parallel to `switchback-protobuf`'s
compile-to-descriptors pipeline. Vendored meta-schemas for family parser crates
landed in ADR 0005. OpenAPI, AsyncAPI, and OpenRPC parsers will reuse loader,
resolver, envelope, and schema body production from this crate.

## Decision

Implement `switchback-jsonschema` as a library-first parser crate with:

- **Loader** — read `.yaml`/`.yml`/`.json` and extensionless JSON (OAI
  meta-schemas); collect entry and transitively referenced files; lossless
  source bytes for the switchback source layer.
- **RefResolver** — RFC 6901 internal pointers, external file `$ref`, cycle
  termination markers; URL `$ref` deferred behind optional `url-refs` feature
  (`reqwest`).
- **YAML parsing** — `serde-saphyr` at load time; documents stored as
  `serde_json::Value` internally.
- **Envelope IR** — shared `info`/`servers`/`components`/`tags`/`externalDocs`
  types for Phase 2+ family parsers; lightly populated in standalone catalog
  mode.
- **Schema IR** — internal `Schema`/`SchemaObject` flattened to seam
  `SchemaBody` at populate time.
- **Catalog mode** — `JsonSchemaFamily` with one `Group` per entry file (group
  id = file stem); entities from root schema object plus `$defs`/`definitions`
  entries; structural `$ref` → `StoredEntity.refs`.
- **LinkExtractor stub** — empty `intra_links` initially (mirrors protobuf Phase
  1).
- **Optional features** — `validate` (`jsonschema` crate) for meta-schema
  validation hook; not wired to CLI in this pass. Default features empty; CI
  runs without network.
- **Fixtures** — in-crate `tests/fixtures/catalog/` catalog (protobuf-style
  layout); curated vendored meta-schema paths from sibling crates (ADR 0005) in
  integration tests.

## Consequences

- Phase 2 `switchback-openapi` can depend on shared loader/envelope/schema
  modules instead of duplicating `$ref` resolution.
- Envelope types may later split to `switchback-api` if the shared surface grows
  unwieldy (escape hatch from spread-it-out).
- URL `$ref` and `--validate` remain feature-gated stretch goals until CLI/xtask
  wiring lands.
- Agents should record optional-feature status in `planning/PROGRESS.md` when
  extending this crate.
