# Product created event

**PUBLISH** `catalog/products/created` — `kafka` topic `acme.catalog.products.created`

```mermaid
sequenceDiagram
  participant Client
  participant Broker as catalog/products/created
  Client->>Broker: publish (publishProductCreated)
```

```yaml
message:
  $ref: "#/components/messages/ProductCreated"
operationId: publishProductCreated
summary: Product created event
tags:
- catalog
```

