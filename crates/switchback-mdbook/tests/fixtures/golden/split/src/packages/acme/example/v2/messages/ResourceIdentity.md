# ResourceIdentity

ResourceIdentity combines tenant, labels, and metadata.

```protobuf
message [ResourceIdentity](ResourceIdentity.md) {
  [TenantRef](TenantRef.md) tenant = 1;
  [LabelSet](LabelSet.md) labels = 2;
  [SharedMetadata](SharedMetadata.md) metadata = 3;
  string resource_name = 4;
}
```

