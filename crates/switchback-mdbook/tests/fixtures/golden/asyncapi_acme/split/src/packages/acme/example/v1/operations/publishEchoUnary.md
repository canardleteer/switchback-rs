# Publish echo request

**PUBLISH** `echo/unary` — QoS 1 · `kafka` topic `acme.echo.unary`

```mermaid
sequenceDiagram
  participant Client
  participant Broker as echo/unary
  Client->>Broker: publish (publishEchoUnary)
```

#### Messages

- [EchoUnaryRequest](../message/EchoUnaryRequest.md)

```yaml
message:
  $ref: "#/components/messages/EchoUnaryRequest"
operationId: publishEchoUnary
summary: Publish echo request
```

