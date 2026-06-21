# mdbook-openrpc example

End-to-end workspace example: load OpenRPC fixtures from
[`switchback-openrpc/tests/fixtures/`](../../crates/switchback-openrpc/tests/fixtures/),
render reference manuals with `switchback-mdbook`, and write one mdBook project
per fixture.

**Default output** builds one **Acme JSON-RPC** book (`acme-api`): a synthetic
three-version corpus (`v1`, `v2`, `v3alpha1`) mirroring the OpenAPI, AsyncAPI,
and protobuf Acme packages. Use `--tier upstream` for vendored
[open-rpc/examples](https://github.com/open-rpc/examples) corpora, or
`--tier micro` / `--all-fixtures` for other hand-maintained corpora.

See also the combined HTTP+gRPC+events+JSON-RPC example at
[`examples/reference-manual/`](../reference-manual/).

## Prerequisites

- Upstream fixtures vendored once per machine (only when using `--tier upstream`
  or upstream fixture ids):

  ```bash
  cargo xtask spec-vendor fetch-fixtures --family openrpc
  ```

  Micro fixtures (including Acme) ship in-repo and need no fetch step.

- Optional preview: **mdbook** CLI pinned to **0.5.3** (see workspace
  `Cargo.toml`).

## Run

From the repository root:

### Default — Acme JSON-RPC mdBook

```bash
cargo run -p mdbook-openrpc-example -- -o /tmp/acme-openrpc-book
```

Preview:

```bash
cd /tmp/acme-openrpc-book/acme-api
mdbook serve
```

### Upstream metrics example

```bash
cargo run -p mdbook-openrpc-example -- --tier upstream -o /tmp/openrpc-books
```

### One fixture

```bash
cargo run -p mdbook-openrpc-example -- --fixture petstore-expanded-1.4 -o /tmp/openrpc-books
```

### Micro fixtures

```bash
cargo run -p mdbook-openrpc-example -- --tier micro -o /tmp/openrpc-micro
```

### All catalogued fixtures

```bash
cargo run -p mdbook-openrpc-example -- --all-fixtures -o /tmp/openrpc-all
```

### Entity layout

```bash
cargo run -p mdbook-openrpc-example -- --layout entity -o /tmp/acme-openrpc-book
```

Layouts: `package` (default), `entity`, `split`.

### Markdown-only refresh

```bash
cargo run -p mdbook-openrpc-example -- --markdown-only -o /tmp/openrpc-out
```

With `--markdown-only`, pass `--summary` to regenerate `src/SUMMARY.md` only.

### List fixture ids

```bash
cargo run -p mdbook-openrpc-example -- --list-fixtures
```

### From a serialized switchback artifact

```bash
cargo run -p mdbook-openrpc-example -- --via-binpb /tmp/switchback.binpb -o /tmp/openrpc-book
```

## Useful flags

| Flag | Purpose |
| --- | --- |
| `-o`, `--output` | Parent output directory (default `./openrpc-books`) |
| `--fixture ID` | Repeatable; render only these ids |
| `--tier example\|upstream\|micro\|all` | Tier when `--fixture` omitted (default `example` → acme-api) |
| `--all-fixtures` | Render every catalogued fixture |
| `--list-fixtures` | Print fixture ids and exit |
| `--markdown-only` / `--no-init` | Skip mdBook scaffold |
| `--summary` | Regenerate SUMMARY (requires `--markdown-only`) |
| `--title` | Override init book title |
| `--layout` | `package`, `entity`, or `split` |

All options:

```bash
cargo run -p mdbook-openrpc-example -- --help
```
