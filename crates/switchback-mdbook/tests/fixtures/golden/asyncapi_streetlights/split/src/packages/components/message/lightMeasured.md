# lightMeasured

Inform about environmental lighting conditions of a particular streetlight.

#### Payload

- [lightMeasuredPayload](../schemas/lightMeasuredPayload.md)
- `commonHeaders`

```yaml
contentType: application/json
name: lightMeasured
payload:
  $ref: "#/components/schemas/lightMeasuredPayload"
summary: Inform about environmental lighting conditions of a particular streetlight.
title: Light measured
traits:
- $ref: "#/components/messageTraits/commonHeaders"
```

