# catalog/products/deleted

**channel** `catalog/products/deleted`

```yaml
bindings:
  kafka:
    bindingVersion: 0.4.0
    topic: acme.catalog.products.deleted
publish:
  message:
    $ref: "#/components/messages/ProductDeleted"
  operationId: publishProductDeleted
  summary: Product deleted event
  tags:
  - catalog
tags:
- name: catalog
```

