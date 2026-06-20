# BatchItemResult

BatchItemResult reports per-item outcomes.

```protobuf
message [BatchItemResult](BatchItemResult.md) {
  [BatchKey](BatchKey.md) key = 1;
  bool ok = 2;
  [ErrorDetail](ErrorDetail.md) error = 3;
  [PayloadEnvelope](PayloadEnvelope.md) response_envelope = 4;
}
```

