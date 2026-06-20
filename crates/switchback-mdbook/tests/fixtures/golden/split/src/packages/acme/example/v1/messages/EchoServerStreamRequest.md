# EchoServerStreamRequest

EchoServerStreamRequest opens a server-streaming RPC.

```protobuf
message [EchoServerStreamRequest](EchoServerStreamRequest.md) {
  string message = 1;
  uint32 chunk_count = 2;
  google.protobuf.Duration inter_chunk_delay = 3;
  acme.example.v2.[SharedMetadata](../../v2/messages/SharedMetadata.md) metadata = 4;
}
```

