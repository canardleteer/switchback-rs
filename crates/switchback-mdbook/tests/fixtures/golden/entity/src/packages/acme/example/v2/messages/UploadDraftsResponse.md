# UploadDraftsResponse

UploadDraftsResponse aggregates uploaded draft parts.

```protobuf
message [UploadDraftsResponse](UploadDraftsResponse.md) {
  string draft_id = 1;
  uint32 parts_received = 2;
  [Product](Product.md) product = 3;
}
```

