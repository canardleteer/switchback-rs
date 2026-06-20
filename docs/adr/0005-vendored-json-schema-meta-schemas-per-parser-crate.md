# 5. Vendored JSON Schema meta-schemas per parser crate

Date: 2026-06-19

## Status

Proposed

## Context

Before implementing `switchback-jsonschema`, we need real JSON Schema corpora to
exercise load, `$ref` resolution, and validation. OpenAPI, AsyncAPI, and OpenRPC
each publish JSON Schema meta-schemas on GitHub; we vendor them into the
respective parser crates as test data and future `--validate` inputs.

## Decision

Vendor JSON Schema meta-schemas into `switchback-openapi`,
`switchback-asyncapi`, and `switchback-openrpc` under each crate's
`meta-schemas/` directory, with SHA-256 recorded in `meta-schemas.lock.toml`.
Expose `pub mod meta_schemas` with `MetaSchemaAsset`, `ALL`, named entry-point
constants, and `read()` helpers.

Maintain assets with `cargo xtask spec-vendor fetch` (redownload only; never
auto-updates lock hashes) and `cargo xtask spec-vendor validate` (offline
SHA-256 check; wired into `cargo xtask ci`). Initial bootstrap uses
`--write-lock` to populate the lock file and generated `meta_schemas_assets.rs`.

OpenAPI: all JSON blobs for 2.0, 3.0, 3.1, and 3.2 from OAI/spec.openapis.org.
AsyncAPI: full validation corpus from asyncapi/spec-json-schemas (`schemas/`
with both with-`$id` and without-`$id` bundles, `definitions/`, `bindings/`,
`common/`, `extensions/`). OpenRPC: `spec/1.3/schema.json` and
`spec/1.4/schema.json` from open-rpc/spec.

JSON Schema only in this pass. AsyncAPI custom validation rules beyond JSON
Schema (duplicate operationIds, channel parameter checks, etc.) are deferred to
parser logic.

## Consequences

Large vendored trees (~720 JSON files in AsyncAPI) ship in crate tarballs; lock
files are long but give per-file integrity. Manual SHA-256 edits after fetch are
required by policy. `switchback-jsonschema` can depend on these crates for
fixtures before family parsers exist. Non–JSON Schema validation remains
unimplemented.
