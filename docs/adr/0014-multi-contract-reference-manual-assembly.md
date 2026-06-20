# 14. Multi-contract reference manual assembly

Date: 2026-06-20

## Status

Proposed

Relates to
[11. Protocol layer and contract family binding](0011-protocol-layer-and-contract-family-binding.md)

## Context

The glossary describes modules that may contain multiple contracts (OpenAPI HTTP

+ protobuf gRPC). ADR 0011 binds one contract family per ManualContract. We need
a documented assembly path for examples and tooling that merges family-specific
ReferenceManual fragments into one module without group-id collisions in
ResolvedManual or mdBook SUMMARY indexes.

## Decision

Introduce switchback-assemble with an explicit GroupPrefixPolicy. MVP
module.yaml schema (parsed by examples only): id, title, overview, contracts[]
with family and inputs[]. Assembly loads each family via existing LoadArgs,
normalizes module id/title/overview on a single Module, and merges contracts
into one ReferenceManual. Default group prefix policy ContractFamily prefixes
every group id as `{family}.{original_group_id}` (e.g. openapi.acme.example.v1,
protobuf.acme.example.v1) so ResolvedManual by_ref keys and mdBook package
indexes stay unique. Document sources are concatenated with existing URI
disambiguation per family loader. Cross-family IntraLink resolution and
module.yaml parsing inside family parsers remain deferred.

## Consequences

examples/reference-manual can assemble Acme v1 HTTP+gRPC; mdBook gains
mixed-family SUMMARY sections. Group ids in assembled manuals differ from
single-family loads — examples must document the prefix. Future work: optional
unprefixed mode when renderer keys include contract family; full module.yaml
parser in xtask/CI.
