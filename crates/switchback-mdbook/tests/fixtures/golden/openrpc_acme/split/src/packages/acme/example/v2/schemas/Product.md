# Product

```json
{
  "properties": {
    "display_name": {
      "type": "string"
    },
    "product_id": {
      "type": "string"
    },
    "skus": {
      "items": {
        "$ref": "#/components/schemas/Sku"
      },
      "type": "array"
    }
  },
  "required": [
    "display_name"
  ],
  "type": "object"
}
```

