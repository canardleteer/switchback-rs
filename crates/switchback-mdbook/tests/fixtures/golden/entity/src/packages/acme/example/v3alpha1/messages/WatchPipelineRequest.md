# WatchPipelineRequest

WatchPipelineRequest subscribes to pipeline step events.

```protobuf
message [WatchPipelineRequest](WatchPipelineRequest.md) {
  string run_id = 1 [(buf.validate.field).string.uuid = true];
  acme.example.v2.[TimeWindow](../../v2/messages/TimeWindow.md) window = 2;
  repeated acme.example.v2.[FilterExpression](../../v2/messages/FilterExpression.md) filters = 3;
}
```

