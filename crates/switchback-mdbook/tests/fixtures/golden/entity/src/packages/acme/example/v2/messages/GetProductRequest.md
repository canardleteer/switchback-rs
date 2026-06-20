# GetProductRequest

GetProductRequest fetches a single product by id.

```protobuf
message [GetProductRequest](GetProductRequest.md) {
  string product_id = 1 [
      (buf.validate.field).required = true,
      (buf.validate.field).string.uuid = true
    ];
}
```

