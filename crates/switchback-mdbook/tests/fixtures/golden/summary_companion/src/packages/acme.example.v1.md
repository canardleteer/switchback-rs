# acme.example.v1

## Services

### EchoService

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

**EchoUnary** ( [EchoUnaryRequest](#echounaryrequest) ) returns ( [EchoUnaryResponse](#echounaryresponse) )

```protobuf
rpc EchoUnary (acme.example.v1.[EchoUnaryRequest](#echounaryrequest)) returns (acme.example.v1.[EchoUnaryResponse](#echounaryresponse));
option idempotency_level = NO_SIDE_EFFECTS;
```

EchoUnary is the hello-world RPC for this fixture.

 | Direction | Type |
 |-----------|------|
 | request | EchoUnaryRequest |
 | response | EchoUnaryResponse |

 hello-world RPC — leading comment preserved for mdBook heading link tests.

**EchoServerStream** ( [EchoServerStreamRequest](#echoserverstreamrequest) ) returns ( [EchoServerStreamResponse](#echoserverstreamresponse) )

```protobuf
rpc EchoServerStream (acme.example.v1.[EchoServerStreamRequest](#echoserverstreamrequest)) returns (stream acme.example.v1.[EchoServerStreamResponse](#echoserverstreamresponse));
```

EchoServerStream demonstrates server streaming in generated docs.

**EchoClientStream** ( [EchoClientStreamRequest](#echoclientstreamrequest) ) returns ( [EchoClientStreamResponse](#echoclientstreamresponse) )

```protobuf
rpc EchoClientStream (stream acme.example.v1.[EchoClientStreamRequest](#echoclientstreamrequest)) returns (acme.example.v1.[EchoClientStreamResponse](#echoclientstreamresponse));
```

EchoClientStream demonstrates client streaming uploads.

**EchoBidiStream** ( [EchoBidiStreamRequest](#echobidistreamrequest) ) returns ( [EchoBidiStreamResponse](#echobidistreamresponse) )

```protobuf
rpc EchoBidiStream (stream acme.example.v1.[EchoBidiStreamRequest](#echobidistreamrequest)) returns (stream acme.example.v1.[EchoBidiStreamResponse](#echobidistreamresponse));
```

EchoBidiStream demonstrates bidirectional streaming.

**BatchEcho** ( [BatchEchoRequest](#batchechorequest) ) returns ( [BatchEchoResponse](#batchechoresponse) )

```protobuf
rpc BatchEcho (acme.example.v1.[BatchEchoRequest](#batchechorequest)) returns (acme.example.v1.[BatchEchoResponse](#batchechoresponse));
```

BatchEcho sends many envelopes in one unary call.

 Response type BatchEchoResponse is defined in this package; payload uses v2 batch types.

**WatchEcho** ( [WatchEchoRequest](#watchechorequest) ) returns ( [WatchEchoResponse](#watchechoresponse) )

```protobuf
rpc WatchEcho (acme.example.v1.[WatchEchoRequest](#watchechorequest)) returns (stream acme.example.v1.[WatchEchoResponse](#watchechoresponse));
```

WatchEcho pushes events for a subscription (server streaming).

### GatewayService

GatewayService focuses on streaming patterns and catalog adjacency.

**RelayConnect** ( [RelayConnectRequest](#relayconnectrequest) ) returns ( [RelayConnectResponse](#relayconnectresponse) )

```protobuf
rpc RelayConnect (stream acme.example.v1.[RelayConnectRequest](#relayconnectrequest)) returns (stream acme.example.v1.[RelayConnectResponse](#relayconnectresponse));
```

RelayConnect opens a bidirectional stream for frame exchange.

**PublishEvents** ( [PublishEventsRequest](#publisheventsrequest) ) returns ( [PublishEventsResponse](#publisheventsresponse) )

```protobuf
rpc PublishEvents (stream acme.example.v1.[PublishEventsRequest](#publisheventsrequest)) returns (acme.example.v1.[PublishEventsResponse](#publisheventsresponse));
```

PublishEvents accepts a client stream of PublishEventsRequest messages.

**ListCatalogProducts** ( [ListCatalogProductsRequest](#listcatalogproductsrequest) ) returns ( [ListCatalogProductsResponse](#listcatalogproductsresponse) )

```protobuf
rpc ListCatalogProducts (acme.example.v1.[ListCatalogProductsRequest](#listcatalogproductsrequest)) returns (acme.example.v1.[ListCatalogProductsResponse](#listcatalogproductsresponse));
```

ListCatalogProducts bridges to v2 catalog types for cross-package links.

### AdminService

AdminService documents infrequent control-plane RPCs.

**RotateKeys** ( [RotateKeysRequest](#rotatekeysrequest) ) returns ( [RotateKeysResponse](#rotatekeysresponse) )

```protobuf
rpc RotateKeys (acme.example.v1.[RotateKeysRequest](#rotatekeysrequest)) returns (acme.example.v1.[RotateKeysResponse](#rotatekeysresponse));
```

RotateKeys starts a synthetic long-running operation.

## Messages and enums

### EchoUnaryRequest

EchoUnaryRequest carries the unary payload.

 Fields reference `[SharedMetadata](acme.example.v2.md#sharedmetadata)` for trace identifiers.
 See also `[BatchRequest](acme.example.v2.md#batchrequest)` for multi-item batch uploads.

```protobuf
message [EchoUnaryRequest](#echounaryrequest) {
// User-visible text to echo back.
// 
//  Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
//  commodo consequat.
  string message = 1;
  acme.example.v2.[SharedMetadata](acme.example.v2.md#sharedmetadata) metadata = 2;
  oneof _locale {
    optional string locale = 3;
  }
  repeated string tags = 4;
  acme.example.v2.[EchoExtension](acme.example.v2.md#echoextension) extension = 5;
}
```

### EchoUnaryResponse

EchoUnaryResponse returns the echoed text.

```protobuf
message [EchoUnaryResponse](#echounaryresponse) {
  string message = 1;
  google.protobuf.Timestamp echoed_at = 2;
  acme.example.v2.[SharedMetadata](acme.example.v2.md#sharedmetadata) metadata = 3;
}
```

### EchoServerStreamRequest

EchoServerStreamRequest opens a server-streaming RPC.

```protobuf
message [EchoServerStreamRequest](#echoserverstreamrequest) {
  string message = 1;
  uint32 chunk_count = 2;
  google.protobuf.Duration inter_chunk_delay = 3;
  acme.example.v2.[SharedMetadata](acme.example.v2.md#sharedmetadata) metadata = 4;
}
```

### EchoServerStreamResponse

EchoServerStreamResponse is one chunk in a server stream.

```protobuf
message [EchoServerStreamResponse](#echoserverstreamresponse) {
  string message = 1;
  acme.example.v2.[StreamCursor](acme.example.v2.md#streamcursor) cursor = 2;
  uint32 index = 3;
}
```

### EchoClientStreamRequest

EchoClientStreamRequest is one chunk in a client stream upload.

```protobuf
message [EchoClientStreamRequest](#echoclientstreamrequest) {
  string message = 1;
  uint32 part_index = 2;
  bool last_part = 3;
}
```

### EchoClientStreamResponse

EchoClientStreamResponse aggregates a client stream on the server.

```protobuf
message [EchoClientStreamResponse](#echoclientstreamresponse) {
  string joined_message = 1;
  uint32 parts_received = 2;
  google.protobuf.Timestamp completed_at = 3;
}
```

### EchoBidiStreamRequest

EchoBidiStreamRequest is one frame in a bidirectional echo session.

```protobuf
message [EchoBidiStreamRequest](#echobidistreamrequest) {
  string message = 1;
  uint64 sequence = 2;
  bool fin = 3;
}
```

### EchoBidiStreamResponse

EchoBidiStreamResponse mirrors a bidirectional frame back to the client.

```protobuf
message [EchoBidiStreamResponse](#echobidistreamresponse) {
  string message = 1;
  uint64 sequence = 2;
  bool fin = 3;
  acme.example.v2.[StreamCursor](acme.example.v2.md#streamcursor) cursor = 4;
}
```

### WatchEchoRequest

WatchEchoRequest subscribes to echo events for a topic.

```protobuf
message [WatchEchoRequest](#watchechorequest) {
  string topic = 1;
  acme.example.v2.[TimeWindow](acme.example.v2.md#timewindow) window = 2;
  repeated acme.example.v2.[FilterExpression](acme.example.v2.md#filterexpression) filters = 3;
}
```

### WatchEchoResponse

WatchEchoResponse is pushed on the WatchEcho server stream.

```protobuf
message [WatchEchoResponse](#watchechoresponse) {
  string event_id = 1;
  [EchoUnaryResponse](#echounaryresponse) payload = 2;
  google.protobuf.Timestamp observed_at = 3;
}
```

### BatchEchoRequest

BatchEchoRequest wraps a v2 batch payload for STANDARD RPC naming.

```protobuf
message [BatchEchoRequest](#batchechorequest) {
  acme.example.v2.[BatchRequest](acme.example.v2.md#batchrequest) batch = 1;
}
```

### BatchEchoResponse

BatchEchoResponse wraps a v2 batch result for STANDARD RPC naming.

```protobuf
message [BatchEchoResponse](#batchechoresponse) {
  acme.example.v2.[BatchResponse](acme.example.v2.md#batchresponse) batch = 1;
}
```

### LegacyEchoNote

LegacyEchoNote is an ancillary message for cross-link examples.

```protobuf
message [LegacyEchoNote](#legacyechonote) {
  string note_id = 1;
  string body = 2;
  acme.example.v2.[SeeAlsoBlock](acme.example.v2.md#seealsoblock) see_also = 3;
}
```

### EchoConfiguration

EchoConfiguration captures static config referenced in comments only.

```protobuf
message [EchoConfiguration](#echoconfiguration) {
  uint32 default_chunk_size = 1;
  google.protobuf.Duration default_timeout = 2;
  acme.example.v2.[RetryPolicy](acme.example.v2.md#retrypolicy) retry_policy = 3;
}
```

### EchoConfigurationRequest

EchoConfigurationRequest fetches EchoConfiguration.

```protobuf
message [EchoConfigurationRequest](#echoconfigurationrequest) {
  acme.example.v2.[TenantRef](acme.example.v2.md#tenantref) tenant = 1;
}
```

### EchoConfigurationResponse

EchoConfigurationResponse returns EchoConfiguration.

```protobuf
message [EchoConfigurationResponse](#echoconfigurationresponse) {
  [EchoConfiguration](#echoconfiguration) config = 1;
}
```

### DocumentedEchoPair

DocumentedEchoPair ties request/response for See Also blocks.

```protobuf
message [DocumentedEchoPair](#documentedechopair) {
  [EchoUnaryRequest](#echounaryrequest) request = 1;
  [EchoUnaryResponse](#echounaryresponse) response = 2;
}
```

### EchoHistoryEntry

EchoHistoryEntry is one row in a fictional audit trail.

```protobuf
message [EchoHistoryEntry](#echohistoryentry) {
  google.protobuf.Timestamp at = 1;
  string rpc_name = 2;
  [EchoUnaryRequest](#echounaryrequest) unary_request = 3;
  acme.example.v2.[ErrorDetail](acme.example.v2.md#errordetail) error = 4;
}
```

### ListEchoHistoryRequest

ListEchoHistoryRequest lists EchoHistoryEntry rows.

```protobuf
message [ListEchoHistoryRequest](#listechohistoryrequest) {
  acme.example.v2.[ListOptions](acme.example.v2.md#listoptions) options = 1;
  acme.example.v2.[ResourceIdentity](acme.example.v2.md#resourceidentity) identity = 2;
}
```

### ListEchoHistoryResponse

ListEchoHistoryResponse returns history rows.

```protobuf
message [ListEchoHistoryResponse](#listechohistoryresponse) {
  repeated [EchoHistoryEntry](#echohistoryentry) entries = 1;
  acme.example.v2.[PageResult](acme.example.v2.md#pageresult) page = 2;
}
```

### RelayFrame

RelayFrame is one unit in a bidirectional relay stream.

```protobuf
message [RelayFrame](#relayframe) {
  uint64 sequence = 1;
  bytes payload = 2;
  acme.example.v2.[SharedMetadata](acme.example.v2.md#sharedmetadata) metadata = 3;
  bool fin = 4;
}
```

### RelayOpen

RelayOpen carries handshake metadata for RelayConnect.

```protobuf
message [RelayOpen](#relayopen) {
  string session_name = 1;
  acme.example.v2.[ResourceIdentity](acme.example.v2.md#resourceidentity) identity = 2;
  acme.example.v2.[QuotaLimits](acme.example.v2.md#quotalimits) requested_limits = 3;
  repeated acme.example.v2.[Label](acme.example.v2.md#label) labels = 4;
}
```

### RelayAck

RelayAck confirms a relay session is ready.

```protobuf
message [RelayAck](#relayack) {
  string session_id = 1;
  google.protobuf.Timestamp opened_at = 2;
  acme.example.v2.[AggregateHealth](acme.example.v2.md#aggregatehealth) health = 3;
}
```

### RelayClose

RelayClose ends a relay session gracefully.

```protobuf
message [RelayClose](#relayclose) {
  string session_id = 1;
  string reason = 2;
}
```

### RelayConnectRequest

RelayConnectRequest is one client-to-server frame in RelayConnect.

```protobuf
message [RelayConnectRequest](#relayconnectrequest) {
  oneof payload {
      [RelayOpen](#relayopen) open = 1;
      [RelayFrame](#relayframe) frame = 2;
      [RelayClose](#relayclose) close = 3;
    }
}
```

### RelayConnectResponse

RelayConnectResponse is one server-to-client frame in RelayConnect.

```protobuf
message [RelayConnectResponse](#relayconnectresponse) {
  oneof payload {
      [RelayAck](#relayack) ack = 1;
      [RelayFrame](#relayframe) frame = 2;
      [RelayClose](#relayclose) close = 3;
    }
}
```

### PublishEventsRequest

PublishEventsRequest is one client-streaming upload chunk.

```protobuf
message [PublishEventsRequest](#publisheventsrequest) {
  string event_type = 1;
  acme.example.v2.[PayloadEnvelope](acme.example.v2.md#payloadenvelope) envelope = 2;
  google.protobuf.Timestamp client_time = 3;
}
```

### PublishEventsResponse

PublishEventsResponse acknowledges a published event stream.

```protobuf
message [PublishEventsResponse](#publisheventsresponse) {
  string event_id = 1;
  acme.example.v2.[StreamCursor](acme.example.v2.md#streamcursor) cursor = 2;
}
```

### ListCatalogProductsRequest

ListCatalogProductsRequest wraps the v2 catalog list request.

```protobuf
message [ListCatalogProductsRequest](#listcatalogproductsrequest) {
  acme.example.v2.[ListProductsRequest](acme.example.v2.md#listproductsrequest) request = 1;
}
```

### ListCatalogProductsResponse

ListCatalogProductsResponse wraps the v2 catalog list response.

```protobuf
message [ListCatalogProductsResponse](#listcatalogproductsresponse) {
  acme.example.v2.[ListProductsResponse](acme.example.v2.md#listproductsresponse) response = 1;
}
```

### RotateKeysRequest

RotateKeysRequest triggers a long-running operation example.

```protobuf
message [RotateKeysRequest](#rotatekeysrequest) {
  acme.example.v2.[TenantRef](acme.example.v2.md#tenantref) tenant = 1;
  bool dry_run = 2;
}
```

### RotateKeysResponse

RotateKeysResponse returns an operation handle.

```protobuf
message [RotateKeysResponse](#rotatekeysresponse) {
  acme.example.v2.[LongRunningOperation](acme.example.v2.md#longrunningoperation) operation = 1;
}
```

