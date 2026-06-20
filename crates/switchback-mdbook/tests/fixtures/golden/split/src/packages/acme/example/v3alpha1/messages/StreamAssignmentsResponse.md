# StreamAssignmentsResponse

StreamAssignmentsResponse is one assignment event on the stream.

```protobuf
message [StreamAssignmentsResponse](StreamAssignmentsResponse.md) {
  [ExperimentAssignment](ExperimentAssignment.md) assignment = 1;
  acme.example.v2.[StreamCursor](../../v2/messages/StreamCursor.md) cursor = 2;
  google.protobuf.Timestamp observed_at = 3;
}
```

