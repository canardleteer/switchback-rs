# Upstream OpenRPC example fixtures

Vendored from [open-rpc/examples](https://github.com/open-rpc/examples) at the
commit pinned in
[`example-fixtures.lock.toml`](../../../example-fixtures.lock.toml).

**Do not hand-edit.** Refresh with:

```bash
cargo xtask spec-vendor fetch-fixtures --family openrpc
cargo xtask spec-vendor fetch-fixtures --family openrpc --write-lock  # recompute SHA-256
```

Validate in CI via
`cargo xtask spec-vendor validate-fixtures --family openrpc`.

| Id | File | Upstream | Notes |
| --- | --- | --- | --- |
| `metrics-1.3` | `metrics-1.3/openrpc.json` | `metrics-openrpc.json` | Native `openrpc: 1.3.0`; notification method |
| `petstore-expanded-1.4` | `petstore-expanded-1.4/openrpc.json` | `petstore-expanded-openrpc.json` | Fetch bumps `openrpc` to `1.4.0` (upstream still ships `1.0.0-rc1`) |
| `link-example-1.4` | `link-example-1.4/openrpc.json` | `link-example-openrpc.json` | Fetch bumps `openrpc` to `1.4.0` (upstream still ships `1.0.0-rc1`) |

Hand-maintained regressions live under [`../micro/`](../micro/).
