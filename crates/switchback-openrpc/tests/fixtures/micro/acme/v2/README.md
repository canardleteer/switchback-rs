# Acme Example API v2

`acme.example.v2` — catalog CRUD, inventory streaming, draft uploads, and
platform audit export. OpenRPC **1.3.0** for version-mix tests.

Shared problem details are defined in `../shared/schemas.json`.

## Catalog and inventory

- Product list/get/create/update/delete methods
- `watchInventory` — inventory adjustment stream
- `uploadDrafts` — draft SKU upload payloads

## Platform

`exportAuditBatch` returns a paginated audit batch snapshot.
