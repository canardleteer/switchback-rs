# RelayFrame

RelayFrame is one unit in a bidirectional relay stream.

```protobuf
message [RelayFrame](RelayFrame.md) {
  uint64 sequence = 1;
  bytes payload = 2;
  acme.example.v2.[SharedMetadata](../../v2/messages/SharedMetadata.md) metadata = 3;
  bool fin = 4;
}
```

