# StreamCursor

StreamCursor supports resumable stream examples.

```protobuf
message [StreamCursor](StreamCursor.md) {
  string stream_id = 1;
  uint64 sequence = 2;
  google.protobuf.Timestamp emitted_at = 3;
}
```

