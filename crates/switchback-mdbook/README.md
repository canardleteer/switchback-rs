# switchback-mdbook

mdBook renderer for the switchback-rs toolchain.

`switchback-mdbook` turns a deserialized
[`ReferenceManual`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-traits/src/model/manual.rs)
into an mdBook project tree (markdown pages, `SUMMARY.md`, optional init
scaffolding). It implements
[`Renderer`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-traits/src/traits/renderer.rs)
and
[`SyncRenderer`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-traits/src/traits/renderer.rs)
as `MdBookRenderer`.

Runtime dependencies are
[`switchback-traits`](https://github.com/canardleteer/switchback-rs/tree/main/crates/switchback-traits)
and
[`switchback-codec-pb`](https://github.com/canardleteer/switchback-rs/tree/main/crates/switchback-codec-pb)
(for reading `switchback.binpb`). Parsers such as `switchback-protobuf` are
**dev-dependencies only** (integration tests and golden fixtures).

## Usage

Library API (typical in-process path):

```rust
use switchback_codec_pb::ProtobufCodec;
use switchback_mdbook::MdBookRenderer;
use switchback_traits::{Options, SyncRenderer, SyncSwitchbackCodec};

let manual = ProtobufCodec.deserialize(&std::fs::read("switchback.binpb")?)?;
let opts = Options::default();
MdBookRenderer.render(&manual, &opts, "book/")?;
```

CLI entry point (`switchback-mdbook`) is stubbed; use the library renderer or
workspace `cargo xtask render --renderer mdbook` while CLI parity is deferred.

## Features

- Package / entity / split output modes via `Options`
- Companion markdown and nav metadata from the wire model
  ([ADR 0009](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0009-companion-nav-metadata-on-wire-in-switchback-traits.md))
- Protobuf and CEL syntax highlighting preprocessor
  (`mdbook-protobuf-highlight`)
- Relative link validation helpers (`link_check`)

## Architecture

See
[ADR 0009](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0009-companion-nav-metadata-on-wire-in-switchback-traits.md)
for companion navigation on the wire, and the workspace
[Glossary](https://github.com/canardleteer/switchback-rs/blob/main/docs/GLOSSARY.md)
for renderer terminology.
