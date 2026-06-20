# RetryPolicy

RetryPolicy is embedded in long-running operation messages.

```protobuf
message [RetryPolicy](RetryPolicy.md) {
  uint32 max_attempts = 1;
  google.protobuf.Duration initial_backoff = 2;
  google.protobuf.Duration max_backoff = 3;
  double backoff_multiplier = 4;
}
```

