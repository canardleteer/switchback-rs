# AggregateHealth

AggregateHealth rolls up component statuses.

```protobuf
message [AggregateHealth](AggregateHealth.md) {
  [HealthStatus](../enums/HealthStatus.md) overall = 1;
  repeated [ComponentHealth](ComponentHealth.md) components = 2;
  google.protobuf.Timestamp evaluated_at = 3;
}
```

