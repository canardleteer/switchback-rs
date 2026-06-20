# AuditBatch

AuditBatch groups records for batch RPC demonstrations.

```protobuf
message [AuditBatch](AuditBatch.md) {
  repeated [AuditRecord](AuditRecord.md) records = 1;
  string batch_id = 2;
}
```

