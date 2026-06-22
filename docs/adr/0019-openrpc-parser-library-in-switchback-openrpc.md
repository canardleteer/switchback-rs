# 19. OpenRPC parser library in switchback-openrpc

Date: 2026-06-21

## Status

Proposed

## Context

ADR 0008 pins OpenRPC grouping at a high level; implementation details for
entity naming, multi-entry Acme scoping, content-descriptor mapping, and
intra-link deferral were left to the first behavior parser. `switchback-openrpc`
must land a library-first `load()` pipeline reusing `switchback-jsonschema` and
pair with `switchback-mdbook` for render regression without CLI or `xtask parse`
gates.

## Decision

Implement `switchback-openrpc` as a library-first parser per ADR 0007:

- **Versions:** OpenRPC 1.3.x and 1.4.x only. Read `openrpc` from the document
  root; reject missing or unsupported versions. Preserve authored JSON Schema
  shape; no normalization or upgrade.
- **Extensions:** `.json`, `.yaml`, `.yml`.
- **Entity naming:** Operations use method `name` (e.g. `echoUnary`). Component
  entities use map keys under `components.schemas`. Reusable content descriptors
  use `components.contentDescriptors` keys as **parameter** entities.
- **Operation body:** Reuse `OperationBody`: `signature` =
  `**methodName** ( … ) -> ResultType`; `parameters` from method `params[]`;
  single **result** mapped to `responses` with `ResponseSeverity::Unspecified`.
  `fence_body` = serialized method object.
- **Grouping (single-entry):** One `Group` per `x-tagGroup` name; **`default`**
  for untagged methods; **`components`** for standalone schema/parameter
  entities from `components.*`. `x-tagGroups` orders groups only.
- **Grouping (multi-entry):** One group per entry (`info.x-switchback-group` or
  `vN/openrpc.json` → `acme.example.vN`); all entities from an entry land in
  that group (ADR 0013 pattern).
- **Structural refs:** `#/components/schemas/*` → category `schema`;
  `#/components/contentDescriptors/*` → category `parameter`. Prose
  **intra-links** deferred; `OpenRpcLinkExtractor` returns empty `intra_links`.
- **Companions:** `CompanionDiscovery::Beside` — `{stem}.md` beside each entry
  file.
- **Fixtures:** Upstream
  [open-rpc/examples](https://github.com/open-rpc/examples) via
  `example-fixtures.lock.toml` (metrics 1.3, petstore/link with fetch-time
  `openrpc` bump to 1.4.0 until upstream merges); hand-maintained **micro**
  fixtures and Acme three-version corpus.

## Consequences

- OpenRPC populate implements tag-group/default/components grouping and
  multi-entry Acme scoping.
- mdBook renderer gains family-aware OpenRPC pages; assemble + reference-manual
  gain JSON-RPC slice.
- `switchback-assemble` takes a runtime dependency on `switchback-openrpc`;
  `switchback-mdbook` uses `switchback-openrpc` as a **dev-dependency** only
  (golden/link-check tests). No forward workspace dev-deps on
  `switchback-openrpc`.
- CLI, `xtask parse`, `--validate`, and prose intra-links remain follow-up work.
