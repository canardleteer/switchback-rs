# SyncFlagsResponse

SyncFlagsResponse mirrors a bidirectional flag sync frame.

```protobuf
message [SyncFlagsResponse](SyncFlagsResponse.md) {
  uint64 sequence = 1;
  acme.example.v2.[StreamCursor](../../v2/messages/StreamCursor.md) cursor = 2;
  acme.example.v2.[ErrorDetail](../../v2/messages/ErrorDetail.md) error = 3;
  bool fin = 4;
}
```

