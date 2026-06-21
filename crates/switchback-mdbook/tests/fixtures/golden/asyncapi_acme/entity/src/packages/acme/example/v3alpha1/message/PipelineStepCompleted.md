# PipelineStepCompleted

#### Payload

- [PipelineStatus](../schemas/PipelineStatus.md)

#### Properties

| Field | Type |
| --- | --- |
| `status` | [PipelineStatus](../schemas/PipelineStatus.md) |

```yaml
name: PipelineStepCompleted
payload:
  schema:
    fields:
    - name: run_id
      type: string
    - name: step_name
      type: string
    - name: status
      type:
        name: PipelineStatus
        symbols:
        - queued
        - running
        - succeeded
        - failed
        - cancelled
        type: enum
    name: PipelineStepCompleted
    namespace: acme.events.v3alpha1
    type: record
  schemaFormat: application/vnd.apache.avro+json
title: Pipeline step completed (Avro)
```

