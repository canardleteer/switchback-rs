# BatchRequest

BatchRequest aggregates many items for BatchEcho RPCs.

```protobuf
message [BatchRequest](BatchRequest.md) {
  repeated [BatchItem](BatchItem.md) items = 1 [(buf.validate.field).repeated.min_items = 1];
  [ResourceIdentity](ResourceIdentity.md) identity = 2;
  bool partial_failure_allowed = 3;
}
```

