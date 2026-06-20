# FeatureFlag

FeatureFlag describes a toggle evaluated by fictional clients.

```protobuf
message [FeatureFlag](FeatureFlag.md) {
// Stable identifier (slug).
  string key = 1 [
      (buf.validate.field).required = true,
      (buf.validate.field).string.min_len = 1,
      (buf.validate.field).string.max_len = 128,
      (buf.validate.field).string.pattern = "^[a-z][a-z0-9_.-]*$"
    ];
  string display_name = 2 [(buf.validate.field).string.max_len = 256];
  string description = 3;
  bool default_enabled = 4;
  [ReleaseChannel](../enums/ReleaseChannel.md) channel = 5;
  acme.example.v2.[SharedMetadata](../../v2/messages/SharedMetadata.md) metadata = 6;
  google.protobuf.Timestamp updated_at = 7;
}
```

