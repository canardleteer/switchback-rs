# ApplyInventoryAdjustmentsRequest

ApplyInventoryAdjustmentsRequest batches adjustments.

```protobuf
message [ApplyInventoryAdjustmentsRequest](ApplyInventoryAdjustmentsRequest.md) {
  repeated [InventoryAdjustment](InventoryAdjustment.md) adjustments = 1;
  [ResourceIdentity](ResourceIdentity.md) actor = 2;
}
```

