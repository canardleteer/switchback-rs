# /products/{product_id}

**DELETE** `/products/{product_id}`

Permanently removes a catalog product and its SKUs.

**Irreversible.** Inventory adjustments are *not* rolled back
automatically; reconcile warehouse counts separately.

| Concern | Behavior |
| --- | --- |
| Idempotency | Repeating DELETE after success returns `404` |
| Audit | Emits a `product.deleted` event (see audit export) |


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `ProductId` | path | `string` | required | UUID of the catalog product (see the [Product](../schemas/Product.md) schema). |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 204 | Deleted | — | `204` |
| 404 |  | — | [NotFound](../responses/NotFound.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Permanently removes a catalog product and its SKUs.
  
  **Irreversible.** Inventory adjustments are *not* rolled back
  automatically; reconcile warehouse counts separately.
  
  | Concern | Behavior |
  | --- | --- |
  | Idempotency | Repeating DELETE after success returns `404` |
  | Audit | Emits a `product.deleted` event (see audit export) |
operationId: deleteProduct
parameters:
- $ref: "#/components/parameters/ProductId"
responses:
  "204":
    description: Deleted
  "404":
    $ref: "#/components/responses/NotFound"
summary: Delete a product
```

</details>

