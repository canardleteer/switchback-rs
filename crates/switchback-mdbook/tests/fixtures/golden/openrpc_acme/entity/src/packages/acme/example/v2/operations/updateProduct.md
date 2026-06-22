# Replace a product

**updateProduct**(`product_id`: `string`, `product`: [Product](../schemas/Product.md)) -> [Product](../schemas/Product.md)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `product_id` | param | `string` | required |  |
| `product` | param | [Product](../schemas/Product.md) | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| product |  | — | [Product](../schemas/Product.md) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "updateProduct",
  "params": [
    {
      "name": "product_id",
      "required": true,
      "schema": {
        "type": "string"
      }
    },
    {
      "name": "product",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/Product"
      }
    }
  ],
  "result": {
    "name": "product",
    "schema": {
      "$ref": "#/components/schemas/Product"
    }
  },
  "summary": "Replace a product"
}
```

</details>

