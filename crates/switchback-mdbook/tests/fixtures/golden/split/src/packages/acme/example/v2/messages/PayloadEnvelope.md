# PayloadEnvelope

PayloadEnvelope wraps opaque bytes with metadata.

```protobuf
message [PayloadEnvelope](PayloadEnvelope.md) {
  [PayloadHeader](PayloadHeader.md) header = 1;
  bytes body = 2;
  [SharedMetadata](SharedMetadata.md) metadata = 3;
}
```

