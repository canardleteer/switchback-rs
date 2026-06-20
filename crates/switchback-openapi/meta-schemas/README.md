# Vendored OpenAPI JSON Schema meta-schemas

JSON Schema documents from
[OAI/spec.openapis.org](https://github.com/OAI/spec.openapis.org) (`main`),
covering OpenAPI **2.0**, **3.0**, **3.1**, and **3.2** minor lines.

Do not edit files under this tree by hand. Refresh from upstream:

```bash
cargo xtask spec-vendor fetch --family openapi
# after reviewing diffs, hand-edit meta-schemas.lock.toml sha256 values
cargo xtask spec-vendor validate --family openapi
```

SHA-256 digests live in `meta-schemas.lock.toml` at the crate root.
