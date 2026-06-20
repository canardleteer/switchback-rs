# StreamAuditRecordsRequest

StreamAuditRecordsRequest opens a server stream of audit rows.

```protobuf
message [StreamAuditRecordsRequest](StreamAuditRecordsRequest.md) {
  [TenantRef](TenantRef.md) tenant = 1;
  repeated [FilterExpression](FilterExpression.md) filters = 2;
}
```

