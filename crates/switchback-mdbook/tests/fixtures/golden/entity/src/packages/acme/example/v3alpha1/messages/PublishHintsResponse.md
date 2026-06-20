# PublishHintsResponse

PublishHintsResponse aggregates uploaded hint parts.

```protobuf
message [PublishHintsResponse](PublishHintsResponse.md) {
  string batch_id = 1;
  uint32 parts_received = 2;
  acme.example.v2.[PageResult](../../v2/messages/PageResult.md) page = 3;
}
```

