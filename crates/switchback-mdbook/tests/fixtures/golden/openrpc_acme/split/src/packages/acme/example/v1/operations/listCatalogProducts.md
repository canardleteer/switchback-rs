# Proxy list products (v2 catalog types)

**listCatalogProducts** (XRequestId, Authorization, page_size) -> [ListProductsResponse](../../v2/schemas/ListProductsResponse.md)

Gateway proxy to the v2 catalog list operation.

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | param | `string` | required | Caller correlation id echoed in logs and problem details. |
| `Authorization` | param | `string` | optional |  |
| `page_size` | param | `integer` | optional |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| page |  | — | [ListProductsResponse](../../v2/schemas/ListProductsResponse.md) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "description": "Gateway proxy to the v2 catalog list operation.",
  "name": "listCatalogProducts",
  "params": [
    {
      "$ref": "#/components/contentDescriptors/XRequestId"
    },
    {
      "$ref": "#/components/contentDescriptors/Authorization"
    },
    {
      "name": "page_size",
      "schema": {
        "default": 20,
        "maximum": 100,
        "minimum": 1,
        "type": "integer"
      }
    }
  ],
  "result": {
    "name": "page",
    "schema": {
      "$ref": "../v2/openrpc.json#/components/schemas/ListProductsResponse"
    }
  },
  "summary": "Proxy list products (v2 catalog types)"
}
```

</details>

