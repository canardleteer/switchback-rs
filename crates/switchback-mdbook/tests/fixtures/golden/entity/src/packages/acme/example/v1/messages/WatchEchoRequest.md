# WatchEchoRequest

WatchEchoRequest subscribes to echo events for a topic.

```protobuf
message [WatchEchoRequest](WatchEchoRequest.md) {
  string topic = 1;
  acme.example.v2.[TimeWindow](../../v2/messages/TimeWindow.md) window = 2;
  repeated acme.example.v2.[FilterExpression](../../v2/messages/FilterExpression.md) filters = 3;
}
```

