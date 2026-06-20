# EchoServerStreamResponse

EchoServerStreamResponse is one chunk in a server stream.

```protobuf
message [EchoServerStreamResponse](EchoServerStreamResponse.md) {
  string message = 1;
  acme.example.v2.[StreamCursor](../../v2/messages/StreamCursor.md) cursor = 2;
  uint32 index = 3;
}
```

