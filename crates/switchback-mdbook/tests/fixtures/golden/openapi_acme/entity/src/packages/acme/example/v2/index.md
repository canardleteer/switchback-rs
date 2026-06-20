# Acme Example API v2

Catalog, inventory, and platform services for the Acme fixture.

## Operations

- [/inventory/watch](operations/GET%20-inventory-watch.md)
- [/platform/audit/export](operations/GET%20-platform-audit-export.md)
- [/products](operations/GET%20-products.md)
- [/products](operations/POST%20-products.md)
- [/products/drafts](operations/PUT%20-products-drafts.md)
- [/products/{product_id}](operations/GET%20-products-%7Bproduct_id%7D.md)
- [/products/{product_id}](operations/PUT%20-products-%7Bproduct_id%7D.md)
- [/products/{product_id}](operations/DELETE%20-products-%7Bproduct_id%7D.md)

## Schemas

- [AuditBatch](schemas/AuditBatch.md)
- [AuditRecord](schemas/AuditRecord.md)
- [ExportAuditBatchResponse](schemas/ExportAuditBatchResponse.md)
- [InventoryAdjustment](schemas/InventoryAdjustment.md)
- [ListOptions](schemas/ListOptions.md)
- [ListProductsResponse](schemas/ListProductsResponse.md)
- [PageResult](schemas/PageResult.md)
- [Problem](schemas/Problem.md)
- [Product](schemas/Product.md)
- [ProductSku](schemas/ProductSku.md)
- [SharedMetadata](schemas/SharedMetadata.md)
- [UploadDraftsResponse](schemas/UploadDraftsResponse.md)

## Parameters

- [ProductId](parameters/ProductId.md)

## Responses

- [BadRequest](responses/BadRequest.md)
- [Forbidden](responses/Forbidden.md)
- [InternalError](responses/InternalError.md)
- [NotFound](responses/NotFound.md)

