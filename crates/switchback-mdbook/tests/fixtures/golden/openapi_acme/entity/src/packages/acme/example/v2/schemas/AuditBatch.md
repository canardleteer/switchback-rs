# AuditBatch

```yaml
properties:
  batch_id:
    type: string
  records:
    items:
      $ref: "#/components/schemas/AuditRecord"
    type: array
type: object
```

