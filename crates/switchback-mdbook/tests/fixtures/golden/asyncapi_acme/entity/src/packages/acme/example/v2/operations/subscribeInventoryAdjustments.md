# Stream inventory adjustments

**SUBSCRIBE** `inventory/adjustments` — `kafka` topic `acme.inventory.adjustments`

```mermaid
sequenceDiagram
  participant Client
  participant Broker as inventory/adjustments
  Client->>Broker: subscribe (subscribeInventoryAdjustments)
```

#### Messages

- [InventoryAdjustmentEvent](../message/InventoryAdjustmentEvent.md)

```yaml
message:
  $ref: "#/components/messages/InventoryAdjustmentEvent"
operationId: subscribeInventoryAdjustments
summary: Stream inventory adjustments
tags:
- inventory
```

