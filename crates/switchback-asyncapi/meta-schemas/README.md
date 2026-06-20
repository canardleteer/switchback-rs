# Vendored AsyncAPI JSON Schema meta-schemas

JSON Schema validation corpus from
[asyncapi/spec-json-schemas](https://github.com/asyncapi/spec-json-schemas)
(`master`): `schemas/`, `definitions/`, `bindings/`, `common/`, and
`extensions/`.

This is the complete **JSON Schema substrate** for AsyncAPI documents. JSON
Schema alone does not fully validate AsyncAPI — see upstream
[custom validation needs](https://github.com/asyncapi/spec-json-schemas#custom-validation-needs)
and
[ADR 0005](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0005-vendored-json-schema-meta-schemas-per-parser-crate.md).

Do not edit files under this tree by hand. Refresh from upstream:

```bash
cargo xtask spec-vendor fetch --family asyncapi
# after reviewing diffs, hand-edit meta-schemas.lock.toml sha256 values
cargo xtask spec-vendor validate --family asyncapi
```

SHA-256 digests live in `meta-schemas.lock.toml` at the crate root.
