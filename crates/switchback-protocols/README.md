# switchback-protocols

Built-in [`http`](https://www.rfc-editor.org/rfc/rfc9110.html) and
[`grpc`](https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-GRPC.md) protocol
implementations plus `ProtocolRegistry` for encoding and decoding
[`ProtocolAttachment`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-traits/src/model/protocol.rs)
payloads on the switchback wire.

See
[ADR 0011](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0011-protocol-layer-and-contract-family-binding.md).
