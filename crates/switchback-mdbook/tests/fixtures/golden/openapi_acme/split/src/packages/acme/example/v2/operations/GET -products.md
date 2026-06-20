# /products

**GET** `/products`

Returns a **paginated** page of catalog products visible to the caller.

Use `page_size` and `page_token` to walk large catalogs. The default
page size is 20; the maximum is 100.

| Field | Notes |
| --- | --- |
| `products[]` | Full [Product](../schemas/Product.md) records including SKUs |
| `page.next_page_token` | Opaque; pass back for the next page |


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `page_size` | query | `integer` | optional | Max products to return (1–100, default 20). |
| `page_token` | query | `string` | optional | Opaque token from a prior [ListProductsResponse](../schemas/ListProductsResponse.md).page. |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Paginated product page | application/json | [ListProductsResponse](../schemas/ListProductsResponse.md) |
| 400 |  | — | [BadRequest](../responses/BadRequest.md) |
| 500 |  | — | [InternalError](../responses/InternalError.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Returns a **paginated** page of catalog products visible to the caller.
  
  Use `page_size` and `page_token` to walk large catalogs. The default
  page size is 20; the maximum is 100.
  
  | Field | Notes |
  | --- | --- |
  | `products[]` | Full `Product` records including SKUs |
  | `page.next_page_token` | Opaque; pass back for the next page |
operationId: listProducts
parameters:
- description: Max products to return (1–100, default 20).
  in: query
  name: page_size
  schema:
    default: 20
    maximum: 100
    minimum: 1
    type: integer
- description: Opaque token from a prior `ListProductsResponse.page`.
  in: query
  name: page_token
  schema:
    type: string
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/ListProductsResponse"
    description: Paginated product page
  "400":
    $ref: "#/components/responses/BadRequest"
  "500":
    $ref: "#/components/responses/InternalError"
summary: List catalog products
```

</details>

