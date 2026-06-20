# Vendored OpenRPC JSON Schema meta-schemas

JSON Schema documents from [open-rpc/spec](https://github.com/open-rpc/spec)
(`master`): `spec/1.3/schema.json` and `spec/1.4/schema.json`.

Do not edit files under this tree by hand. Refresh from upstream:

```bash
cargo xtask spec-vendor fetch --family openrpc
# after reviewing diffs, hand-edit meta-schemas.lock.toml sha256 values
cargo xtask spec-vendor validate --family openrpc
```

SHA-256 digests live in `meta-schemas.lock.toml` at the crate root.
