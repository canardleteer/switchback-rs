# switchback-openrpc

OpenRPC parser for the switchback-rs toolchain (stub).

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

After `fetch`, hand-edit `meta-schemas.lock.toml` SHA-256 values when updates
are intentional.
