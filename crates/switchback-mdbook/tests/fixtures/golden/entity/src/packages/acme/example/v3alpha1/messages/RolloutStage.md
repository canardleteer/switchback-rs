# RolloutStage

RolloutStage documents progressive delivery for pipeline stories.

```protobuf
message [RolloutStage](RolloutStage.md) {
  [ReleaseChannel](../enums/ReleaseChannel.md) channel = 1;
  google.protobuf.Duration bake_time = 2;
  uint32 max_parallel = 3 [(buf.validate.field).uint32.lte = 1000];
}
```

