# EchoBidiStreamRequest

EchoBidiStreamRequest is one frame in a bidirectional echo session.

```protobuf
message [EchoBidiStreamRequest](EchoBidiStreamRequest.md) {
  string message = 1;
  uint64 sequence = 2;
  bool fin = 3;
}
```

