# PublishEventsResponse

PublishEventsResponse acknowledges a published event stream.

```protobuf
message [PublishEventsResponse](PublishEventsResponse.md) {
  string event_id = 1;
  acme.example.v2.[StreamCursor](../../v2/messages/StreamCursor.md) cursor = 2;
}
```

