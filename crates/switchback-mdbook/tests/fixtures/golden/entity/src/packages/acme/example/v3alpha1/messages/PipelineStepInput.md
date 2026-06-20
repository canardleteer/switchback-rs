# PipelineStepInput

PipelineStepInput is one unit of work inside a pipeline run.

```protobuf
message [PipelineStepInput](PipelineStepInput.md) {
  string step_name = 1 [
      (buf.validate.field).required = true,
      (buf.validate.field).string.min_len = 1,
      (buf.validate.field).string.max_len = 64
    ];
  acme.example.v2.[PayloadEnvelope](../../v2/messages/PayloadEnvelope.md) input = 2;
  [RolloutStage](RolloutStage.md) rollout = 3;
}
```

