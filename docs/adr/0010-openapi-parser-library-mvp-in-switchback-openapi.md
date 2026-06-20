# 10. OpenAPI parser library MVP in switchback-openapi

Date: 2026-06-20

## Status

Proposed

Relates to
[7. Parser library conventions for switchback family crates](0007-parser-library-conventions-for-switchback-family-crates.md)

Relates to
[8. Per-family grouping rules and jsonschema shared-layer boundary](0008-per-family-grouping-rules-and-jsonschema-shared-layer-boundary.md)

## Context

ADR 0008 pins OpenAPI grouping at a high level; implementation details for
entity naming, multi-tag operations, webhooks, component placement, and
intra-link deferral were left to the first behavior parser. `switchback-openapi`
must land a library-first `load()` pipeline reusing `switchback-jsonschema` and
pair with `switchback-mdbook` for render regression without CLI or `xtask parse`
gates.

## Decision

Implement `switchback-openapi` as a library-first parser per ADR 0007:

- **Versions:** OpenAPI 3.0.x and 3.1.x only. Read `openapi` from the document
  root; reject Swagger 2.0 (`swagger: "2.0"`) at load with a clear error.
  Preserve authored schema shape (`nullable` vs `type: [T, "null"]`); no
  normalization or upgrade.
- **Entity naming:** Operations use `{METHOD} {path}` (e.g.
  `GET /pets/{petId}`). Component entities use the map key in `components.*`.
  Webhooks use the same operation shape as path operations
  (`webhooks.*.{method}`).
- **Grouping:** One `Group` per root `tags[]` name (dir = slugified tag);
  **`untagged`** for operations with no tags after merging operation and
  path-item tags; **`components`** for all reusable component entities.
  `x-tagGroups` orders tag groups in the manual but does not create extra
  groups.
- **Multi-tag operations:** Duplicate the operation entity into **each** tag
  group it belongs to (distinct `EntityId` per group; same display name).
- **Component schemas:** Always populate in **`components`**. Untagged
  operations still reference component entities in `components`; schemas are not
  copied into `untagged`.
- **Structural refs:** `$ref` under `#/components/...` → `RefKind::Component`
  with category from the pointer segment (`schemas`, `parameters`, etc.).
  Prose **intra-links** deferred; `OpenApiLinkExtractor` returns empty
  `intra_links` (library-first, mirrors jsonschema).
- **Companions:** `CompanionDiscovery::Beside` — `{stem}.md` beside each entry
  OpenAPI file; implemented in-crate until a shared traits helper lands (ADR
  0009).
- **Fixtures:** Upstream OAI examples via `example-fixtures.lock.toml` and
  hand-maintained **micro** fixtures for isolated regressions (see AGENTS.md).

## Consequences

- OpenAPI populate must implement tag duplication and a dedicated `components`
  group for all `$ref` targets from operations.
- mdBook renderer gains family-aware path indexing and YAML fences for OpenAPI
  manuals; protobuf paths unchanged.
- CLI, `xtask parse`, `--validate`, Swagger 2.0, and prose intra-links remain
  follow-up work tracked in `planning/PROGRESS.md`.
