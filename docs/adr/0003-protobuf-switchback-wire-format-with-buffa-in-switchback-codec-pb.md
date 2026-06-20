# 3. Protobuf switchback wire format with buffa in switchback-codec-pb

Date: 2026-06-19

## Status

Proposed

Relates to
[11. Protocol layer and contract family binding](0011-protocol-layer-and-contract-family-binding.md)

## Context

switchback-rs serializes reference manuals to a binary switchback artifact
between parsers and renderers. The seam crate defines SwitchbackCodec (ADR
0002); switchback-codec-pb implements it. We need a pinned protobuf stack,
wire-format rules that differ from the in-memory model, and an explicit unstable
v1alpha1 schema status.

## Decision

**Protobuf stack:** Use buffa for codegen (buffa-build from the wire schema in
`crates/switchback-codec-pb/proto/canardleteer/switchback/v1alpha1/switchback.proto`;
repo-root `proto/` symlinks to that directory) and wire encode/decode
(buffa::Message). Same stack as protobuf-mdbook.

**Schema package:** `canardleteer.switchback.v1alpha1` is unstable and in active
development alongside parsers and the codec. Expect field and message changes;
no BSR publish or external stability promise yet. The module is linted with Buf
`STANDARD` (local `buf.yaml`, no registry name).

**Default filename:** switchback.binpb (not manual.binpb).

**Wire exclusions from the in-memory model:**

- Group.source_path is parser-local (markdown path line provenance) and is not
  serialized.
- LinkTarget::Unresolved is in-memory/parser-only; serialize rejects manuals
  containing unresolved intra-links.

**Version check:** Deserialize rejects switchback_version values that do not
start with v1alpha1 (minimal check while schema moves).

**Errors:** Use SwitchbackError::codec with message strings; do not design a
structured codec error taxonomy upfront.

## Consequences

Positive: One protobuf stack aligned with protobuf-mdbook; clear wire vs
in-memory boundaries; codec crate can ship before schema graduation to v1.

Negative: v1alpha1 wire bytes may change as the schema evolves; downstream tools
must tolerate schema churn until v1.

Neutral: Golden byte pinning deferred until update-golden lands.
