# ProductSku

ProductSku identifies a sellable item in documentation tables.

```protobuf
message [ProductSku](ProductSku.md) {
  string sku = 1 [
      (buf.validate.field).required = true,
      (buf.validate.field).string.min_len = 1,
      (buf.validate.field).string.max_len = 64
    ];
  string title = 2;
  string description = 3;
  [Money](Money.md) price = 4;
  [ProductStatus](../enums/ProductStatus.md) status = 5;
  repeated [Label](Label.md) labels = 6;
}
```

