# HealthStatus

HealthStatus is reported by gateway health RPCs.

```protobuf
enum [HealthStatus](HealthStatus.md) {
  HEALTH_STATUS_UNSPECIFIED = 0;
  HEALTH_STATUS_SERVING = 1;
  HEALTH_STATUS_NOT_SERVING = 2;
  HEALTH_STATUS_DEGRADED = 3;
}
```

