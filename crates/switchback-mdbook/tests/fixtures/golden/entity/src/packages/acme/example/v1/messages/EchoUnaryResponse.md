# EchoUnaryResponse

EchoUnaryResponse returns the echoed text.

```protobuf
message [EchoUnaryResponse](EchoUnaryResponse.md) {
  string message = 1;
  google.protobuf.Timestamp echoed_at = 2;
  acme.example.v2.[SharedMetadata](../../v2/messages/SharedMetadata.md) metadata = 3;
}
```

