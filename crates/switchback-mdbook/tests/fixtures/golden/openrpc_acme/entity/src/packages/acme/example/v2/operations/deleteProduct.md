# Delete a product

**deleteProduct**(`product_id`: `string`) -> `boolean`

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `product_id` | param | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| deleted |  | — | — |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "deleteProduct",
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
    "name": "deleted",
    "schema": {
      "type": "boolean"
    }
  },
  "summary": "Delete a product"
}
```

</details>

