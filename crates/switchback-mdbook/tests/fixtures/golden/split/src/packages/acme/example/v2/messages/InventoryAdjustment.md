# InventoryAdjustment

InventoryAdjustment documents stock changes for admin RPCs.

```protobuf
message [InventoryAdjustment](InventoryAdjustment.md) {
  string sku = 1;
  int64 delta = 2;
  string reason = 3;
  google.protobuf.Timestamp adjusted_at = 4;
}
```

