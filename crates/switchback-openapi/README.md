# switchback-openapi

OpenAPI parser for the switchback-rs toolchain.

Parses OpenAPI 3.0.x and 3.1.x descriptions into a
[`ReferenceManual`](https://github.com/canardleteer/switchback-rs/blob/main/crates/switchback-traits/)
using the shared JSON Schema loader from `switchback-jsonschema`. Version shape
is preserved (`nullable` vs `type: [T, "null"]`).

## Library usage

```rust
use switchback_openapi::load::{load, LoadArgs};
use std::path::PathBuf;

let args = LoadArgs {
    module_root: PathBuf::from("api"),
    inputs: vec![PathBuf::from("openapi.yaml")],
    search_roots: vec![PathBuf::from("api")],
    title: None,
};
let manual = load(&args)?;
```

Render with `switchback-mdbook` (family-aware YAML fences and operation blocks).

The CLI (`switchback-openapi` binary) remains a stub; use the library API or
[`examples/mdbook-openapi`](../../examples/mdbook-openapi/) to render fixture
corpora to mdBook projects.

## Workspace example

```bash
cargo xtask spec-vendor fetch-fixtures --family openapi   # once
cargo run -p mdbook-openapi-example -- -o /tmp/openapi-books
```

See
[`examples/mdbook-openapi/README.md`](../../examples/mdbook-openapi/README.md).

## Vendored meta-schemas

JSON Schema meta-schemas from
[OAI/spec.openapis.org](https://github.com/OAI/spec.openapis.org) live under
`meta-schemas/`:

```bash
cargo xtask spec-vendor fetch --family openapi
cargo xtask spec-vendor validate --family openapi
```

## Example API fixtures (tests)

Upstream corpus (locked, do not hand-edit):

```bash
cargo xtask spec-vendor fetch-fixtures --family openapi
cargo xtask spec-vendor validate-fixtures --family openapi
```

See
[`tests/fixtures/upstream/FIXTURES.md`](tests/fixtures/upstream/FIXTURES.md).
Micro fixtures under `tests/fixtures/micro/` are hand-maintained regressions.

## Status

- Implemented: `load()` → populate → `ReferenceManual`,
  tag/`x-tagGroups`/components/untagged grouping, six entity categories,
  structural `$ref` → `refs`, beside companions, mdBook render path.
- Deferred: CLI, `--validate`, prose intra-links (`OpenApiLinkExtractor` returns
  empty `intra_links`).

See
[ADR 0010](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0010-openapi-parser-library-mvp-in-switchback-openapi.md).
