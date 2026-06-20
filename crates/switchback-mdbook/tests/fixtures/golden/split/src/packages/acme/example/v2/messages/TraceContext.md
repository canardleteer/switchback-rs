# TraceContext

TraceContext duplicates some metadata for nested RPC examples.

```protobuf
message [TraceContext](TraceContext.md) {
  string trace_id = 1;
  string span_id = 2;
  google.protobuf.Duration sampling_delay = 3;
}
```

