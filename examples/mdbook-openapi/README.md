# mdbook-openapi example

End-to-end workspace example: load OpenAPI fixtures from
[`switchback-openapi/tests/fixtures/`](../../crates/switchback-openapi/tests/fixtures/),
render reference manuals with `switchback-mdbook`, and write one mdBook project
per fixture.

Default output builds **four upstream books** (the 2×2 OpenAPI 3.0 / 3.1
matrix). Use `--tier micro` or `--all-fixtures` for the hand-maintained corpora
too.

## Prerequisites

- Upstream fixtures vendored once per machine:

  ```bash
  cargo xtask spec-vendor fetch-fixtures --family openapi
  ```

  Micro fixtures ship in-repo and need no fetch step.

- Optional preview: **mdbook** CLI pinned to **0.5.3** (see workspace
  `Cargo.toml`).

## Run

From the repository root:

### Default — four upstream mdBook projects

```bash
cargo run -p mdbook-openapi-example -- -o /tmp/openapi-books
```

Creates:

| Directory | Fixture |
| --- | --- |
| `petstore-3.0/` | OAI petstore (3.0) |
| `link-example-3.0/` | OAI link example (3.0) |
| `tictactoe-3.1/` | learn.openapis.org tictactoe (3.1) |
| `webhook-3.1/` | learn.openapis.org webhook (3.1) |

Preview one book:

```bash
cd /tmp/openapi-books/tictactoe-3.1
mdbook serve
```

### One fixture

```bash
cargo run -p mdbook-openapi-example -- --fixture petstore-3.0 -o /tmp/openapi-books
# → /tmp/openapi-books/petstore-3.0/
```

### Micro fixtures (four books)

```bash
cargo run -p mdbook-openapi-example -- --tier micro -o /tmp/openapi-micro
```

### All eight fixtures

```bash
cargo run -p mdbook-openapi-example -- --all-fixtures -o /tmp/openapi-all
```

### Entity layout

```bash
cargo run -p mdbook-openapi-example -- --layout entity -o /tmp/openapi-books
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

## Useful flags

| Flag | Purpose |
| --- | --- |
| `-o`, `--output` | Parent output directory (default `./openapi-books`) |
| `--fixture ID` | Repeatable; render only these ids |
| `--tier upstream\|micro\|all` | Tier when `--fixture` omitted (default `upstream`) |
| `--all-fixtures` | Render all eight catalogued fixtures |
| `--list-fixtures` | Print fixture ids and exit |
| `--markdown-only` / `--no-init` | Skip mdBook scaffold |
| `--summary` | Regenerate SUMMARY (requires `--markdown-only`) |
| `--title` | Override init book title |
| `--layout` | `package`, `entity`, or `split` |

All options:

```bash
cargo run -p mdbook-openapi-example -- --help
```
