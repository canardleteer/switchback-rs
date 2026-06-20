# SyncFlagsRequest

SyncFlagsRequest is one frame in a bidirectional flag sync session.

```protobuf
message [SyncFlagsRequest](SyncFlagsRequest.md) {
  uint64 sequence = 1;
  [FeatureFlag](FeatureFlag.md) flag = 2;
  bool fin = 3;
}
```

