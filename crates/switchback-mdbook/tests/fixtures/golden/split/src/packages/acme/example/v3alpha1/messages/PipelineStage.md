# PipelineStage

PipelineStage is a oneof-heavy stage definition for doc rendering.

```protobuf
message [PipelineStage](PipelineStage.md) {
  oneof stage {
      [PipelineStepInput](PipelineStepInput.md) step = 1;
      [RolloutStage](RolloutStage.md) rollout_only = 2;
      acme.example.v2.[AuditBatch](../../v2/messages/AuditBatch.md) audit_snapshot = 3;
    }
}
```

