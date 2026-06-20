# PublishEventsRequest

PublishEventsRequest is one client-streaming upload chunk.

```protobuf
message [PublishEventsRequest](PublishEventsRequest.md) {
  string event_type = 1;
  acme.example.v2.[PayloadEnvelope](../../v2/messages/PayloadEnvelope.md) envelope = 2;
  google.protobuf.Timestamp client_time = 3;
}
```

