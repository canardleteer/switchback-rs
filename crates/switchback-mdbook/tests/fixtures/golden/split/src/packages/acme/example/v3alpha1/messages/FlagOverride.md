# FlagOverride

FlagOverride pins a flag value for one tenant.

```protobuf
message [FlagOverride](FlagOverride.md) {
  acme.example.v2.[TenantRef](../../v2/messages/TenantRef.md) tenant = 1;
  string flag_key = 2 [(buf.validate.field).string.min_len = 1];
  bool enabled = 3;
  google.protobuf.Timestamp expires_at = 4;
}
```

