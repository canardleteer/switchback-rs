# ExportAuditBatchRequest

ExportAuditBatchRequest exports a batch of audit rows (unary).

```protobuf
message [ExportAuditBatchRequest](ExportAuditBatchRequest.md) {
  [ResourceIdentity](ResourceIdentity.md) identity = 1;
  [TimeWindow](TimeWindow.md) window = 2;
  [ListOptions](ListOptions.md) options = 3;
}
```

