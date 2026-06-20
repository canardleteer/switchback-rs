# ComponentHealth

ComponentHealth describes one sub-system in aggregate health.

```protobuf
message [ComponentHealth](ComponentHealth.md) {
  string component = 1;
  [HealthStatus](../enums/HealthStatus.md) status = 2;
  string detail = 3;
}
```

