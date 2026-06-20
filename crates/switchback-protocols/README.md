# switchback-protocols

Built-in [`http`](https://www.rfc-editor.org/rfc/rfc9110.html) and
[`grpc`](https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-GRPC.md) protocol
implementations plus `ProtocolRegistry` for encoding and decoding
[`ProtocolAttachment`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-traits/src/model/protocol.rs)
payloads on the switchback wire.

## Role

Contract-family parsers attach transport facts during populate; this crate owns
the built-in protocol trait implementations and the registry that renderers and
tools use to decode those attachments without family-specific knowledge.

- `HttpProtocol` — RFC 9110 / Problem Details metadata; OpenAPI populate uses
  this for method, path, status, parameters, and
  [HTTP streaming flags](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0012-http-streaming-inference-and-grpc-metadata-from-protobuf-options.md)
- `GrpcProtocol` — RPC name, streaming shape, status/error codes,
  [gRPC call metadata](https://github.com/canardleteer/switchback-rs/blob/main/docs/GLOSSARY.md#grpc-call-metadata)
  keys
- `ProtocolRegistry` — decode `ProtocolAttachment` envelopes; opaque passthrough
  for custom protocol ids

## Decode example

```rust
use switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::HttpOperationMeta;
use switchback_protocols::{DecodedAttachment, HttpPayloadKind, ProtocolRegistry};
use switchback_traits::ProtocolAttachment;

let registry = ProtocolRegistry::with_builtins();
let meta = HttpOperationMeta {
    method: "GET".into(),
    path_template: "/pets".into(),
    ..Default::default()
};
let attachment = registry.http().attach_operation(&meta);

match registry.decode_attachment(&attachment)? {
    DecodedAttachment::Http(HttpPayloadKind::Operation(decoded)) => {
        assert_eq!(decoded.method, "GET");
    }
    other => panic!("unexpected decode: {other:?}"),
}
# Ok::<(), switchback_traits::SwitchbackError>(())
```

## Entity attachment matrix

| IR node | Typical `http` payload | Typical `grpc` payload |
| --- | --- | --- |
| Contract | `HttpContractMeta` | `GrpcContractMeta` |
| Operation | `HttpOperationMeta` | `GrpcOperationMeta` |
| Response ref/body | `HttpResponseMeta` / `HttpErrorMeta` | `GrpcStatusMeta` / `GrpcErrorMeta` |
| Parameter ref/body | `HttpParameterMeta` | `GrpcMetadataMeta` |

Full matrix and decode steps:
[ADR 0011](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0011-protocol-layer-and-contract-family-binding.md).

## Custom protocols

Register a custom slug in downstream code without editing this crate: supply
your own proto package (same envelope pattern as `HttpPayload` / `GrpcPayload`)
and handle `DecodedAttachment::Opaque` for unknown ids until you add a decoder.

## Further reading

- [ADR 0011](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0011-protocol-layer-and-contract-family-binding.md)
  — protocol layer architecture
- [ADR 0012](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0012-http-streaming-inference-and-grpc-metadata-from-protobuf-options.md)
  — streaming inference and protobuf metadata extension
- [Glossary — protocol](https://github.com/canardleteer/switchback-rs/blob/main/docs/GLOSSARY.md#protocol)
- `planning/PROGRESS.md` (local; gitignored) — protocol layer status
  (**Implemented**); AsyncAPI multi-binding remains follow-on
