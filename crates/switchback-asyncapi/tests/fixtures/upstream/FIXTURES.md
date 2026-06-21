# Upstream AsyncAPI example fixtures

Vendored from [asyncapi/spec](https://github.com/asyncapi/spec) at the commits
pinned in [`example-fixtures.lock.toml`](../../../example-fixtures.lock.toml).

**Do not hand-edit.** Refresh with:

```bash
cargo xtask spec-vendor fetch-fixtures --family asyncapi
cargo xtask spec-vendor fetch-fixtures --family asyncapi --write-lock  # recompute SHA-256
```

Validate in CI via
`cargo xtask spec-vendor validate-fixtures --family asyncapi`.

## Corpus (2×2 matrix)

| Id | File | Upstream | Spec |
| --- | --- | --- | --- |
| `streetlights-kafka` | `streetlights-kafka/asyncapi.yaml` | `examples/streetlights-kafka.yml` | **2.6.0** @ `v2.6.0` |
| `streetlights-mqtt` | `streetlights-mqtt/asyncapi.yaml` | `examples/streetlights-mqtt.yml` | **2.6.0** @ `v2.6.0` |
| `simple-3.1` | `simple-3.1/asyncapi.yaml` | `examples/simple-asyncapi.yml` | **3.1.0** @ `v3.1.0` |
| `streetlights-kafka-3.1` | `streetlights-kafka-3.1/asyncapi.yaml` | `examples/streetlights-kafka-asyncapi.yml` | **3.1.0** @ `v3.1.0` |

Hand-maintained regressions live under [`../micro/`](../micro/).
