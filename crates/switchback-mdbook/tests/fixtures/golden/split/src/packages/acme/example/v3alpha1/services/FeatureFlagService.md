# FeatureFlagService

FeatureFlagService documents flag and override RPCs.

**ListFeatureFlags** ( [ListFeatureFlagsRequest](../messages/ListFeatureFlagsRequest.md) ) returns ( [ListFeatureFlagsResponse](../messages/ListFeatureFlagsResponse.md) )

ListFeatureFlags returns a paginated flag catalog page.

```protobuf
rpc ListFeatureFlags (acme.example.v3alpha1.[ListFeatureFlagsRequest](../messages/ListFeatureFlagsRequest.md)) returns (acme.example.v3alpha1.[ListFeatureFlagsResponse](../messages/ListFeatureFlagsResponse.md));
```

**UpsertFlagOverride** ( [UpsertFlagOverrideRequest](../messages/UpsertFlagOverrideRequest.md) ) returns ( [UpsertFlagOverrideResponse](../messages/UpsertFlagOverrideResponse.md) )

UpsertFlagOverride stores a tenant override row.

```protobuf
rpc UpsertFlagOverride (acme.example.v3alpha1.[UpsertFlagOverrideRequest](../messages/UpsertFlagOverrideRequest.md)) returns (acme.example.v3alpha1.[UpsertFlagOverrideResponse](../messages/UpsertFlagOverrideResponse.md));
```

**PublishHints** ( [PublishHintsRequest](../messages/PublishHintsRequest.md) ) returns ( [PublishHintsResponse](../messages/PublishHintsResponse.md) )

PublishHints accepts a client stream of platform hints.

```protobuf
rpc PublishHints (stream acme.example.v3alpha1.[PublishHintsRequest](../messages/PublishHintsRequest.md)) returns (acme.example.v3alpha1.[PublishHintsResponse](../messages/PublishHintsResponse.md));
```

**SyncFlags** ( [SyncFlagsRequest](../messages/SyncFlagsRequest.md) ) returns ( [SyncFlagsResponse](../messages/SyncFlagsResponse.md) )

SyncFlags synchronizes flag revisions over a bidirectional stream.

```protobuf
rpc SyncFlags (stream acme.example.v3alpha1.[SyncFlagsRequest](../messages/SyncFlagsRequest.md)) returns (stream acme.example.v3alpha1.[SyncFlagsResponse](../messages/SyncFlagsResponse.md));
```

