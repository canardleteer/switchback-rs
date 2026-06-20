# mdbook-openapi example

End-to-end workspace example: load OpenAPI fixtures from
[`switchback-openapi/tests/fixtures/`](../../crates/switchback-openapi/tests/fixtures/),
render reference manuals with `switchback-mdbook`, and write one mdBook project
per fixture.

**Default output** builds one **Acme APIs** book (`acme-api`): a synthetic
three-version corpus (`v1`, `v2`, `v3alpha1`) mirroring the protobuf Acme
packages. Use `--tier upstream` for the vendored learn.openapis.org matrix, or
`--tier micro` / `--all-fixtures` for other hand-maintained corpora.

See also the combined HTTP+gRPC example at
[`examples/reference-manual/`](../reference-manual/).

## Prerequisites

- Upstream fixtures vendored once per machine (only when using `--tier upstream`
  or upstream fixture ids):

  ```bash
  cargo xtask spec-vendor fetch-fixtures --family openapi
  ```

  Micro fixtures (including Acme) ship in-repo and need no fetch step.

- Optional preview: **mdbook** CLI pinned to **0.5.3** (see workspace
  `Cargo.toml`).

## Run

From the repository root:

### Default — Acme APIs mdBook

```bash
cargo run -p mdbook-openapi-example -- -o /tmp/acme-book
```

Creates `acme-api/` with three version groups, streaming operations, shared
error components, and companion markdown.

Preview:

```bash
cd /tmp/acme-book/acme-api
mdbook serve
```

### Upstream fixtures (four books)

```bash
cargo run -p mdbook-openapi-example -- --tier upstream -o /tmp/openapi-books
```

### One fixture

```bash
cargo run -p mdbook-openapi-example -- --fixture tictactoe-3.1 -o /tmp/openapi-books
```

### Micro fixtures

```bash
cargo run -p mdbook-openapi-example -- --tier micro -o /tmp/openapi-micro
```

### All catalogued fixtures

```bash
cargo run -p mdbook-openapi-example -- --all-fixtures -o /tmp/openapi-all
```

### Entity layout

```bash
cargo run -p mdbook-openapi-example -- --layout entity -o /tmp/acme-book
```

Layouts: `package` (default), `entity`, `split`.

### Markdown-only refresh

```bash
cargo run -p mdbook-openapi-example -- --markdown-only -o /tmp/openapi-out
```

With `--markdown-only`, pass `--summary` to regenerate `src/SUMMARY.md` only.

### List fixture ids

```bash
cargo run -p mdbook-openapi-example -- --list-fixtures
```

### From a serialized switchback artifact

```bash
cargo run -p mdbook-openapi-example -- --via-binpb /tmp/switchback.binpb -o /tmp/api-book
```

## Acme corpus notes

The Acme OpenAPI fixture parallels
[`switchback-protobuf/tests/fixtures/proto/acme/`](../../crates/switchback-protobuf/tests/fixtures/proto/acme/).
All three API versions are documented as concurrently supported, each with its
own `servers[]` URL.

**OpenAPI modeling limits:** bidirectional streaming is not a single operation
in OpenAPI (HTTP supports relay patterns; the v1 corpus uses SSE + POST and
companion prose explains the gap). gRPC bidi remains on the protobuf side in
[`examples/reference-manual/`](../reference-manual/).

## Useful flags

| Flag | Purpose |
| --- | --- |
| `-o`, `--output` | Parent output directory (default `./openapi-books`) |
| `--fixture ID` | Repeatable; render only these ids |
| `--tier example\|upstream\|micro\|all` | Tier when `--fixture` omitted (default `example` → acme-api) |
| `--all-fixtures` | Render every catalogued fixture |
| `--list-fixtures` | Print fixture ids and exit |
| `--markdown-only` / `--no-init` | Skip mdBook scaffold |
| `--summary` | Regenerate SUMMARY (requires `--markdown-only`) |
| `--title` | Override init book title |
| `--layout` | `package`, `entity`, or `split` |
| `--openapi-summary-label` | SUMMARY link text: `endpoint` (path only, default), `summary`, or `prefixed` |
| `--openapi-operation-source` | Raw operation YAML: `collapsed`, `trimmed`, or `hidden` |

All options:

```bash
cargo run -p mdbook-openapi-example -- --help
```
