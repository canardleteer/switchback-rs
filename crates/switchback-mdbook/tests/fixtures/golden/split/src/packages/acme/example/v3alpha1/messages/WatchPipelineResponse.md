# WatchPipelineResponse

WatchPipelineResponse is one event on the WatchPipeline server stream.

```protobuf
message [WatchPipelineResponse](WatchPipelineResponse.md) {
  string event_id = 1;
  [PipelineStepResult](PipelineStepResult.md) step = 2;
  google.protobuf.Timestamp observed_at = 3;
  acme.example.v2.[StreamCursor](../../v2/messages/StreamCursor.md) cursor = 4;
}
```

