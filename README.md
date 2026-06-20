# switchback-rs

A three-layer toolchain for turning API **contracts** into rendered **reference
manuals**. A contract is one family's description of a service: a set of
`.proto` files, an `openapi.yaml` (and the files it `$ref`s), an
`asyncapi.yaml`, an `openrpc.json`, or a JSON Schema catalog.

The toolchain has three sides, separated by a versioned, lossless intermediate
representation called the **switchback** (a switchback binary file):

1. **Parsers** turn a contract into the switchback.
2. **`switchback-traits`** is the core: the trait spine and in-memory model
   every parser and renderer depends on, plus the reference binary codec.
3. **Renderers** turn the switchback into a target format (for example, mdBook).

The switchback is the canonical artifact; every rendered manual is one view of
it. The name is the metaphor: a switchback is the hairpin turn on a mountain
trail where you reverse direction to gain altitude. You climb up from the
source contracts (parsing), reach the switchback, and from there turn back
down to the verbatim source documents — carried losslessly — or continue up
into any rendered form.

Every parser always emits a switchback binary file. A parser may also call a
renderer in-process for the common case
(`switchback-openapi --render mdbook openapi.yaml`); a renderer may also run
standalone on a switchback binary file (`switchback-mdbook switchback.binpb`).
No parser depends on any renderer and no renderer depends on any parser; the
switchback is the only thing both sides see.

See the [Glossary](docs/GLOSSARY.md) for terminology (reference manual,
contract, module, entity, and how each contract family maps).

## Workspace layout

```text
switchback-rs/
├── Cargo.toml                    workspace + [workspace.dependencies]
├── rust-toolchain.toml
├── proto/
│   └── switchback.proto          lossless switchback schema (switchback.v1alpha1)
├── crates/
│   ├── switchback-traits/        core: traits + in-memory model + helpers
│   ├── switchback-codec-pb/      reference binary codec (protobuf)
│   ├── switchback-mdbook/        renderer: switchback -> mdBook
│   ├── switchback-protobuf/      parser: .proto -> switchback
│   ├── switchback-jsonschema/    parser + shared JSON-Schema layer
│   ├── switchback-openapi/       parser: OpenAPI -> switchback
│   ├── switchback-asyncapi/      parser: AsyncAPI -> switchback
│   └── switchback-openrpc/       parser: OpenRPC -> switchback
└── xtask/                        workspace task runner (ci, parse, render, ...)
```

## Crates

### Core (the seam)

- **`switchback-traits`** — the seam. Owns the `ContractFamily`, `Contract`,
  `Renderer`, `LinkExtractor`, `LinkFormatter`, and `SwitchbackCodec` traits;
  the in-memory `ReferenceManual` / `Module` / `Contract` / `Group` /
  `Entity` model; and the format-agnostic helpers (slug, link context, link
  check, companion discovery, paths, prose escaping, source snippets). Knows
  nothing about any contract family, output format, or serialization format.
- **`switchback-codec-pb`** — the reference binary codec. Implements
  `SwitchbackCodec` using `buffa`-generated types compiled from
  `proto/switchback.proto` (`switchback.v1alpha1`). A switchback binary file is
  the canonical, deterministic, cacheable serialized form. The codec is a binary
  IDL format.

### Renderers

- **`switchback-mdbook`** — the mdBook renderer. Turns a switchback into an
  mdBook via `MdBookRenderer`. Owns the mdBook-specific scaffolding lifted from
  `protobuf-mdbook` (inspired by
  [canardleteer/protobuf-mdbook](https://github.com/canardleteer/protobuf-mdbook)):
  `book.toml` inference, `init` scaffolding, SUMMARY generation, and the mdBook
  render driver. mdBook is one renderer impl, not the center of the pipeline.

### Parsers

- **`switchback-protobuf`** — parses `.proto` files into a switchback via the
  compile-to-descriptors strategy (`buf build` / `protoc` / prebuilt
  `--descriptor-set`). The protobuf-specific periphery of `protobuf-mdbook`
  (plugin protocol, input pipeline, `SourceCodeInfo` span extraction, fence
  synthesis, CEL/Protovalidate extraction) rewritten as the parser side of the
  seam. Depends only on `switchback-traits`.
- **`switchback-jsonschema`** — the shared JSON-Schema-document parser layer:
  a document `Loader` and `$ref` resolver, the shared API-description
  envelope (`info`/`servers`/`components`/`security`/`tags`/`externalDocs`),
  and a schema entity-body producer. Also a standalone parser that turns a
  JSON Schema catalog into a switchback. The OpenAPI, AsyncAPI, and OpenRPC
  parsers build on it; renderers never see it.
- **`switchback-openapi`** — parses an OpenAPI Description (3.0.x and 3.1.x)
  into a switchback, preserving the contract's version. Categories: `schema`,
  `operation`, `parameter`, `response`, `request-body`, `security-scheme`.
  Grouping via `tags` / `x-tagGroups`.
- **`switchback-asyncapi`** — parses an AsyncAPI document (2.x and 3.x) into a
  switchback, preserving the contract's version (2.x channel-embedded
  operations stay embedded; 3.x top-level operations stay top-level).
  Categories: `channel`, `operation`, `message`, `schema`. Generates Mermaid
  sequence diagrams from operations and replies.
- **`switchback-openrpc`** — parses an OpenRPC document into a switchback. A
  thin behavior layer over JSON Schema content descriptors: `methods` ->
  `Operation` entities, content descriptors / `components.schemas` -> `Schema`
  entities. Categories: `operation`, `schema`, `parameter`. Reuses
  `switchback-jsonschema` and `switchback-traits` with a method-to-operation
  mapper.

### Tooling

- **`xtask`** — the workspace task runner: `ci`, `fmt`, `fmt-check`, `clippy`,
  `test`, `parse` (`--parser <family>`), `render` (`--renderer <target>`),
  `link-check`, `update-golden`, `check-toolchain`.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
