# CatalogService

CatalogService documents commerce and inventory RPCs in the v2 package.

**ListProducts** ( [ListProductsRequest](../messages/ListProductsRequest.md) ) returns ( [ListProductsResponse](../messages/ListProductsResponse.md) )

ListProducts returns a paginated product catalog page.

```protobuf
rpc ListProducts (acme.example.v2.[ListProductsRequest](../messages/ListProductsRequest.md)) returns (acme.example.v2.[ListProductsResponse](../messages/ListProductsResponse.md));
```

**GetProduct** ( [GetProductRequest](../messages/GetProductRequest.md) ) returns ( [GetProductResponse](../messages/GetProductResponse.md) )

GetProduct fetches a single product by identifier.

```protobuf
rpc GetProduct (acme.example.v2.[GetProductRequest](../messages/GetProductRequest.md)) returns (acme.example.v2.[GetProductResponse](../messages/GetProductResponse.md));
```

**ApplyInventoryAdjustments** ( [ApplyInventoryAdjustmentsRequest](../messages/ApplyInventoryAdjustmentsRequest.md) ) returns ( [ApplyInventoryAdjustmentsResponse](../messages/ApplyInventoryAdjustmentsResponse.md) )

ApplyInventoryAdjustments applies batched stock deltas.

```protobuf
rpc ApplyInventoryAdjustments (acme.example.v2.[ApplyInventoryAdjustmentsRequest](../messages/ApplyInventoryAdjustmentsRequest.md)) returns (acme.example.v2.[ApplyInventoryAdjustmentsResponse](../messages/ApplyInventoryAdjustmentsResponse.md));
```

**WatchInventory** ( [WatchInventoryRequest](../messages/WatchInventoryRequest.md) ) returns ( [WatchInventoryResponse](../messages/WatchInventoryResponse.md) )

WatchInventory streams inventory adjustments for a warehouse.

```protobuf
rpc WatchInventory (acme.example.v2.[WatchInventoryRequest](../messages/WatchInventoryRequest.md)) returns (stream acme.example.v2.[WatchInventoryResponse](../messages/WatchInventoryResponse.md));
```

**UploadDrafts** ( [UploadDraftsRequest](../messages/UploadDraftsRequest.md) ) returns ( [UploadDraftsResponse](../messages/UploadDraftsResponse.md) )

UploadDrafts accepts a client stream of draft SKU payloads.

```protobuf
rpc UploadDrafts (stream acme.example.v2.[UploadDraftsRequest](../messages/UploadDraftsRequest.md)) returns (acme.example.v2.[UploadDraftsResponse](../messages/UploadDraftsResponse.md));
```

**SyncCatalog** ( [SyncCatalogRequest](../messages/SyncCatalogRequest.md) ) returns ( [SyncCatalogResponse](../messages/SyncCatalogResponse.md) )

SyncCatalog synchronizes catalog revisions over a bidirectional stream.

```protobuf
rpc SyncCatalog (stream acme.example.v2.[SyncCatalogRequest](../messages/SyncCatalogRequest.md)) returns (stream acme.example.v2.[SyncCatalogResponse](../messages/SyncCatalogResponse.md));
```

