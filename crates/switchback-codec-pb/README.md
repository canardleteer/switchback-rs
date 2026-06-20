# switchback-codec-pb

Reference binary codec for the switchback artifact in the switchback-rs
toolchain.

Every parser emits a
[`ReferenceManual`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-traits/src/model/manual.rs)
through a
[`SwitchbackCodec`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-traits/src/traits/codec.rs)
implementation; this crate provides the protobuf wire encoding using
[buffa](https://github.com/anthropics/buffa) types generated from
[`switchback.proto`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-codec-pb/proto/canardleteer/switchback/v1alpha1/switchback.proto)
(`canardleteer.switchback.v1alpha1`; repo-root
[`proto/`](https://github.com/canardleteer/switchback-rs/tree/main/proto)
symlinks here).

The default on-disk filename is `switchback.binpb`
([`DEFAULT_SWITCHBACK_FILENAME`](https://docs.rs/switchback-codec-pb/latest/switchback_codec_pb/constant.DEFAULT_SWITCHBACK_FILENAME.html)).

## Usage

```rust
use switchback_codec_pb::ProtobufCodec;
use switchback_traits::{ReferenceManual, SyncSwitchbackCodec};

let manual = ReferenceManual {
    switchback_version: "v1alpha1".into(),
    title: "My API".into(),
    ..Default::default()
};

let codec = ProtobufCodec;
let bytes = codec.serialize(&manual)?;
let restored = codec.deserialize(&bytes)?;
```

For async callers, use the same methods on
[`SwitchbackCodec`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-traits/src/traits/codec.rs)
with `.await`.

## Schema status

`canardleteer.switchback.v1alpha1` is **unstable** and in active development.
Expect wire-format changes until the schema graduates to `v1`. See
[ADR 0003](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0003-protobuf-switchback-wire-format-with-buffa-in-switchback-codec-pb.md).

## Protocol attachments

Transport semantics live outside the core contract graph in a
[`ProtocolAttachment`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-traits/src/model/protocol.rs)
envelope (`protocol_id` + opaque `payload` bytes) on contract and entity nodes.
This crate compiles **four** protobuf schemas from `proto/`:

| Schema | Package | Role |
| --- | --- | --- |
| [`switchback.proto`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-codec-pb/proto/canardleteer/switchback/v1alpha1/switchback.proto) | `canardleteer.switchback.v1alpha1` | Core IR; `ProtocolAttachment` and `protocols[]` fields |
| [`http.proto`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-codec-pb/proto/canardleteer/switchback/protocol/http/v1alpha1/http.proto) | `canardleteer.switchback.protocol.http.v1alpha1` | `HttpPayload` oneof (method, path, status, parameters) |
| [`grpc.proto`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-codec-pb/proto/canardleteer/switchback/protocol/grpc/v1alpha1/grpc.proto) | `canardleteer.switchback.protocol.grpc.v1alpha1` | `GrpcPayload` oneof (RPC, status, metadata) |
| [`metadata_options.proto`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-codec-pb/proto/canardleteer/switchback/protocol/grpc/v1alpha1/metadata_options.proto) | same gRPC package | Author-facing `switchback_rpc_metadata` extension on RPCs |

Decode `payload` by `protocol_id`: built-in ids `"http"` and `"grpc"` map to
`HttpPayload` and `GrpcPayload`. See the entity attachment matrix in
[ADR 0011](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0011-protocol-layer-and-contract-family-binding.md).
HTTP streaming inference and gRPC call metadata authoring are in
[ADR 0012](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0012-http-streaming-inference-and-grpc-metadata-from-protobuf-options.md).

Generated Rust types:

- `canardleteer::switchback::v1alpha1::*` — core wire messages
- `canardleteer::switchback::protocol::http::v1alpha1::*`
- `canardleteer::switchback::protocol::grpc::v1alpha1::*`

Use
[`switchback-protocols`](https://github.com/canardleteer/switchback-rs/tree/main/crates/switchback-protocols)
`ProtocolRegistry` for encode/decode in application code.

## Build

Protoc is vendored via `protoc-bin-vendored` in `build.rs`; no system `protoc`
install is required. The wire schema lives in this crate's `proto/` tree and is
checked with `buf lint` / `buf format` in CI.
