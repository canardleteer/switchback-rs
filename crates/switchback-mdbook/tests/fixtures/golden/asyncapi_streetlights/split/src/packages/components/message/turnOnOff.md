# turnOnOff

Command a particular streetlight to turn the lights on or off.

#### Payload

- [turnOnOffPayload](../schemas/turnOnOffPayload.md)
- `commonHeaders`

```yaml
name: turnOnOff
payload:
  $ref: "#/components/schemas/turnOnOffPayload"
summary: Command a particular streetlight to turn the lights on or off.
title: Turn on/off
traits:
- $ref: "#/components/messageTraits/commonHeaders"
```

