# WatchInventoryRequest

WatchInventoryRequest subscribes to inventory change events.

```protobuf
message [WatchInventoryRequest](WatchInventoryRequest.md) {
  string warehouse_id = 1 [
      (buf.validate.field).required = true,
      (buf.validate.field).string.min_len = 1,
      (buf.validate.field).string.max_len = 128
    ];
  [TimeWindow](TimeWindow.md) window = 2;
  repeated [FilterExpression](FilterExpression.md) filters = 3;
}
```

