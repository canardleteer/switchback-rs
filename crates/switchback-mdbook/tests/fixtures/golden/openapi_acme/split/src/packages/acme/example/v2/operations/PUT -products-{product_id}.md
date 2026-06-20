# /products/{product_id}

**PUT** `/products/{product_id}`

Replaces the entire product record. Partial updates are **not**
supported; send the full [Product](../schemas/Product.md) shape.

Optimistic concurrency is out of scope for this fixture; last writer
wins.


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `ProductId` | path | `string` | required | UUID of the catalog product (see the [Product](../schemas/Product.md) schema). |

#### Request body

`application/json`: [Product](../schemas/Product.md) (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Updated product | application/json | [Product](../schemas/Product.md) |
| 404 |  | — | [NotFound](../responses/NotFound.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Replaces the entire product record. Partial updates are **not**
  supported; send the full `Product` shape.
  
  Optimistic concurrency is out of scope for this fixture; last writer
  wins.
operationId: updateProduct
parameters:
- $ref: "#/components/parameters/ProductId"
requestBody:
  content:
    application/json:
      schema:
        $ref: "#/components/schemas/Product"
  required: true
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/Product"
    description: Updated product
  "404":
    $ref: "#/components/responses/NotFound"
summary: Replace a product
```

</details>

