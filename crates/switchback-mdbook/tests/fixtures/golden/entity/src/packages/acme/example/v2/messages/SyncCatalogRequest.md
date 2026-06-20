# SyncCatalogRequest

SyncCatalogRequest is one frame in a bidirectional catalog sync session.

```protobuf
message [SyncCatalogRequest](SyncCatalogRequest.md) {
  uint64 sequence = 1;
  string catalog_revision = 2;
  [PayloadEnvelope](PayloadEnvelope.md) payload = 3;
  bool fin = 4;
}
```

