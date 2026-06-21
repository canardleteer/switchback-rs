# components

## Messages

### dimLight

Command a particular streetlight to dim the lights.

```yaml
name: dimLight
payload:
  $ref: "#/components/schemas/dimLightPayload"
summary: Command a particular streetlight to dim the lights.
title: Dim light
traits:
- $ref: "#/components/messageTraits/commonHeaders"
```

### lightMeasured

Inform about environmental lighting conditions of a particular streetlight.

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

### turnOnOff

Command a particular streetlight to turn the lights on or off.

```yaml
name: turnOnOff
payload:
  $ref: "#/components/schemas/turnOnOffPayload"
summary: Command a particular streetlight to turn the lights on or off.
title: Turn on/off
traits:
- $ref: "#/components/messageTraits/commonHeaders"
```

## Schemas

### dimLightPayload

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

### lightMeasuredPayload

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

### sentAt

Date and time when the message was sent.

```yaml
description: Date and time when the message was sent.
format: date-time
type: string
```

### turnOnOffPayload

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

