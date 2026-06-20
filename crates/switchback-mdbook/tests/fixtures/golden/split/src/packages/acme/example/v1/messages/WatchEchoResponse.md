# WatchEchoResponse

WatchEchoResponse is pushed on the WatchEcho server stream.

```protobuf
message [WatchEchoResponse](WatchEchoResponse.md) {
  string event_id = 1;
  [EchoUnaryResponse](EchoUnaryResponse.md) payload = 2;
  google.protobuf.Timestamp observed_at = 3;
}
```

