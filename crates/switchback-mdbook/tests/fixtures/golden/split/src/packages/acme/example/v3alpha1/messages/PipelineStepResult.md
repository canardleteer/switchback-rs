# PipelineStepResult

PipelineStepResult reports completion for a single step.

```protobuf
message [PipelineStepResult](PipelineStepResult.md) {
  string step_name = 1;
  [PipelineStatus](../enums/PipelineStatus.md) status = 2;
  acme.example.v2.[ErrorDetail](../../v2/messages/ErrorDetail.md) error = 3;
  acme.example.v2.[StreamCursor](../../v2/messages/StreamCursor.md) cursor = 4;
  google.protobuf.Timestamp finished_at = 5;
}
```

