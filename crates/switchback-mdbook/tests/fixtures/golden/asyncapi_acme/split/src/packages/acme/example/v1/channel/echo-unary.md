# echo/unary

**channel** `echo/unary`

Echo unary request/response as events.

#### Messages

- [EchoUnaryRequest](../message/EchoUnaryRequest.md)
- [EchoUnaryResponse](../message/EchoUnaryResponse.md)

```yaml
bindings:
  kafka:
    bindingVersion: 0.4.0
    partitions: 6
    replicas: 3
    topic: acme.echo.unary
  mqtt:
    bindingVersion: 0.2.0
    qos: 1
    retain: false
description: Echo unary request/response as events.
publish:
  message:
    $ref: "#/components/messages/EchoUnaryRequest"
  operationId: publishEchoUnary
  summary: Publish echo request
subscribe:
  message:
    $ref: "#/components/messages/EchoUnaryResponse"
  operationId: subscribeEchoUnary
  summary: Receive echo response
```

