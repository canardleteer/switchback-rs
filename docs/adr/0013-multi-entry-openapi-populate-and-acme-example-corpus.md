# 13. Multi-entry OpenAPI populate and acme example corpus

Date: 2026-06-20

## Status

Proposed

Relates to
[8. Per-family grouping rules and JSON Schema shared layer boundary](0008-per-family-grouping-rules-and-jsonschema-shared-layer-boundary.md)

Relates to
[10. OpenAPI parser library MVP in switchback-openapi](0010-openapi-parser-library-mvp-in-switchback-openapi.md)

## Context

OpenAPI populate used only the first resolved entry document. The protobuf
example loads eight `.proto` files into one manual with one group per package.
We need the same for OpenAPI so the default mdBook example can render a
three-version Acme API (`acme.example.v1`, `.v2`, `.v3alpha1`) in one book.

## Decision

### Multi-entry populate

When `load()` resolves **multiple entry URIs** into one contract:

| Rule | Behavior |
| --- | --- |
| Group id | One group per entry. Id from `info.x-switchback-group` when set, else path convention (`v1/openapi.yaml` → `acme.example.v1`) |
| Entities | All operations and components from an entry land in that entry's group (not tag-based groups) |
| Module id | Slug from first entry `info.title`, default `acme-apis` for the acme fixture |
| Companions | Union of beside markdown discovered for all entry URIs |
| Contract `protocols[]` | Union of `servers[]` HTTP attachments from all entry roots |
| Spec version | Highest supported OAS version among entries (3.1.0 wins over 3.0.3) |

**Single-entry** loads keep existing tag / `x-tagGroups` / components / untagged
grouping (no behavior change for upstream and micro regressions).

### Acme example corpus

Hand-maintained micro fixture at
`crates/switchback-openapi/tests/fixtures/micro/acme/` with three entry YAML
files mirroring the protobuf Acme packages. Registered as fixture id
**`acme-api`** and the **sole default** for `examples/mdbook-openapi` (upstream
remains via `--tier upstream`).

OpenAPI cannot model every protobuf shape (for example gRPC bidi as one
operation); companion prose documents **OpenAPI** gaps, not HTTP transport
limits.

## Consequences

- `switchback-openapi` populate and `examples.rs` gain multi-entry and acme
  paths.
- mdBook goldens and link-check add an `openapi_acme` scenario.
- Upstream vendored fixtures unchanged; still loaded explicitly.
