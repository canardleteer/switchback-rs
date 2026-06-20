# 7. Parser library conventions for switchback family crates

Date: 2026-06-20

## Status

Proposed

Relates to
[9. Companion nav metadata on wire in switchback-traits](0009-companion-nav-metadata-on-wire-in-switchback-traits.md)

## Context

Early parser libraries (`switchback-protobuf`, `switchback-jsonschema`)
established a repeatable shape before OpenAPI/AsyncAPI/OpenRPC behavior parsers
land. Conventions were established in those crates but not recorded as an ADR
until now. Future family parsers and agents need a single authoritative
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
   protobuf FQN prose extraction populates `intra_links` via
   `ProtobufFqnLinkExtractor`. Field-level links inside protobuf fences are
   out of scope (renderer places links in RPC signature and doc prose).
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
