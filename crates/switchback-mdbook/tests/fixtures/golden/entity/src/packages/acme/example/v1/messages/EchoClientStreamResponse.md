# EchoClientStreamResponse

EchoClientStreamResponse aggregates a client stream on the server.

```protobuf
message [EchoClientStreamResponse](EchoClientStreamResponse.md) {
  string joined_message = 1;
  uint32 parts_received = 2;
  google.protobuf.Timestamp completed_at = 3;
}
```

