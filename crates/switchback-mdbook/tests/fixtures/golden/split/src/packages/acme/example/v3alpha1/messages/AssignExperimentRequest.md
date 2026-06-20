# AssignExperimentRequest

AssignExperimentRequest assigns a subject to an experiment arm.

```protobuf
message [AssignExperimentRequest](AssignExperimentRequest.md) {
  [ExperimentSpec](ExperimentSpec.md) spec = 1;
  [SubjectRef](SubjectRef.md) subject = 2;
  repeated string exclusion_keys = 3 [(buf.validate.field).repeated.max_items = 32];
}
```

