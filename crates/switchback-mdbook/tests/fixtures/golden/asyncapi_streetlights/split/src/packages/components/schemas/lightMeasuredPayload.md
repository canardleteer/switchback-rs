# lightMeasuredPayload

#### Properties

| Field | Type |
| --- | --- |
| `sentAt` | [sentAt](sentAt.md) |

```yaml
properties:
  lumens:
    description: Light intensity measured in lumens.
    minimum: 0
    type: integer
  sentAt:
    $ref: "#/components/schemas/sentAt"
type: object
```

