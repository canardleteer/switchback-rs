# Acme Example API v2

`acme.example.v2` — catalog CRUD, inventory streaming, draft uploads, and
platform audit export. OpenAPI **3.0.3** (nullable schemas) for version-mix
tests.

Shared problem details are defined in the v2 entry (see `Problem` under
Schemas).

## Catalog and inventory

- Product list/get/create/update/delete under `/products`
- `GET /inventory/watch` — inventory adjustments over SSE
- `PUT /products/drafts` — draft SKU upload via octet-stream

## Platform

`GET /platform/audit/export` returns a paginated audit batch snapshot.
