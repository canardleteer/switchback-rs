# ListProductsRequest

ListProductsRequest paginates catalog inventory.

```protobuf
message [ListProductsRequest](ListProductsRequest.md) {
  [ListOptions](ListOptions.md) options = 1;
  [ProductStatus](../enums/ProductStatus.md) status_filter = 2;
  string search_query = 3 [(buf.validate.field).string.max_len = 256];
}
```

