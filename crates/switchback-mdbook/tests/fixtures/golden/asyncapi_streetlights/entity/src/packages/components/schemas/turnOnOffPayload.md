# turnOnOffPayload

#### Properties

| Field | Type |
| --- | --- |
| `sentAt` | [sentAt](sentAt.md) |

```yaml
properties:
  command:
    description: Whether to turn on or off the light.
    enum:
    - true
    - false
    type: string
  sentAt:
    $ref: "#/components/schemas/sentAt"
type: object
```

