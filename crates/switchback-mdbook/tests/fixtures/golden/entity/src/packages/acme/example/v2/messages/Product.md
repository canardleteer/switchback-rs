# Product

Product bundles many SKUs for list RPC examples.

```protobuf
message [Product](Product.md) {
  string product_id = 1;
  string display_name = 2;
  repeated [ProductSku](ProductSku.md) skus = 3;
  [TenantRef](TenantRef.md) owner = 4;
  google.protobuf.Timestamp created_at = 5;
  google.protobuf.Timestamp updated_at = 6;
  [Address](Address.md) warehouse = 7;
}
```

