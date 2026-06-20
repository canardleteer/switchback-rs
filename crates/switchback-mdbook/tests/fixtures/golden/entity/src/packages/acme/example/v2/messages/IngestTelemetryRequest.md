# IngestTelemetryRequest

IngestTelemetryRequest is one client-streaming telemetry point.

```protobuf
message [IngestTelemetryRequest](IngestTelemetryRequest.md) {
  string metric_name = 1;
  double value = 2;
  google.protobuf.Timestamp observed_at = 3;
  [LabelSet](LabelSet.md) labels = 4;
}
```

