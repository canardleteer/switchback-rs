# CancelPipelineRequest

CancelPipelineRequest stops a run by id.

```protobuf
message [CancelPipelineRequest](CancelPipelineRequest.md) {
  string run_id = 1 [(buf.validate.field).string.uuid = true];
  string reason = 2 [(buf.validate.field).string.max_len = 512];
}
```

