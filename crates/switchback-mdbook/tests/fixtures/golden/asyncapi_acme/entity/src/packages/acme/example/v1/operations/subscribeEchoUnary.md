# Receive echo response

**SUBSCRIBE** `echo/unary` — QoS 1 · `kafka` topic `acme.echo.unary`

```mermaid
sequenceDiagram
  participant Client
  participant Broker as echo/unary
  Client->>Broker: subscribe (subscribeEchoUnary)
```

#### Messages

- [EchoUnaryResponse](../message/EchoUnaryResponse.md)

```yaml
message:
  $ref: "#/components/messages/EchoUnaryResponse"
operationId: subscribeEchoUnary
summary: Receive echo response
```

