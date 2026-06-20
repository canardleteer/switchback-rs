# 8. Per-family grouping rules and jsonschema shared-layer boundary

Date: 2026-06-20

## Status

Proposed

Relates to
[5. Vendored JSON Schema meta-schemas per parser crate](0005-vendored-json-schema-meta-schemas-per-parser-crate.md)

Relates to
[6. JSON Schema parser loader and catalog in switchback-jsonschema](0006-json-schema-parser-loader-and-catalog-in-switchback-jsonschema.md)

Relates to
[7. Parser library conventions for switchback family crates](0007-parser-library-conventions-for-switchback-family-crates.md)

## Context

Before Phase 2 switchback-openapi, grouping rules and the boundary between
switchback-jsonschema shared layer and family-specific behavior IR were pinned
informally in planning/PROGRESS.md. Spread-it-out defines different grouping per
family and a shared envelope for OpenAPI/AsyncAPI/OpenRPC. ADR 0007 records
parser conventions; this ADR records populate grouping and IR placement.

## Decision

Per-family grouping rules, envelope/IR boundary, and AsyncAPI meta-schema
corpus selection are pinned as follows.

### Grouping (populate → Group)

| Family | Group key | Title / dir source | Notes |
|--------|-----------|-------------------|-------|
| **OpenAPI** | One group per `tags[]` entry; **untagged** bucket for operations without tags | `x-tagGroups` defines section order and nested tag membership when present | Operations inherit tags from operation or path item. Component schemas land in the **components** group when not tied to a tag context; schemas referenced only from untagged operations use **untagged**. |
| **AsyncAPI** | Primary group: application `id` or slug of `info.title`; tag-based subgroups where spread-it-out expects | Preserve 2.x vs 3.x structure — no normalization | **MVP categories:** `channel`, `operation`, `message`, `schema`. **`parameter` and `security-scheme`:** standalone entities only when populated from `components` maps; otherwise inline in operation/channel fence bodies. |
| **OpenRPC** | One group per `x-tagGroup` name; default group when absent | Module id = service (`info.title` or document stem) | Methods → `operation`; content descriptors and `components.schemas` → `schema` / `parameter`. |
| **JSON Schema catalog** | One group per entry file (ADR 0006) | File stem or `$id` segment | No change. |

### Envelope / IR split

**Stays in switchback-jsonschema:** `Info`, `Server`, `Components` (as
`BTreeMap<String, Value>`), `Tag`, `ExternalDocs`, `Envelope::from_value`.

**Family-local (initially in each parser crate):** OpenAPI
paths/PathItem/Operation/Parameter/Response; AsyncAPI
channels/operations/messages/bindings; OpenRPC methods mapper.

**Escape hatch:** If the shared surface grows unwieldy, split to a
`switchback-api` crate later (per ADR 0006); do not block OpenAPI on that split.

### AsyncAPI meta-schema validation corpus

For `ContractFamily::meta_schema()`, use curated document schemas:
`schemas/3.0.0-without-$id.json` (3.x line) and `schemas/2.6.0-without-$id.json`
(2.x line) from the vendored corpus (ADR 0005).

## Consequences

- OpenAPI populate must implement tag/x-tagGroup grouping and a pinned rule for
  component-only schemas.
- AsyncAPI populate preserves 2.x vs 3.x document shape;
  parameter/security-scheme standalone entities are optional.
- OpenRPC populate maps x-tagGroup to groups and methods to operations.
- Shared envelope in switchback-jsonschema stays small; family crates own
  paths/channels/methods IR until a future switchback-api split.
