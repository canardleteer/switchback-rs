# ListFeatureFlagsResponse

ListFeatureFlagsResponse returns a page of flags.

```protobuf
message [ListFeatureFlagsResponse](ListFeatureFlagsResponse.md) {
  repeated [FeatureFlag](FeatureFlag.md) flags = 1;
  acme.example.v2.[PageResult](../../v2/messages/PageResult.md) page = 2;
}
```

