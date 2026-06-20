# ListFeatureFlagsRequest

ListFeatureFlagsRequest paginates feature flags for a tenant.

```protobuf
message [ListFeatureFlagsRequest](ListFeatureFlagsRequest.md) {
  acme.example.v2.[ListOptions](../../v2/messages/ListOptions.md) options = 1;
  acme.example.v2.[TenantRef](../../v2/messages/TenantRef.md) tenant = 2;
  [ReleaseChannel](../enums/ReleaseChannel.md) channel_filter = 3;
}
```

