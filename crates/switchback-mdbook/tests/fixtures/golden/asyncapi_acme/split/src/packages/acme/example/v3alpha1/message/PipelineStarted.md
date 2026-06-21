# PipelineStarted

```yaml
name: PipelineStarted
payload:
  schema:
    fields:
    - name: run_id
      type: string
    - name: pipeline_name
      type: string
    - name: started_at
      type: string
    name: PipelineStarted
    namespace: acme.events.v3alpha1
    type: record
  schemaFormat: application/vnd.apache.avro+json
title: Pipeline started (Avro)
```

