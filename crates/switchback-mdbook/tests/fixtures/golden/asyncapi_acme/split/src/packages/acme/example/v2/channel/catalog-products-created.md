# catalog/products/created

**channel** `catalog/products/created`

```yaml
bindings:
  kafka:
    bindingVersion: 0.4.0
    partitions: 12
    topic: acme.catalog.products.created
publish:
  message:
    $ref: "#/components/messages/ProductCreated"
  operationId: publishProductCreated
  summary: Product created event
  tags:
  - catalog
tags:
- name: catalog
```

