# pipelines/runs

**channel** `pipelines/runs`

Pipeline run lifecycle events.

```yaml
address: pipelines/runs
bindings:
  kafka:
    bindingVersion: 0.5.0
    partitions: 8
    topic: acme.pipelines.runs
description: Pipeline run lifecycle events.
messages:
  pipelineStarted:
    $ref: "#/components/messages/PipelineStarted"
  pipelineStepCompleted:
    $ref: "#/components/messages/PipelineStepCompleted"
```

