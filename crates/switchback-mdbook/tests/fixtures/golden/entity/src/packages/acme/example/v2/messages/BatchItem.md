# BatchItem

BatchItem pairs a key with an envelope for batch APIs.

```protobuf
message [BatchItem](BatchItem.md) {
  [BatchKey](BatchKey.md) key = 1;
  [PayloadEnvelope](PayloadEnvelope.md) envelope = 2;
}
```

