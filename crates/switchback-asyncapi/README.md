# switchback-asyncapi

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

AsyncAPI parser for the switchback-rs toolchain (stub).

## Vendored meta-schemas

This crate vendors the AsyncAPI JSON Schema validation corpus from
[asyncapi/spec-json-schemas](https://github.com/asyncapi/spec-json-schemas)
under `meta-schemas/` (`schemas/`, `definitions/`, `bindings/`, `common/`,
`extensions/`). Intended as test data for `switchback-jsonschema` before the
family parser lands.

JSON Schema alone does not fully validate AsyncAPI — custom parser rules are
deferred. See
[ADR 0005](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0005-vendored-json-schema-meta-schemas-per-parser-crate.md).

```rust
use switchback_asyncapi::meta_schemas::{self, SCHEMAS_3_1_0};

let schema = meta_schemas::read(&SCHEMAS_3_1_0)?;
```

Refresh vendored files:

```bash
cargo xtask spec-vendor fetch --family asyncapi
cargo xtask spec-vendor validate --family asyncapi
```

After `fetch`, hand-edit `meta-schemas.lock.toml` SHA-256 values when updates
are intentional.
