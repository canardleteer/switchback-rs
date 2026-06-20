# SharedMetadata

SharedMetadata is referenced from v1 echo and gateway messages.

 Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque
 laudantium. Keep `trace_id` opaque to callers; format is service-specific.

```protobuf
message [SharedMetadata](SharedMetadata.md) {
// Distributed trace id (W3C `traceparent` or equivalent).
  string trace_id = 1 [(buf.validate.field).string.min_len = 1];
// Optional parent span for nested calls.
  string parent_span_id = 2;
// When the trace context was minted.
  google.protobuf.Timestamp created_at = 3;
// Arbitrary baggage for examples (not a production pattern).
  map<string, string> baggage = 4;
// Classification for routing in docs.
  [SharedKind](../enums/SharedKind.md) kind = 5;
}
```

