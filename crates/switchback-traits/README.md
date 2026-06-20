# switchback-traits

The seam of the `switchback-rs` toolchain.

`switchback-traits` owns the trait spine and the in-memory model that every
parser and every renderer depends on, with no dependency on any contract
family, output format, or serialization implementation:

- `ContractFamily` and `Contract` — parser-side identity and loaded contract
  views; `ContractFamily::supported_protocols` and
  `ContractFamily::default_protocol` declare which
  [protocol](https://github.com/canardleteer/switchback-rs/blob/main/docs/GLOSSARY.md#protocol)
  slugs a family may attach during populate
- `ProtocolAttachment` — transport envelope (`protocol_id` + payload bytes) on
  contract and entity nodes; see
  [ADR 0011](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0011-protocol-layer-and-contract-family-binding.md)
- `Renderer` / `SyncRenderer`, `SwitchbackCodec` / `SyncSwitchbackCodec`,
  `LinkExtractor`, `LinkFormatter` — renderer-side and serialization seams
- `ReferenceManual` and related model types — the lossless in-memory switchback
  graph
- `Options`, `LinkContext` — shared option and link-index data shapes (logic
  deferred)

**Protocol vs contract family:** contract family is spec grammar (OpenAPI,
Protobuf, AsyncAPI); protocol is invocation/transport semantics (`http`, `grpc`,
custom). Families populate `protocols[]` on contract and entity nodes; renderers
decode attachments via `switchback-protocols` rather than re-parsing
`OperationBody.signature`.

I/O traits follow
[ADR 0002](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0002-async-first-traits-with-synchronous-secondary-apis-in-switchback-traits.md):
async-primary APIs with sync-secondary counterparts for external compatibility
(when a caller cannot host an async runtime or wrap the async traits). All seam
types are `Send` / `Sync` as appropriate for async pipelines.

Helper implementations (slug, link check, paths, companion discovery, prose
escaping) live in follow-up work; this crate ships traits and model types only.

See the workspace
[Glossary](https://github.com/canardleteer/switchback-rs/blob/main/docs/GLOSSARY.md)
for terminology.
