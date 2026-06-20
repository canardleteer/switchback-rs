# PipelineService

PipelineService documents orchestration RPCs in the alpha package.

**StartPipeline** ( [StartPipelineRequest](../messages/StartPipelineRequest.md) ) returns ( [StartPipelineResponse](../messages/StartPipelineResponse.md) )

StartPipeline creates a pipeline run from staged inputs.

```protobuf
rpc StartPipeline (acme.example.v3alpha1.[StartPipelineRequest](../messages/StartPipelineRequest.md)) returns (acme.example.v3alpha1.[StartPipelineResponse](../messages/StartPipelineResponse.md));
```

**WatchPipeline** ( [WatchPipelineRequest](../messages/WatchPipelineRequest.md) ) returns ( [WatchPipelineResponse](../messages/WatchPipelineResponse.md) )

WatchPipeline streams step results for a run.

```protobuf
rpc WatchPipeline (acme.example.v3alpha1.[WatchPipelineRequest](../messages/WatchPipelineRequest.md)) returns (stream acme.example.v3alpha1.[WatchPipelineResponse](../messages/WatchPipelineResponse.md));
```

**CancelPipeline** ( [CancelPipelineRequest](../messages/CancelPipelineRequest.md) ) returns ( [CancelPipelineResponse](../messages/CancelPipelineResponse.md) )

CancelPipeline stops a run by identifier.

```protobuf
rpc CancelPipeline (acme.example.v3alpha1.[CancelPipelineRequest](../messages/CancelPipelineRequest.md)) returns (acme.example.v3alpha1.[CancelPipelineResponse](../messages/CancelPipelineResponse.md));
```

