# PlatformService

PlatformService documents cross-cutting telemetry and health RPCs.

**ExportAuditBatch** ( [ExportAuditBatchRequest](../messages/ExportAuditBatchRequest.md) ) returns ( [ExportAuditBatchResponse](../messages/ExportAuditBatchResponse.md) )

ExportAuditBatch returns a snapshot of audit records (unary).

```protobuf
rpc ExportAuditBatch (acme.example.v2.[ExportAuditBatchRequest](../messages/ExportAuditBatchRequest.md)) returns (acme.example.v2.[ExportAuditBatchResponse](../messages/ExportAuditBatchResponse.md));
```

**StreamAuditRecords** ( [StreamAuditRecordsRequest](../messages/StreamAuditRecordsRequest.md) ) returns ( [StreamAuditRecordsResponse](../messages/StreamAuditRecordsResponse.md) )

StreamAuditRecords pushes audit rows over a server stream.

```protobuf
rpc StreamAuditRecords (acme.example.v2.[StreamAuditRecordsRequest](../messages/StreamAuditRecordsRequest.md)) returns (stream acme.example.v2.[StreamAuditRecordsResponse](../messages/StreamAuditRecordsResponse.md));
```

**IngestTelemetry** ( [IngestTelemetryRequest](../messages/IngestTelemetryRequest.md) ) returns ( [IngestTelemetryResponse](../messages/IngestTelemetryResponse.md) )

IngestTelemetry accepts a client stream of metric samples.

```protobuf
rpc IngestTelemetry (stream acme.example.v2.[IngestTelemetryRequest](../messages/IngestTelemetryRequest.md)) returns (acme.example.v2.[IngestTelemetryResponse](../messages/IngestTelemetryResponse.md));
```

**GetAggregateHealth** ( [GetAggregateHealthRequest](../messages/GetAggregateHealthRequest.md) ) returns ( [GetAggregateHealthResponse](../messages/GetAggregateHealthResponse.md) )

GetAggregateHealth returns synthetic component health.

```protobuf
rpc GetAggregateHealth (acme.example.v2.[GetAggregateHealthRequest](../messages/GetAggregateHealthRequest.md)) returns (acme.example.v2.[GetAggregateHealthResponse](../messages/GetAggregateHealthResponse.md));
```

