# switchback-openrpc

> [!WARNING]
> Early prototype while exploring design and aiming for equivalence
> with [protobuf-mdbook](https://github.com/canardleteer/protobuf-mdbook), while
> expanding scope through traits and intermediary on-disk representation.
>
> This is not ready for adoption, nor even stable at a `v1alpha1` yet. You'll
> want to keep eyes on the repository for development.
>
> A lot of this is clanker driven, so vetting a good human read through pass
> hasn't been completed yet.

OpenRPC parser for the switchback-rs toolchain. Parses OpenRPC **1.3.x** and
**1.4.x** documents into a
[`ReferenceManual`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-traits/)
via `load()`. Methods map to `operation` entities; component schemas and content
descriptors map to `schema` and `parameter` entities. JSON Schema loading,
envelope rendering, and `$ref` resolution reuse `switchback-jsonschema`.

## Load Acme corpus

```rust
use switchback_openrpc::load_acme_example;

let manual = load_acme_example()?;
```

The default **`acme-api`** fixture is the three-version micro corpus under
`tests/fixtures/micro/acme/` (`v1`, `v2`, `v3alpha1`).

## mdBook example

Render the Acme corpus to an mdBook project:

```bash
cargo run -p mdbook-openrpc-example -- -o /tmp/acme-openrpc-book
```

See
[`examples/mdbook-openrpc/`](https://github.com/canardleteer/switchback-rs/blob/main/examples/mdbook-openrpc/).

## Vendored meta-schemas

This crate vendors OpenRPC JSON Schema meta-schemas from
[open-rpc/spec](https://github.com/open-rpc/spec) under `meta-schemas/`.

```rust
use switchback_openrpc::meta_schemas::{self, SCHEMA_1_4};

let schema = meta_schemas::read(&SCHEMA_1_4)?;
```

Refresh vendored files:

```bash
cargo xtask spec-vendor fetch --family openrpc
cargo xtask spec-vendor validate --family openrpc
```

Upstream example API descriptions (metrics, petstore, link example):

```bash
cargo xtask spec-vendor fetch-fixtures --family openrpc
cargo xtask example-fixtures validate --family openrpc
```

After `fetch`, hand-edit lock SHA-256 values when updates are intentional.

## ADR

Parser scope and deferrals are recorded in
[ADR 0019](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0019-openrpc-parser-library-in-switchback-openrpc.md).
