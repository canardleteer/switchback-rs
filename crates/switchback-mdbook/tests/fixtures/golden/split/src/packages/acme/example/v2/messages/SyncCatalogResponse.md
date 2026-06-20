# SyncCatalogResponse

SyncCatalogResponse mirrors a bidirectional catalog sync frame.

```protobuf
message [SyncCatalogResponse](SyncCatalogResponse.md) {
  uint64 sequence = 1;
  [StreamCursor](StreamCursor.md) cursor = 2;
  [ErrorDetail](ErrorDetail.md) error = 3;
  bool fin = 4;
}
```

