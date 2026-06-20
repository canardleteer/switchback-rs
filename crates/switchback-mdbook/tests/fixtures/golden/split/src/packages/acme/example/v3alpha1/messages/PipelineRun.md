# PipelineRun

PipelineRun identifies a long-lived orchestration instance.

```protobuf
message [PipelineRun](PipelineRun.md) {
  
  string run_id = 1 [
      (buf.validate.field).required = true,
      (buf.validate.field).string.uuid = true
    ];
  string pipeline_name = 2 [(buf.validate.field).string.min_len = 1];
  [PipelineStatus](../enums/PipelineStatus.md) status = 3;
  repeated [PipelineStepResult](PipelineStepResult.md) results = 4;
  acme.example.v2.[ResourceIdentity](../../v2/messages/ResourceIdentity.md) owner = 5;
  google.protobuf.Timestamp started_at = 6;
  google.protobuf.Timestamp completed_at = 7;
}
```

**Protovalidate (CEL)**

```cel
id: "pipeline_run.completed_after_started"
      message: "completed_at must not precede started_at when both are set"
      expression: "!has(this.completed_at) || !has(this.started_at) || this.completed_at >= this.started_at"
```

