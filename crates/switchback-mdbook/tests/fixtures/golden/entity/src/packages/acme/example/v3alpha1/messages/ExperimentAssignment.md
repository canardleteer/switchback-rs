# ExperimentAssignment

ExperimentAssignment records which arm a subject received.

```protobuf
message [ExperimentAssignment](ExperimentAssignment.md) {
  string experiment_id = 1;
  string subject_id = 2 [(buf.validate.field).string.min_len = 1];
  string arm_id = 3;
  google.protobuf.Timestamp assigned_at = 4;
  acme.example.v2.[TraceContext](../../v2/messages/TraceContext.md) trace = 5;
}
```

