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
(`canardleteer.switchback.v1alpha1`; repo-root [`proto/`](https://github.com/canardleteer/switchback-rs/tree/main/proto)
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

## Build

Protoc is vendored via `protoc-bin-vendored` in `build.rs`; no system `protoc`
install is required. The wire schema lives in this crate's `proto/` tree and is
checked with `buf lint` / `buf format` in CI.
