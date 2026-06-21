# Watch pipeline step completions

**RECEIVE** `pipelines~1runs`

```mermaid
sequenceDiagram
  participant Client
  participant Broker as pipelines~1runs
  Client->>Broker: receive (watchPipelineSteps)
```

```yaml
action: receive
bindings:
  kafka:
    bindingVersion: 0.5.0
    groupId: pipeline-watchers
channel:
  $ref: "#/channels/pipelines~1runs"
messages:
- $ref: "#/channels/pipelines~1runs/messages/pipelineStepCompleted"
summary: Watch pipeline step completions
```

