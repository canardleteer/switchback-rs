# echo/stream

**channel** `echo/stream`

Server-sent echo chunks as an event stream.

```yaml
bindings:
  kafka:
    bindingVersion: 0.4.0
    partitions: 3
    topic: acme.echo.stream
description: Server-sent echo chunks as an event stream.
subscribe:
  message:
    $ref: "#/components/messages/EchoStreamChunk"
  operationId: subscribeEchoStream
  summary: Stream echo chunks
```

