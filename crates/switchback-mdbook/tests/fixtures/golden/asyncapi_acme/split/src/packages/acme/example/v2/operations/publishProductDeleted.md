# Product deleted event

**PUBLISH** `catalog/products/deleted` — `kafka` topic `acme.catalog.products.deleted`

```mermaid
sequenceDiagram
  participant Client
  participant Broker as catalog/products/deleted
  Client->>Broker: publish (publishProductDeleted)
```

#### Messages

- [ProductDeleted](../message/ProductDeleted.md)

```yaml
message:
  $ref: "#/components/messages/ProductDeleted"
operationId: publishProductDeleted
summary: Product deleted event
tags:
- catalog
```

