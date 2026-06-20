# Upstream OpenAPI example fixtures

Vendored example API descriptions for integration tests in `switchback-openapi`
and `switchback-mdbook`. Locked in
[`example-fixtures.lock.toml`](../../../example-fixtures.lock.toml).

Refresh:

```bash
cargo xtask spec-vendor fetch-fixtures --family openapi
cargo xtask spec-vendor validate-fixtures --family openapi
```

## Corpus (2×2 matrix)

| Tier | OAS | File | Upstream | Exercises |
| --- | --- | --- | --- | --- |
| Low | 3.0 | `oas3.0-petstore/petstore.yaml` | [OAI/OpenAPI-Specification](https://github.com/OAI/OpenAPI-Specification) @ `f8449d1` | tags, paths, components, parameters |
| High | 3.0 | `oas3.0-link-example/link-example.yaml` | OAI/OpenAPI-Specification @ `f8449d1` | `components.links`, cross-ref wiring |
| Low | 3.1 | `oas3.1-tictactoe/tictactoe.yaml` | [OAI/learn.openapis.org](https://github.com/OAI/learn.openapis.org) @ `4375654` | OpenAPI 3.1 root, JSON Schema 2020-12 keywords |
| High | 3.1 | `oas3.1-webhook/webhook-example.yaml` | OAI/learn.openapis.org @ `4375654` | webhooks, richer 3.1 operation shapes |

All assets are **Apache-2.0**. OAI does not publish a standalone 3.1 petstore
under OpenAPI-Specification; the 3.1 low fixture uses learn.openapis.org
instead (same license family).

Hand-maintained regressions live under [`../micro/`](../micro/) — see
[`AGENTS.md`](https://github.com/canardleteer/switchback-rs/blob/main/AGENTS.md)
(parser test fixtures section).
