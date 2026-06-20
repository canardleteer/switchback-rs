# UploadDraftsRequest

UploadDraftsRequest is one client-streaming product draft chunk.

```protobuf
message [UploadDraftsRequest](UploadDraftsRequest.md) {
  string draft_id = 1;
  [ProductSku](ProductSku.md) sku = 2;
  uint32 part_index = 3;
  bool last_part = 4;
}
```

