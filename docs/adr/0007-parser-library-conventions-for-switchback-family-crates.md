# 7. Parser library conventions for switchback family crates

Date: 2026-06-20

## Status

Proposed

Relates to
[4. Protobuf parser compile-to-descriptors in switchback-protobuf](0004-protobuf-parser-compile-to-descriptors-in-switchback-protobuf.md)

Relates to
[6. JSON Schema parser loader and catalog in switchback-jsonschema](0006-json-schema-parser-loader-and-catalog-in-switchback-jsonschema.md)

Relates to
[8. Per-family grouping rules and jsonschema shared-layer boundary](0008-per-family-grouping-rules-and-jsonschema-shared-layer-boundary.md)

## Context

Phase 1 parser libraries (`switchback-protobuf`, `switchback-jsonschema`)
established a repeatable shape before OpenAPI/AsyncAPI/OpenRPC behavior parsers
land. Conventions were pinned informally in planning/PROGRESS.md but not
recorded as an ADR. Future family parsers and agents need a single authoritative
reference.

## Decision

All switchback family parser crates follow these library conventions until
superseded:

1. **Public entry** — `load(args) -> Result<ReferenceManual>`; internal errors
   (e.g. `anyhow`) map to `SwitchbackError::load` at the crate boundary.
2. **Assembly pipeline** — `populate/` builds a `PopulatedContract` (groups,
   entities-by-group, companions); `manual::build_reference_manual()` flattens
   to wire `StoredEntity` values and the lossless source layer.
3. **`Contract::entities()`** — returns an empty slice on loaded contract views;
   entities live in populate-side maps until manual assembly. `Contract` is for
   trait wiring, not the primary traversal API.
4. **Companions** — each parser crate implements discovery per its
   `CompanionStrategy`; logic is not centralized in `switchback-traits` yet.
5. **Provenance** — `StoredEntity.source` and parser-side `Span` are deferred in
   the library phase; group `source` and fence bodies carry provenance for now.
6. **LinkExtractor** — structural cross-refs populate `StoredEntity.refs`;
   `intra_links` stay empty until prose extraction is ported.
7. **JSON Schema families** — OpenAPI, AsyncAPI, and OpenRPC reuse
   `switchback-jsonschema` loader, resolver, envelope shell, and schema body
   producer; family-specific behavior IR stays in each family crate (see ADR
   0008).

Exemplars: ADR 0004 (`switchback-protobuf`), ADR 0006 (`switchback-jsonschema`
catalog mode).

## Consequences

- OpenAPI/AsyncAPI/OpenRPC parsers can mirror protobuf/jsonschema without
  inventing alternate assembly patterns.
- Renderers and LinkExtractors must not assume `Contract::entities()` is
  populated during library-only phases.
- Companion and provenance centralization remain follow-up work in traits or a
  helper crate.
