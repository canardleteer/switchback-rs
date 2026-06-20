# RelayAck

RelayAck confirms a relay session is ready.

```protobuf
message [RelayAck](RelayAck.md) {
  string session_id = 1;
  google.protobuf.Timestamp opened_at = 2;
  acme.example.v2.[AggregateHealth](../../v2/messages/AggregateHealth.md) health = 3;
}
```

