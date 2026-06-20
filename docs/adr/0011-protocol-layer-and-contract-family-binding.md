# 11. Protocol layer and contract family binding

Date: 2026-06-20

## Status

Proposed

Relates to
[2. Async-first traits with synchronous secondary APIs in switchback-traits](0002-async-first-traits-with-synchronous-secondary-apis-in-switchback-traits.md)

Relates to
[3. Protobuf switchback wire format with buffa in switchback-codec-pb](0003-protobuf-switchback-wire-format-with-buffa-in-switchback-codec-pb.md)

Relates to
[7. Parser library conventions for switchback family crates](0007-parser-library-conventions-for-switchback-family-crates.md)

Relates to
[8. Per-family grouping rules and jsonschema shared-layer boundary](0008-per-family-grouping-rules-and-jsonschema-shared-layer-boundary.md)

## Context

Transport semantics (HTTP methods, gRPC status, metadata) previously leaked into
family crates and generic renderers. Contract family describes spec grammar;
protocol describes invocation/transport semantics. AsyncAPI multi-binding
motivates `repeated ProtocolAttachment` on the wire.

The entity attachment matrix below lists which `HttpPayload` / `GrpcPayload`
arm typically appears on each IR node.

## Decision

Introduce a protocol dimension orthogonal to contract family:

- **`ProtocolAttachment { protocol_id, payload }`** on contract and entity
  nodes.
- Separate protobuf packages: `canardleteer.switchback.protocol.http.v1alpha1`,
  `canardleteer.switchback.protocol.grpc.v1alpha1`.
- **`HttpPayload` / `GrpcPayload` oneof** inside each protocol package.
- Built-in IDs: `"http"`, `"grpc"`.
- **`switchback-protocols`** crate with `ProtocolRegistry`.
- **`ContractFamily::supported_protocols` / `default_protocol`**.
- **`ResponseRef.severity` / `ResponseBody.severity` set only via protocol
  traits at populate time.**

Spec references: [RFC 9110](https://www.rfc-editor.org/rfc/rfc9110.html),
[RFC 9457](https://www.rfc-editor.org/rfc/rfc9457.html),
[OAS 3.1 Parameter/Response](https://spec.openapis.org/oas/v3.1.0);
[gRPC PROTOCOL-GRPC](https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-GRPC.md),
[statuscodes.md](https://github.com/grpc/grpc/blob/master/doc/statuscodes.md),
[google.rpc.Status](https://github.com/googleapis/googleapis/blob/master/google/rpc/status.proto).

## Consequences

- Additive v1alpha1 wire fields; `switchback-protocols` temporarily depends on
  `switchback-codec-pb` for generated payload types (future
  `switchback-protocol-proto` extraction).
- Golden output may grow when protocol rendering adds detail; `link_check` must
  stay green.
- AsyncAPI populate reuses the envelope in a follow-on.
- Custom protocols register via `ProtocolRegistry` without editing built-in
  crates.

## Known follow-ups

The entity attachment matrix reserved wire fields for
**HTTP streaming** (`HttpOperationMeta.request_streaming` /
`response_streaming`) and **gRPC call metadata** (`GrpcMetadataMeta` on
`ParameterRef`). Initial populate left those rows incomplete. Populate rules are
pinned in
[12. HTTP streaming inference and gRPC metadata from protobuf options](0012-http-streaming-inference-and-grpc-metadata-from-protobuf-options.md).
