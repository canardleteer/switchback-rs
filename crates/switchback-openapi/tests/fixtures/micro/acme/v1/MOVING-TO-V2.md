# Moving from v1 to v2

v1 keeps thin echo and gateway wrappers. Commerce types (`Product`,
`PageResult`, shared metadata) live in **v2** and are referenced from v1 catalog
proxy routes.

When migrating clients:

1. Call v2 catalog endpoints directly (`/products`, inventory watch) instead of
   the v1 catalog proxy.
2. Replace NDJSON echo watch with v2 inventory SSE where event semantics
   overlap.
3. Expect nullable fields in v2 (OpenAPI 3.0.3) versus v2 optional/null union in
   3.1.0 consumers.

The v1 entry remains for backward-compatible documentation and cross-version
link tests.
