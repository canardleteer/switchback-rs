# ListEchoHistoryResponse

ListEchoHistoryResponse returns history rows.

```protobuf
message [ListEchoHistoryResponse](ListEchoHistoryResponse.md) {
  repeated [EchoHistoryEntry](EchoHistoryEntry.md) entries = 1;
  acme.example.v2.[PageResult](../../v2/messages/PageResult.md) page = 2;
}
```

