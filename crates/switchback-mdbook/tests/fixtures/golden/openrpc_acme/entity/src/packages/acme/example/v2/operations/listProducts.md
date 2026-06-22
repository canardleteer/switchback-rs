# List catalog products

**listProducts** (page_size, page_token) -> [ListProductsResponse](../schemas/ListProductsResponse.md)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `page_size` | param | `integer` | optional |  |
| `page_token` | param | `string` | optional |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| page |  | — | [ListProductsResponse](../schemas/ListProductsResponse.md) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "listProducts",
  "params": [
    {
      "name": "page_size",
      "schema": {
        "default": 20,
        "maximum": 100,
        "minimum": 1,
        "type": "integer"
      }
    },
    {
      "name": "page_token",
      "schema": {
        "type": "string"
      }
    }
  ],
  "result": {
    "name": "page",
    "schema": {
      "$ref": "#/components/schemas/ListProductsResponse"
    }
  },
  "summary": "List catalog products"
}
```

</details>

