# dimLight

Command a particular streetlight to dim the lights.

#### Payload

- [dimLightPayload](../schemas/dimLightPayload.md)
- `commonHeaders`

```yaml
name: dimLight
payload:
  $ref: "#/components/schemas/dimLightPayload"
summary: Command a particular streetlight to dim the lights.
title: Dim light
traits:
- $ref: "#/components/messageTraits/commonHeaders"
```

