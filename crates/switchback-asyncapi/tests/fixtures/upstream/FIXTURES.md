# Upstream AsyncAPI example fixtures

Vendored from [asyncapi/spec](https://github.com/asyncapi/spec) at the commit
pinned in [`example-fixtures.lock.toml`](../../../example-fixtures.lock.toml).

**Do not hand-edit.** Refresh with:

```bash
cargo xtask spec-vendor fetch-fixtures --family asyncapi
cargo xtask spec-vendor fetch-fixtures --family asyncapi --write-lock  # recompute SHA-256
```

Validate in CI via
`cargo xtask spec-vendor validate-fixtures --family asyncapi`.

| Id | File | Upstream |
| --- | --- | --- |
| `streetlights-kafka` | `streetlights-kafka/asyncapi.yaml` | `examples/streetlights-kafka.yml` @ v2.6.0 |
