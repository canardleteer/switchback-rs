# WatchInventoryResponse

WatchInventoryResponse is one event on the WatchInventory server stream.

```protobuf
message [WatchInventoryResponse](WatchInventoryResponse.md) {
  string event_id = 1;
  [InventoryAdjustment](InventoryAdjustment.md) adjustment = 2;
  google.protobuf.Timestamp observed_at = 3;
  [StreamCursor](StreamCursor.md) cursor = 4;
}
```

