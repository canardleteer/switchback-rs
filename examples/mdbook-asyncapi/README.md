# mdbook-asyncapi example

End-to-end workspace example: load AsyncAPI fixtures from
[`switchback-asyncapi/tests/fixtures/`](../../crates/switchback-asyncapi/tests/fixtures/),
render reference manuals with `switchback-mdbook`, and write one mdBook project
per fixture.

**Default output** builds one **Acme Events** book (`acme-api`): a synthetic
three-version corpus (`v1`, `v2`, `v3alpha1`) mirroring the OpenAPI and
protobuf Acme packages. Use `--tier upstream` for the vendored streetlights
Kafka example, or `--tier micro` / `--all-fixtures` for other hand-maintained
corpora.

See also the combined HTTP+gRPC+events example at
[`examples/reference-manual/`](../reference-manual/).

## Prerequisites

- Upstream fixtures vendored once per machine (only when using `--tier upstream`
  or upstream fixture ids):

  ```bash
  cargo xtask spec-vendor fetch-fixtures --family asyncapi
  ```

  Micro fixtures (including Acme) ship in-repo and need no fetch step.

- Optional preview: **mdbook** CLI pinned to **0.5.3** (see workspace
  `Cargo.toml`).

## Run

From the repository root:

### Default — Acme Events mdBook

```bash
cargo run -p mdbook-asyncapi-example -- -o /tmp/acme-events-book
```

Preview:

```bash
cd /tmp/acme-events-book/acme-api
mdbook serve
```

### Upstream streetlights Kafka

```bash
cargo run -p mdbook-asyncapi-example -- --tier upstream -o /tmp/asyncapi-books
```

### One fixture

```bash
cargo run -p mdbook-asyncapi-example -- --fixture streetlights-kafka -o /tmp/asyncapi-books
```

### Micro fixtures

```bash
cargo run -p mdbook-asyncapi-example -- --tier micro -o /tmp/asyncapi-micro
```

### All catalogued fixtures

```bash
cargo run -p mdbook-asyncapi-example -- --all-fixtures -o /tmp/asyncapi-all
```

### Entity layout

```bash
cargo run -p mdbook-asyncapi-example -- --layout entity -o /tmp/acme-events-book
```

Layouts: `package` (default), `entity`, `split`.

### Markdown-only refresh

```bash
cargo run -p mdbook-asyncapi-example -- --markdown-only -o /tmp/asyncapi-out
```

With `--markdown-only`, pass `--summary` to regenerate `src/SUMMARY.md` only.

### List fixture ids

```bash
cargo run -p mdbook-asyncapi-example -- --list-fixtures
```

### From a serialized switchback artifact

```bash
cargo run -p mdbook-asyncapi-example -- --via-binpb /tmp/switchback.binpb -o /tmp/events-book
```

## Useful flags

| Flag | Purpose |
| --- | --- |
| `-o`, `--output` | Parent output directory (default `./asyncapi-books`) |
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
cargo run -p mdbook-asyncapi-example -- --help
```
