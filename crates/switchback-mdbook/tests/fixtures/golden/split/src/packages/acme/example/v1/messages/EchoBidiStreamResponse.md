# EchoBidiStreamResponse

EchoBidiStreamResponse mirrors a bidirectional frame back to the client.

```protobuf
message [EchoBidiStreamResponse](EchoBidiStreamResponse.md) {
  string message = 1;
  uint64 sequence = 2;
  bool fin = 3;
  acme.example.v2.[StreamCursor](../../v2/messages/StreamCursor.md) cursor = 4;
}
```

