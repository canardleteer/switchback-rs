# Product

```yaml
properties:
  display_name:
    type: string
  metadata:
    $ref: "#/components/schemas/SharedMetadata"
  product_id:
    format: uuid
    type: string
  skus:
    items:
      $ref: "#/components/schemas/ProductSku"
    type: array
required:
- product_id
- display_name
type: object
```

