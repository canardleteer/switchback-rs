# ListProductsResponse

```json
{
  "properties": {
    "page": {
      "properties": {
        "next_page_token": {
          "type": "string"
        }
      },
      "type": "object"
    },
    "products": {
      "items": {
        "$ref": "#/components/schemas/Product"
      },
      "type": "array"
    }
  },
  "type": "object"
}
```

