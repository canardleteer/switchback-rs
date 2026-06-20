# PublishHintsRequest

PublishHintsRequest is one client-streaming platform hint chunk.

```protobuf
message [PublishHintsRequest](PublishHintsRequest.md) {
  string batch_id = 1;
  [PlatformHint](PlatformHint.md) hint = 2;
  uint32 part_index = 3;
  bool last_part = 4;
}
```

