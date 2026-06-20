# EchoService

EchoService exposes echo RPCs for documentation tests.

 ```mermaid
 sequenceDiagram
   participant C as Client
   participant E as EchoService
   C->>E: EchoUnary
   E-->>C: EchoUnaryResponse
   C->>E: EchoServerStream
   loop chunks
     E-->>C: EchoServerStreamResponse
   end
   C->>E: EchoBidiStream
   E-->>C: EchoBidiStreamResponse
 ```

**EchoUnary** ( [EchoUnaryRequest](../messages/EchoUnaryRequest.md) ) returns ( [EchoUnaryResponse](../messages/EchoUnaryResponse.md) )

EchoUnary is the hello-world RPC for this fixture.

 | Direction | Type |
 |-----------|------|
 | request | EchoUnaryRequest |
 | response | EchoUnaryResponse |

 hello-world RPC — leading comment preserved for mdBook heading link tests.

```protobuf
rpc EchoUnary (acme.example.v1.[EchoUnaryRequest](../messages/EchoUnaryRequest.md)) returns (acme.example.v1.[EchoUnaryResponse](../messages/EchoUnaryResponse.md));
option idempotency_level = NO_SIDE_EFFECTS;
```

**EchoServerStream** ( [EchoServerStreamRequest](../messages/EchoServerStreamRequest.md) ) returns ( [EchoServerStreamResponse](../messages/EchoServerStreamResponse.md) )

EchoServerStream demonstrates server streaming in generated docs.

```protobuf
rpc EchoServerStream (acme.example.v1.[EchoServerStreamRequest](../messages/EchoServerStreamRequest.md)) returns (stream acme.example.v1.[EchoServerStreamResponse](../messages/EchoServerStreamResponse.md));
```

**EchoClientStream** ( [EchoClientStreamRequest](../messages/EchoClientStreamRequest.md) ) returns ( [EchoClientStreamResponse](../messages/EchoClientStreamResponse.md) )

EchoClientStream demonstrates client streaming uploads.

```protobuf
rpc EchoClientStream (stream acme.example.v1.[EchoClientStreamRequest](../messages/EchoClientStreamRequest.md)) returns (acme.example.v1.[EchoClientStreamResponse](../messages/EchoClientStreamResponse.md));
```

**EchoBidiStream** ( [EchoBidiStreamRequest](../messages/EchoBidiStreamRequest.md) ) returns ( [EchoBidiStreamResponse](../messages/EchoBidiStreamResponse.md) )

EchoBidiStream demonstrates bidirectional streaming.

```protobuf
rpc EchoBidiStream (stream acme.example.v1.[EchoBidiStreamRequest](../messages/EchoBidiStreamRequest.md)) returns (stream acme.example.v1.[EchoBidiStreamResponse](../messages/EchoBidiStreamResponse.md));
```

**BatchEcho** ( [BatchEchoRequest](../messages/BatchEchoRequest.md) ) returns ( [BatchEchoResponse](../messages/BatchEchoResponse.md) )

BatchEcho sends many envelopes in one unary call.

 Response type BatchEchoResponse is defined in this package; payload uses v2 batch types.

```protobuf
rpc BatchEcho (acme.example.v1.[BatchEchoRequest](../messages/BatchEchoRequest.md)) returns (acme.example.v1.[BatchEchoResponse](../messages/BatchEchoResponse.md));
```

**WatchEcho** ( [WatchEchoRequest](../messages/WatchEchoRequest.md) ) returns ( [WatchEchoResponse](../messages/WatchEchoResponse.md) )

WatchEcho pushes events for a subscription (server streaming).

```protobuf
rpc WatchEcho (acme.example.v1.[WatchEchoRequest](../messages/WatchEchoRequest.md)) returns (stream acme.example.v1.[WatchEchoResponse](../messages/WatchEchoResponse.md));
```

