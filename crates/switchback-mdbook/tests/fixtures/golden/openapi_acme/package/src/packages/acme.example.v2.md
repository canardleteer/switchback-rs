# acme.example.v2

Catalog, inventory, and platform services for the Acme fixture.



## Operations

### /inventory/watch

**GET** `/inventory/watch` — response stream

Opens a **server-sent events** stream of inventory adjustments for one
warehouse.

Each event carries an [InventoryAdjustment](#inventoryadjustment) JSON payload in the `data`
field. Clients should reconnect with exponential backoff on disconnect.


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `warehouse_id` | query | `string` | required | Warehouse identifier to watch (must exist). |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Inventory adjustment events | text/event-stream | `200` |
| 404 |  | — | [NotFound](#notfound) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Opens a **server-sent events** stream of inventory adjustments for one
  warehouse.
  
  Each event carries an `InventoryAdjustment` JSON payload in the `data`
  field. Clients should reconnect with exponential backoff on disconnect.
operationId: watchInventory
parameters:
- description: Warehouse identifier to watch (must exist).
  in: query
  name: warehouse_id
  required: true
  schema:
    type: string
responses:
  "200":
    content:
      text/event-stream:
        schema:
          type: string
    description: Inventory adjustment events
  "404":
    $ref: "#/components/responses/NotFound"
summary: Stream inventory adjustments (SSE)
```

</details>

### /platform/audit/export

**GET** `/platform/audit/export`

Returns a **paginated snapshot** of audit records for compliance
exports.

Callers must hold the `platform.audit.export` permission. Results are
eventually consistent with the live audit log (typically under 30
seconds).

## Pagination

Use `page_size` to limit rows per response. When `page.next_page_token`
is non-empty, repeat the request with that token until exhausted.

## Related types

- Response body: [ExportAuditBatchResponse](#exportauditbatchresponse)
- Nested batch: [AuditBatch](#auditbatch)


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `tenant_id` | query | `string` | required | Tenant scope; must match the caller's organization. |
| `page_size` | query | `integer` | optional | Maximum records per page (default 50). |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Audit export page | application/json | [ExportAuditBatchResponse](#exportauditbatchresponse) |
| 403 |  | — | [Forbidden](#forbidden) |
| 500 |  | — | [InternalError](#internalerror) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Returns a **paginated snapshot** of audit records for compliance
  exports.
  
  Callers must hold the `platform.audit.export` permission. Results are
  eventually consistent with the live audit log (typically under 30
  seconds).
  
  ## Pagination
  
  Use `page_size` to limit rows per response. When `page.next_page_token`
  is non-empty, repeat the request with that token until exhausted.
  
  ## Related types
  
  - Response body: `ExportAuditBatchResponse`
  - Nested batch: `AuditBatch`
operationId: exportAuditBatch
parameters:
- description: Tenant scope; must match the caller's organization.
  in: query
  name: tenant_id
  required: true
  schema:
    type: string
- description: Maximum records per page (default 50).
  in: query
  name: page_size
  schema:
    default: 50
    type: integer
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/ExportAuditBatchResponse"
    description: Audit export page
  "403":
    $ref: "#/components/responses/Forbidden"
  "500":
    $ref: "#/components/responses/InternalError"
summary: Export audit batch snapshot
```

</details>

### /products

**GET** `/products`

Returns a **paginated** page of catalog products visible to the caller.

Use `page_size` and `page_token` to walk large catalogs. The default
page size is 20; the maximum is 100.

| Field | Notes |
| --- | --- |
| `products[]` | Full [Product](#product) records including SKUs |
| `page.next_page_token` | Opaque; pass back for the next page |


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `page_size` | query | `integer` | optional | Max products to return (1–100, default 20). |
| `page_token` | query | `string` | optional | Opaque token from a prior [ListProductsResponse](#listproductsresponse).page. |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Paginated product page | application/json | [ListProductsResponse](#listproductsresponse) |
| 400 |  | — | [BadRequest](#badrequest) |
| 500 |  | — | [InternalError](#internalerror) |

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

### /products

**POST** `/products`

Creates a new catalog product. The server assigns `product_id` when the
client omits it.

**Validation:** `display_name` is required. SKU entries may be omitted
for draft products; activate them with a follow-up `PUT`.


#### Request body

`application/json`: [Product](#product) (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 201 | Created product | application/json | [Product](#product) |
| 400 |  | — | [BadRequest](#badrequest) |

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

### /products/drafts

**PUT** `/products/drafts` — request stream

Uploads raw draft bytes for a product work-in-progress. The body is
`application/octet-stream`; metadata lives in query parameters only.

Useful for exercising **binary request bodies** in rendered docs.


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `draft_id` | query | `string` | required | Client-generated draft id; idempotent per draft. |

#### Request body

`application/octet-stream`: `string` (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Draft upload accepted | application/json | [UploadDraftsResponse](#uploaddraftsresponse) |
| 400 |  | — | [BadRequest](#badrequest) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Uploads raw draft bytes for a product work-in-progress. The body is
  `application/octet-stream`; metadata lives in query parameters only.
  
  Useful for exercising **binary request bodies** in rendered docs.
operationId: uploadDrafts
parameters:
- description: Client-generated draft id; idempotent per draft.
  in: query
  name: draft_id
  required: true
  schema:
    type: string
requestBody:
  content:
    application/octet-stream:
      schema:
        format: binary
        type: string
  required: true
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/UploadDraftsResponse"
    description: Draft upload accepted
  "400":
    $ref: "#/components/responses/BadRequest"
summary: Upload product draft bytes (octet-stream)
```

</details>

### /products/{product_id}

**GET** `/products/{product_id}`

Fetches a single product by id, including nested SKUs and shared
metadata.

Returns `404` when the product was deleted or the id is unknown.


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `ProductId` | path | `string` | required | UUID of the catalog product (see the [Product](#product) schema). |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Product | application/json | [Product](#product) |
| 404 |  | — | [NotFound](#notfound) |

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

### /products/{product_id}

**PUT** `/products/{product_id}`

Replaces the entire product record. Partial updates are **not**
supported; send the full [Product](#product) shape.

Optimistic concurrency is out of scope for this fixture; last writer
wins.


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `ProductId` | path | `string` | required | UUID of the catalog product (see the [Product](#product) schema). |

#### Request body

`application/json`: [Product](#product) (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Updated product | application/json | [Product](#product) |
| 404 |  | — | [NotFound](#notfound) |

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

### /products/{product_id}

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
| `ProductId` | path | `string` | required | UUID of the catalog product (see the [Product](#product) schema). |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 204 | Deleted | — | `204` |
| 404 |  | — | [NotFound](#notfound) |

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

## Schemas

### AuditBatch

```yaml
properties:
  batch_id:
    type: string
  records:
    items:
      $ref: "#/components/schemas/AuditRecord"
    type: array
type: object
```

### AuditRecord

```yaml
properties:
  action:
    type: string
  actor:
    type: string
  occurred_at:
    format: date-time
    type: string
type: object
```

### ExportAuditBatchResponse

```yaml
properties:
  batch:
    $ref: "#/components/schemas/AuditBatch"
  page:
    $ref: "#/components/schemas/PageResult"
type: object
```

### InventoryAdjustment

```yaml
properties:
  delta:
    format: int64
    type: integer
  reason:
    type: string
  sku:
    type: string
type: object
```

### ListOptions

```yaml
properties:
  page_size:
    type: integer
  page_token:
    nullable: true
    type: string
type: object
```

### ListProductsResponse

```yaml
properties:
  page:
    $ref: "#/components/schemas/PageResult"
  products:
    items:
      $ref: "#/components/schemas/Product"
    type: array
type: object
```

### PageResult

```yaml
properties:
  next_page_token:
    nullable: true
    type: string
  total_size:
    format: int64
    type: integer
type: object
```

### Problem

```yaml
$ref: "../shared/schemas.yaml#/Problem"
```

### Product

```yaml
properties:
  display_name:
    type: string
  metadata:
    $ref: "#/components/schemas/SharedMetadata"
  product_id:
    format: uuid
    type: string
  skus:
    items:
      $ref: "#/components/schemas/ProductSku"
    type: array
required:
- product_id
- display_name
type: object
```

### ProductSku

```yaml
nullable: true
properties:
  sku:
    type: string
  status:
    enum:
    - draft
    - active
    - archived
    type: string
  title:
    type: string
type: object
```

### SharedMetadata

```yaml
properties:
  created_at:
    format: date-time
    type: string
  parent_span_id:
    nullable: true
    type: string
  trace_id:
    type: string
type: object
```

### UploadDraftsResponse

```yaml
properties:
  draft_id:
    type: string
  parts_received:
    type: integer
  product:
    $ref: "#/components/schemas/Product"
type: object
```

## Parameters

### ProductId

Location: `path` (required)

UUID of the catalog product (see the `Product` schema).

```yaml
format: uuid
type: string
```

## Responses

### BadRequest

Status: `Bad request`

Media type: `application/problem+json`

Bad request

```yaml
content:
  application/problem+json:
    schema:
      $ref: "#/components/schemas/Problem"
description: Bad request
```

### Forbidden

Status: `Forbidden`

Media type: `application/problem+json`

Forbidden

```yaml
content:
  application/problem+json:
    schema:
      $ref: "#/components/schemas/Problem"
description: Forbidden
```

### InternalError

Status: `Internal error`

Media type: `application/problem+json`

Internal error

```yaml
content:
  application/problem+json:
    schema:
      $ref: "#/components/schemas/Problem"
description: Internal error
```

### NotFound

Status: `Not found`

Media type: `application/problem+json`

Not found

```yaml
content:
  application/problem+json:
    schema:
      $ref: "#/components/schemas/Problem"
description: Not found
```

