# StartPipelineRequest

StartPipelineRequest kicks off a pipeline from staged inputs.

```protobuf
message [StartPipelineRequest](StartPipelineRequest.md) {
  string pipeline_name = 1 [
      (buf.validate.field).required = true,
      (buf.validate.field).string.pattern = "^[a-z][a-z0-9-]*$"
    ];
  repeated [PipelineStage](PipelineStage.md) stages = 2 [(buf.validate.field).repeated.min_items = 1];
  acme.example.v2.[ResourceIdentity](../../v2/messages/ResourceIdentity.md) actor = 3;
}
```

