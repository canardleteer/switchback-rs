# /products

**POST** `/products`

Creates a new catalog product. The server assigns `product_id` when the
client omits it.

**Validation:** `display_name` is required. SKU entries may be omitted
for draft products; activate them with a follow-up `PUT`.


#### Request body

`application/json`: [Product](../schemas/Product.md) (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 201 | Created product | application/json | [Product](../schemas/Product.md) |
| 400 |  | — | [BadRequest](../responses/BadRequest.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Creates a new catalog product. The server assigns `product_id` when the
  client omits it.
  
  **Validation:** `display_name` is required. SKU entries may be omitted
  for draft products; activate them with a follow-up `PUT`.
operationId: createProduct
requestBody:
  content:
    application/json:
      schema:
        $ref: "#/components/schemas/Product"
  required: true
responses:
  "201":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/Product"
    description: Created product
  "400":
    $ref: "#/components/responses/BadRequest"
summary: Create a catalog product
```

</details>

