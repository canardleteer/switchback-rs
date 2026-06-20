# EchoHistoryEntry

EchoHistoryEntry is one row in a fictional audit trail.

```protobuf
message [EchoHistoryEntry](EchoHistoryEntry.md) {
  google.protobuf.Timestamp at = 1;
  string rpc_name = 2;
  [EchoUnaryRequest](EchoUnaryRequest.md) unary_request = 3;
  acme.example.v2.[ErrorDetail](../../v2/messages/ErrorDetail.md) error = 4;
}
```

