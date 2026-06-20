# StreamChunk

StreamChunk is a unit payload in streaming RPC fixtures.

```protobuf
message [StreamChunk](StreamChunk.md) {
  [StreamCursor](StreamCursor.md) cursor = 1;
  [PayloadEnvelope](PayloadEnvelope.md) payload = 2;
  bool terminal = 3;
}
```

