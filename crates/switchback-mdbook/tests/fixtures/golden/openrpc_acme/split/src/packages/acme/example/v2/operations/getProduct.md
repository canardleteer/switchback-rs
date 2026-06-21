# Get one product

****getProduct**** `(product_id) -> Product`

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `product_id` | param | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| product |  | — | [Product](../schemas/Product.md) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "getProduct",
  "params": [
    {
      "name": "product_id",
      "required": true,
      "schema": {
        "type": "string"
      }
    }
  ],
  "result": {
    "name": "product",
    "schema": {
      "$ref": "#/components/schemas/Product"
    }
  },
  "summary": "Get one product"
}
```

</details>

