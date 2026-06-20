# /products/{product_id}

**GET** `/products/{product_id}`

Fetches a single product by id, including nested SKUs and shared
metadata.

Returns `404` when the product was deleted or the id is unknown.


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `ProductId` | path | `string` | required | UUID of the catalog product (see the [Product](../schemas/Product.md) schema). |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Product | application/json | [Product](../schemas/Product.md) |
| 404 |  | — | [NotFound](../responses/NotFound.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Fetches a single product by id, including nested SKUs and shared
  metadata.
  
  Returns `404` when the product was deleted or the id is unknown.
operationId: getProduct
parameters:
- $ref: "#/components/parameters/ProductId"
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/Product"
    description: Product
  "404":
    $ref: "#/components/responses/NotFound"
summary: Get one product
```

</details>

