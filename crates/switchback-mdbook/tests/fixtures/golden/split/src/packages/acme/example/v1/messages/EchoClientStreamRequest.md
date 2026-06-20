# EchoClientStreamRequest

EchoClientStreamRequest is one chunk in a client stream upload.

```protobuf
message [EchoClientStreamRequest](EchoClientStreamRequest.md) {
  string message = 1;
  uint32 part_index = 2;
  bool last_part = 3;
}
```

