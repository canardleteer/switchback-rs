# inventory/adjustments

**channel** `inventory/adjustments`

#### Messages

- [InventoryAdjustmentEvent](../message/InventoryAdjustmentEvent.md)

```yaml
bindings:
  kafka:
    bindingVersion: 0.4.0
    groupId: inventory-watchers
    topic: acme.inventory.adjustments
subscribe:
  message:
    $ref: "#/components/messages/InventoryAdjustmentEvent"
  operationId: subscribeInventoryAdjustments
  summary: Stream inventory adjustments
  tags:
  - inventory
tags:
- name: inventory
```

