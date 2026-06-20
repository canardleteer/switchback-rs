# PipelineRun

```yaml
properties:
  completed_at:
    format: date-time
    type: string
  pipeline_name:
    type: string
  results:
    items:
      $ref: "#/components/schemas/PipelineStepResult"
    type: array
  run_id:
    format: uuid
    type: string
  started_at:
    format: date-time
    type: string
  status:
    $ref: "#/components/schemas/PipelineStatus"
type: object
```

