# Stream echo chunks

**SUBSCRIBE** `echo/stream` — `kafka` topic `acme.echo.stream`

```mermaid
sequenceDiagram
  participant Client
  participant Broker as echo/stream
  Client->>Broker: subscribe (subscribeEchoStream)
```

#### Messages

- [EchoStreamChunk](../message/EchoStreamChunk.md)

```yaml
message:
  $ref: "#/components/messages/EchoStreamChunk"
operationId: subscribeEchoStream
summary: Stream echo chunks
```

