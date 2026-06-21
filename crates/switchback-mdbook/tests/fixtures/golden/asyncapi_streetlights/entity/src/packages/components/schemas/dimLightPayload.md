# dimLightPayload

#### Properties

| Field | Type |
| --- | --- |
| `sentAt` | [sentAt](sentAt.md) |

```yaml
properties:
  percentage:
    description: Percentage to which the light should be dimmed to.
    maximum: 100
    minimum: 0
    type: integer
  sentAt:
    $ref: "#/components/schemas/sentAt"
type: object
```

