# Create a catalog product

**createProduct**(`product`: [Product](../schemas/Product.md)) -> [Product](../schemas/Product.md)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `product` | param | [Product](../schemas/Product.md) | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| product |  | — | [Product](../schemas/Product.md) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "createProduct",
  "params": [
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
  "summary": "Create a catalog product"
}
```

</details>

