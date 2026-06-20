# StreamAuditRecordsResponse

StreamAuditRecordsResponse is one audit row on the stream.

```protobuf
message [StreamAuditRecordsResponse](StreamAuditRecordsResponse.md) {
  [AuditRecord](AuditRecord.md) record = 1;
  [StreamCursor](StreamCursor.md) cursor = 2;
}
```

