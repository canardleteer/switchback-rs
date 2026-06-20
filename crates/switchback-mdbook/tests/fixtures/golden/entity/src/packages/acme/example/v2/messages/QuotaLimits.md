# QuotaLimits

QuotaLimits documents soft limits referenced from gateway RPCs.

```protobuf
message [QuotaLimits](QuotaLimits.md) {
  uint32 max_requests_per_minute = 1;
  uint32 max_stream_duration_seconds = 2;
  uint64 max_payload_bytes = 3;
}
```

