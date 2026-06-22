# reference-manual example

End-to-end workspace example: assemble **Acme** HTTP (OpenAPI), gRPC
(protobuf), events (AsyncAPI), and JSON-RPC (OpenRPC) for **v1**, **v2**, and
**v3alpha1** into one
[`ReferenceManual`](../../docs/GLOSSARY.md#reference-manual), then render with
`switchback-mdbook`.

This demonstrates
[ADR 0014](../../docs/adr/0014-multi-contract-reference-manual-assembly.md)
multi-contract assembly. Group ids are prefixed as `{family}.{package}` (for
example `openapi.acme.example.v1`, `protobuf.acme.example.v1`,
`asyncapi.acme.example.v1`, and `openrpc.acme.example.v1`) so mdBook indexes stay
unique.

## Prerequisites

- Buf/proto deps export runs automatically on first load (same as
  `mdbook-protobuf`).
- Optional preview: **mdbook** CLI pinned to **0.5.3** (see workspace
  `Cargo.toml`).

## Run

From the repository root:

```bash
cargo run -p reference-manual-example -- -o /tmp/acme-ref
```

Preview:

```bash
cd /tmp/acme-ref
mdbook serve
```

### Entity or split layout

```bash
cargo run -p reference-manual-example -- --layout split -o /tmp/acme-ref
```

Mixed-family books use top-level SUMMARY sections **HTTP (OpenAPI)**, **gRPC
(Protobuf)**, **Events (AsyncAPI)**, and **JSON-RPC (OpenRPC)**.

### Serialized artifact round-trip

```bash
cargo run -p reference-manual-example -- --emit-binpb /tmp/acme-ref.binpb -o /tmp/acme-ref
cargo run -p reference-manual-example -- --via-binpb /tmp/acme-ref.binpb -o /tmp/acme-ref-from-wire
```

## Manifest

[`module.yaml`](module.yaml) documents the MVP schema parsed by this example
(id, title, overview, `contracts[]` with `family`, `module_root`, `inputs[]`).
Input lists mirror `EXAMPLE_ACME_INPUTS` and `EXAMPLE_PROTO_INPUTS` in the
family crates. Full `module.yaml` parsing inside family parsers remains
deferred.

## Scope and limits

- **No cross-family intra-links** in prose yet; HTTP, gRPC, and events sections
  link within their contract only.
- OpenAPI does not model bidirectional streaming as a single operation; the
  Acme HTTP corpus approximates relay with SSE + POST (see Acme v1 companion
  README). gRPC bidi remains on the protobuf side.

Corpus paths point at existing fixtures under
[`switchback-openapi/tests/fixtures/micro/acme/`](../../crates/switchback-openapi/tests/fixtures/micro/acme/),
[`switchback-protobuf/tests/fixtures/proto/`](../../crates/switchback-protobuf/tests/fixtures/proto/),
and
[`switchback-asyncapi/tests/fixtures/micro/acme/`](../../crates/switchback-asyncapi/tests/fixtures/micro/acme/),
and
[`switchback-openrpc/tests/fixtures/micro/acme/`](../../crates/switchback-openrpc/tests/fixtures/micro/acme/).
See also [`examples/mdbook-openrpc/`](../mdbook-openrpc/) for a single-family
JSON-RPC book.
