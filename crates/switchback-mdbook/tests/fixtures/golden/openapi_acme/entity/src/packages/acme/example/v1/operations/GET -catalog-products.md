# /catalog/products

**GET** `/catalog/products`

Gateway proxy to the v2 catalog list operation. Response bodies use
**v2 schemas** even though the path lives under v1.

See `GET /products` in the v2 spec for pagination semantics.


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | header | `string` | required | Caller correlation id echoed in logs and problem details. |
| `Authorization` | header | `string` | optional |  |
| `page_size` | query | `integer` | optional |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Product page from v2 catalog schema | application/json | `ListProductsResponse` |
| 401 |  | — | [Unauthorized](../responses/Unauthorized.md) |
| 403 |  | — | [Forbidden](../responses/Forbidden.md) |
| 500 |  | — | [InternalError](../responses/InternalError.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Gateway proxy to the v2 catalog list operation. Response bodies use
  **v2 schemas** even though the path lives under v1.
  
  See `GET /products` in the v2 spec for pagination semantics.
operationId: listCatalogProducts
parameters:
- $ref: "#/components/parameters/XRequestId"
- $ref: "#/components/parameters/Authorization"
- in: query
  name: page_size
  schema:
    default: 20
    maximum: 100
    minimum: 1
    type: integer
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "../v2/openapi.yaml#/components/schemas/ListProductsResponse"
    description: Product page from v2 catalog schema
  "401":
    $ref: "#/components/responses/Unauthorized"
  "403":
    $ref: "#/components/responses/Forbidden"
  "500":
    $ref: "#/components/responses/InternalError"
summary: Proxy list products (v2 catalog types)
```

</details>

