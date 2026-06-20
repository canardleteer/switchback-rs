# switchback-openapi

OpenAPI parser for the switchback-rs toolchain (stub).

## Vendored meta-schemas

This crate vendors OpenAPI JSON Schema meta-schemas from
[OAI/spec.openapis.org](https://github.com/OAI/spec.openapis.org) under
`meta-schemas/`. Access them via `meta_schemas`:

```rust
use switchback_openapi::meta_schemas::{self, OAS_3_1_SCHEMA_2025_11_23};

let schema = meta_schemas::read(&OAS_3_1_SCHEMA_2025_11_23)?;
```

Refresh vendored files:

```bash
cargo xtask spec-vendor fetch --family openapi
cargo xtask spec-vendor validate --family openapi
```

After `fetch`, hand-edit `meta-schemas.lock.toml` SHA-256 values when updates
are intentional (`validate` prints computed hashes on mismatch).
