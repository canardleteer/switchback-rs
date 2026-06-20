# StreamAssignmentsRequest

StreamAssignmentsRequest opens a server stream of assignment events.

```protobuf
message [StreamAssignmentsRequest](StreamAssignmentsRequest.md) {
  string experiment_id = 1 [(buf.validate.field).string.uuid = true];
  acme.example.v2.[TimeWindow](../../v2/messages/TimeWindow.md) window = 2;
}
```

