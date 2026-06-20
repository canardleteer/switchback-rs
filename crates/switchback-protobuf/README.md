# switchback-protobuf

Protobuf parser for the switchback-rs toolchain.

`switchback-protobuf` turns `.proto` inputs into a
[`ReferenceManual`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-traits/src/model/manual.rs)
using a **compile-to-descriptors** pipeline (`protoc` or `buf build`), then
populates switchback groups, entities, companions, and the source layer. The
design is decomposed from
[`protobuf-mdbook`](https://github.com/canardleteer/protobuf-mdbook); this crate
is library-only (no mdBook renderer or working CLI in this pass).

## Usage

```rust
use std::path::PathBuf;
use switchback_protobuf::{
    default_proto_deps_export, ensure_test_proto_deps, examples::{fixtures_proto_dir, EXAMPLE_PROTO_INPUTS},
    input::Compiler, load, LoadArgs,
};

let module_root = fixtures_proto_dir();
let export = default_proto_deps_export();
let _ = ensure_test_proto_deps(&module_root, None);

let args = LoadArgs {
    compiler: Compiler::Protoc,
    module_root: module_root.clone(),
    inputs: EXAMPLE_PROTO_INPUTS.iter().map(|p| PathBuf::from(*p)).collect(),
    proto_paths: vec![module_root.clone(), export.clone()],
    protoc_path: None,
    buf_path: None,
    proto_deps_export: Some(export),
    title: None,
};

let manual = load(&args)?;
```

Serialize with
[`switchback-codec-pb`](https://github.com/canardleteer/switchback-rs/tree/main/crates/switchback-codec-pb)
(`ProtobufCodec`, default filename `switchback.binpb`).

## Compilers and features

Default features enable both compilers:

| Feature | Dependency | Fallback |
| --- | --- | --- |
| `protoc` (default) | `protoc-bin-vendored` | `PATH` `protoc` |
| `buf` (default) | `buf-tools` 1.70.0-hotfix.1 | `PATH` `buf` |

Disable defaults for slim builds, e.g.
`cargo build -p switchback-protobuf --no-default-features --features protoc`.

BSR deps for the examples module (Protovalidate) are exported via
[`ensure_test_proto_deps`](src/load.rs) / [`proto_deps`](src/proto_deps.rs),
mirroring protobuf-mdbook CI.

## Link extraction

[`ProtobufFqnLinkExtractor`](src/link.rs) (`ProtobufLinkExtractor`) extracts
bare fully-qualified type names from leading-comment prose into `intra_links`.
Structural cross-refs on operations (`StoredEntity.refs`) are populated.
Field-level type links inside protobuf fences are not generated (matches
protobuf-mdbook; links appear in RPC signature prose and doc prose only).

See
[ADR 0004](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0004-protobuf-parser-compile-to-descriptors-in-switchback-protobuf.md).

## Fixtures

Integration tests copy protobuf-mdbook’s `examples/proto/` tree to
[`tests/fixtures/proto/`](tests/fixtures/proto/) (including `buf.yaml`,
`buf.lock`, companion markdown). Loose protos live under
[`tests/fixtures/loose/`](tests/fixtures/loose/). Tests assert protoc/buf
parity, `ProtobufCodec` round-trip, directory-faithful source restoration, and
`buf lint` / `buf format --diff` on restored modules.

## Architecture

See
[ADR 0004](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0004-protobuf-parser-compile-to-descriptors-in-switchback-protobuf.md).
