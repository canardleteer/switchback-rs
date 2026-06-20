# 12. HTTP streaming inference and gRPC metadata from protobuf options

Date: 2026-06-20

## Status

Proposed

Relates to
[11. Protocol layer and contract family binding](0011-protocol-layer-and-contract-family-binding.md)

## Context

ADR 0011 introduced `HttpOperationMeta.request_streaming` /
`response_streaming` and `GrpcMetadataMeta` on the wire. OpenAPI populate
always left streaming flags `false`. Protobuf populate never attached gRPC call
metadata on `ParameterRef` because standard `MethodDescriptorProto` carries no
metadata-key list. Message fields named `metadata` are payload data, not gRPC
transport metadata.

## Decision

### HTTP streaming inference (OpenAPI → `http`)

OpenAPI has no native streaming flags. Infer from documented request/response
**content** media types per
[OAS 3.1 Media Type Object](https://spec.openapis.org/oas/v3.1.0#media-type-object)
and [RFC 9110](https://www.rfc-editor.org/rfc/rfc9110.html) body semantics.

| Flag | Set `true` when |
| --- | --- |
| `response_streaming` | Any operation response (including `default`) has a `content` entry whose media type is in the streaming **response** set |
| `request_streaming` | `requestBody.content` includes a media type in the streaming **request** set |

**Streaming response media types:** `text/event-stream`,
`application/stream+json`, `application/x-ndjson`

**Streaming request media types:** `application/octet-stream` (present among
`requestBody.content` keys)

When both flags are false, populate behavior is unchanged. OpenAPI cannot
express all RFC 9110 streaming cases; vendor `x-*` extensions are out of scope
for this ADR.

### gRPC call metadata (protobuf → `grpc`)

**Populate source:** Switchback protobuf extension on
`google.protobuf.MethodOptions` in
`canardleteer.switchback.protocol.grpc.v1alpha1/metadata_options.proto`:

```protobuf
message RpcMetadataKey {
  string name = 1;
  bool required = 2;
}
message RpcMetadataKeys {
  repeated RpcMetadataKey keys = 1;
}
extend google.protobuf.MethodOptions {
  RpcMetadataKeys switchback_rpc_metadata = 50100;
}
```

For each key in `(switchback_rpc_metadata)`, populate one `ParameterRef` on the
RPC `OperationBody` with `location: "metadata"`, `protocols` carrying
`GrpcPayload.metadata` (`GrpcMetadataMeta`), and severity unchanged (metadata is
not a response outcome).

**Not implemented:** inferring metadata keys from `UninterpretedOption` on
method options. See `planning/PROGRESS.md` (local; gitignored) deferred
alternatives.

## Consequences

- `switchback-openapi` gains a micro streaming fixture and pass-through tests
  for streaming flags.
- `switchback-codec-pb` compiles `metadata_options.proto`; protobuf test
  fixtures import it and annotate selected RPCs.
- `switchback-protobuf` populate reads the extension from compiled descriptors.
- mdBook may show streaming labels and metadata keys when attachments are
  present; golden diffs acceptable when output is richer.
- Closes the two gaps listed in `planning/PROGRESS.md` when populate and
  tests land.
