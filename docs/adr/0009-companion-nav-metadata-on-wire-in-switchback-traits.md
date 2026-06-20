# 9. Companion nav metadata on wire in switchback-traits

Date: 2026-06-20

## Status

Proposed

Relates to
[7. Parser library conventions for switchback family crates](0007-parser-library-conventions-for-switchback-family-crates.md)

## Context

Companions are contract-level markdown files discovered beside contract inputs
and copied verbatim into renderer output. mdBook SUMMARY navigation needs
directory-aware nesting and human titles (from markdown H1 headings). Today the
wire `Companion` message stores only `output_name` and `bytes`; renderers must
re-derive placement keys and titles. Both `switchback-protobuf` and
`switchback-jsonschema` duplicate ancestor-walk companion discovery. ADR 0007
deferred companion centralization in traits.

## Decision

1. Add `title`, `source_dir`, and `stem` to parser-local `CompanionFile`, wire
   `Companion`, and `switchback.proto` `Companion`. `source_path` remains
   parser-local provenance only.
2. Centralize in `switchback-traits`: `title_from_markdown`,
   `module_path_from_output`, and shared `discover_ancestors_companions` for
   `CompanionDiscovery::Ancestors`. Family parser crates compute anchor
   directories from their inputs and delegate; they do not duplicate the walk.
3. Add default `CompanionStrategy::companion_title()` on the trait; families may
   override for non-markdown companions.
4. Nav tree assembly (mdBook SUMMARY nesting, module-path prefix display rules)
   stays in `switchback-mdbook`; the wire artifact stores placement keys only,
   not a pre-built tree.
5. Companions remain a contract-level list (not nested under `Group`) so
   ancestor READMEs above package groups are representable.
6. Renderers consume wire nav fields when present; legacy artifacts with empty
   fields fall back to derive-from-`output_name`+bytes.

## Consequences

- `switchback-codec-pb` maps the new proto fields; existing round-trip tests may
  need fixture updates.
- `switchback-protobuf` and `switchback-jsonschema` thin wrappers over shared
  discovery; duplicated `collect_md_in_dir` removed from family crates.
- Substantial fulfillment of ADR 0007 companion centralization (nav fields +
  ancestor walk + title helper).
- Shared helpers for `CompanionDiscovery::Beside` and `DocsSubdir` remain
  deferred until a family needs them.
- mdBook renderer gains hierarchical SUMMARY parity with protobuf-mdbook init
  output.
