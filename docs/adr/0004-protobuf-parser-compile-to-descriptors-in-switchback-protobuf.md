# 4. Protobuf parser compile-to-descriptors in switchback-protobuf

Date: 2026-06-19

## Status

Proposed

## Context

spread-it-out Phase 5 decomposes protobuf-mdbook into parser, codec, and
renderer crates. switchback-traits and switchback-codec-pb are in place (ADR
0003). We need a protobuf ContractFamily parser that emits ReferenceManual
without lifting the mdBook renderer yet.

## Decision

Implement switchback-protobuf as a library-only parser that:

1. Compiles inputs to FileDescriptorSet via protoc or buf build (Cargo features
   `protoc` and `buf`, both default; vendored fallbacks via protoc-bin-vendored
   and buf-tools 1.70.0-hotfix.1).
2. Uses buffa-descriptor generated types (same as protobuf-mdbook), not prost.
3. Maps protobuf packages to switchback groups; messages/enums to schema
   entities; services and RPCs to service/operation entities with fence
   synthesis lifted from protobuf-mdbook render/proto_syntax.
4. Builds ReferenceManual with a lossless source layer (every file_to_generate
   as Document with URI relative to the Buf module root, SHA-256 content_hash)
   plus companions via ancestor discovery.
5. Exposes `load()` returning ReferenceManual and ProtobufContract for
   downstream CLI/renderer work.
6. Defers full LinkExtractor (ProtobufLinkExtractor returns empty intra_links);
   structural refs on operations populate StoredEntity.refs.
7. Vendors protobuf-mdbook examples/proto fixtures and mirrored protoc/buf +
   ProtobufCodec round-trip tests in-crate.

## Consequences

Positive: protobuf parser is testable in isolation; protoc/buf parity and wire
round-trip are gated in CI; directory-faithful source URIs enable Buf module
restoration after codec round-trip.

Negative: intra-link extraction and FQN prose linking remain incomplete; CLI and
switchback-mdbook renderer are still deferred; lifted internal modules lack full
rustdoc until a follow-up pass.

Follow-up: complete ProtobufLinkExtractor; wire CLI and xtask parse --parser
protobuf; lift mdBook renderer to switchback-mdbook.
