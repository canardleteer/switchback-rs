# PlatformHint

PlatformHint links alpha metadata back into v2 envelopes.

```protobuf
message [PlatformHint](PlatformHint.md) {
  string hint_id = 1;
  acme.example.v2.[PayloadEnvelope](../../v2/messages/PayloadEnvelope.md) envelope = 2;
  repeated acme.example.v2.[Label](../../v2/messages/Label.md) labels = 3;
}
```

