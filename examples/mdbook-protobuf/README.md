# mdbook-protobuf example

End-to-end workspace example: load protobuf fixtures with **buf**, render a
reference manual with `switchback-mdbook`, and write an mdBook project to disk.

Default output matches **`protobuf-mdbook --init`**: `book.toml`, theme,
`README.md`, package `SUMMARY`, and API markdown under `src/packages/`.

Proto inputs come from
[`switchback-protobuf/tests/fixtures/proto/`](../../crates/switchback-protobuf/tests/fixtures/proto/).

## Prerequisites

- **[buf](https://buf.build/docs/installation)** on `PATH` (default load path
  uses the buf compiler).
- Optional preview: **mdbook** CLI pinned to **0.5.3** (see workspace
  `Cargo.toml`).

## Run

From the repository root:

### Default — full mdBook project (init)

```bash
cargo run -p mdbook-protobuf-example -- -o /tmp/api-book
```

Writes a complete book tree (for example `book.toml`, `src/SUMMARY.md`,
`theme/`, `README.md`, and package pages).

Preview:

```bash
cd /tmp/api-book
mdbook serve
```

### Entity layout

One markdown file per message, enum, service, and so on:

```bash
cargo run -p mdbook-protobuf-example -- --layout entity -o /tmp/api-book
```

Other layouts: `package` (default), `entity`, `split`.

### Markdown-only refresh

Emit API markdown without scaffolding (like `protobuf-mdbook` without `--init`):

```bash
cargo run -p mdbook-protobuf-example -- --markdown-only -o /tmp/api-book-out
```

With `--markdown-only`, pass `--summary` to regenerate `src/SUMMARY.md` only.

### From a serialized switchback artifact

Skip buf load and render from a wire file:

```bash
cargo run -p mdbook-protobuf-example -- --via-binpb /tmp/switchback.binpb -o /tmp/api-book
```

## Useful flags

| Flag | Purpose |
| --- | --- |
| `-o`, `--output` | Output directory (default `./api-book`) |
| `--markdown-only` / `--no-init` | Skip mdBook scaffold |
| `--summary` | Regenerate SUMMARY (requires `--markdown-only`) |
| `--title` | `book.toml` title when init (default **Protobuf documentation**) |
| `--no-proto-highlight` | Omit protobuf highlight preprocessor in init |
| `--no-cel-highlight` | Omit CEL highlight preprocessor in init |
| `--link-format` | Link formatter name (default `mdbook-relative`) |
| `--no-proto-markdown` | Skip companion proto markdown files |

All options:

```bash
cargo run -p mdbook-protobuf-example -- --help
```
