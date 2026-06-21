# Moving from v1 to v2

v1 keeps thin echo and gateway wrappers. Commerce types (`Product`,
`ListProductsResponse`, shared metadata) live in **v2** and are referenced from
v1 catalog proxy methods.

When migrating clients:

1. Call v2 catalog methods directly (`listProducts`, `watchInventory`) instead
   of the v1 catalog proxy.
2. Replace NDJSON echo watch with v2 inventory streams where event semantics
   overlap.

The v1 entry remains for backward-compatible documentation and cross-version
link tests.
