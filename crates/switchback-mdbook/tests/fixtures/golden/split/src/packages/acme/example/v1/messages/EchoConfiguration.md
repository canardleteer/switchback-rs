# EchoConfiguration

EchoConfiguration captures static config referenced in comments only.

```protobuf
message [EchoConfiguration](EchoConfiguration.md) {
  uint32 default_chunk_size = 1;
  google.protobuf.Duration default_timeout = 2;
  acme.example.v2.[RetryPolicy](../../v2/messages/RetryPolicy.md) retry_policy = 3;
}
```

