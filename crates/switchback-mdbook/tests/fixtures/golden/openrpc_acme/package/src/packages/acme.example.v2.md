# acme.example.v2

Catalog, inventory, and platform services for the Acme fixture.



## Operations

### Create a catalog product

****createProduct**** `(product) -> Product`

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `product` | param | [Product](#product) | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| product |  | — | [Product](#product) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "createProduct",
  "params": [
    {
      "name": "product",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/Product"
      }
    }
  ],
  "result": {
    "name": "product",
    "schema": {
      "$ref": "#/components/schemas/Product"
    }
  },
  "summary": "Create a catalog product"
}
```

</details>

### Delete a product

****deleteProduct**** `(product_id) -> boolean`

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

### Export an audit batch

****exportAuditBatch**** `(since) -> AuditBatch`

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `since` | param | `string` | optional |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| batch |  | — | [AuditBatch](#auditbatch) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "exportAuditBatch",
  "params": [
    {
      "name": "since",
      "schema": {
        "format": "date-time",
        "type": "string"
      }
    }
  ],
  "result": {
    "name": "batch",
    "schema": {
      "$ref": "#/components/schemas/AuditBatch"
    }
  },
  "summary": "Export an audit batch"
}
```

</details>

### Get one product

****getProduct**** `(product_id) -> Product`

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `product_id` | param | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| product |  | — | [Product](#product) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "getProduct",
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
    "name": "product",
    "schema": {
      "$ref": "#/components/schemas/Product"
    }
  },
  "summary": "Get one product"
}
```

</details>

### List catalog products

****listProducts**** `(page_size, page_token) -> ListProductsResponse`

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `page_size` | param | `integer` | optional |  |
| `page_token` | param | `string` | optional |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| page |  | — | [ListProductsResponse](#listproductsresponse) |

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

### Replace a product

****updateProduct**** `(product_id, product) -> Product`

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `product_id` | param | `string` | required |  |
| `product` | param | [Product](#product) | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| product |  | — | [Product](#product) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "updateProduct",
  "params": [
    {
      "name": "product_id",
      "required": true,
      "schema": {
        "type": "string"
      }
    },
    {
      "name": "product",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/Product"
      }
    }
  ],
  "result": {
    "name": "product",
    "schema": {
      "$ref": "#/components/schemas/Product"
    }
  },
  "summary": "Replace a product"
}
```

</details>

### Upload draft product payloads

****uploadDrafts**** `(drafts) -> integer`

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `drafts` | param | `array` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| accepted |  | — | — |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "uploadDrafts",
  "params": [
    {
      "name": "drafts",
      "required": true,
      "schema": {
        "items": {
          "$ref": "#/components/schemas/ProductDraft"
        },
        "type": "array"
      }
    }
  ],
  "result": {
    "name": "accepted",
    "schema": {
      "type": "integer"
    }
  },
  "summary": "Upload draft product payloads"
}
```

</details>

### Watch inventory changes

****watchInventory**** `(sku) -> InventoryEvent`

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `sku` | param | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| event |  | — | [InventoryEvent](#inventoryevent) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "watchInventory",
  "params": [
    {
      "name": "sku",
      "required": true,
      "schema": {
        "type": "string"
      }
    }
  ],
  "result": {
    "name": "event",
    "schema": {
      "$ref": "#/components/schemas/InventoryEvent"
    }
  },
  "summary": "Watch inventory changes"
}
```

</details>

## Schemas

### AuditBatch

```json
{
  "properties": {
    "entries": {
      "items": {
        "type": "string"
      },
      "type": "array"
    }
  },
  "type": "object"
}
```

### InventoryEvent

```json
{
  "properties": {
    "delta": {
      "type": "integer"
    },
    "observed_at": {
      "format": "date-time",
      "type": "string"
    },
    "sku_id": {
      "type": "string"
    }
  },
  "type": "object"
}
```

### ListProductsResponse

```json
{
  "properties": {
    "page": {
      "properties": {
        "next_page_token": {
          "type": "string"
        }
      },
      "type": "object"
    },
    "products": {
      "items": {
        "$ref": "#/components/schemas/Product"
      },
      "type": "array"
    }
  },
  "type": "object"
}
```

### Problem

```json
{
  "$ref": "../shared/schemas.json#/Problem"
}
```

### Product

```json
{
  "properties": {
    "display_name": {
      "type": "string"
    },
    "product_id": {
      "type": "string"
    },
    "skus": {
      "items": {
        "$ref": "#/components/schemas/Sku"
      },
      "type": "array"
    }
  },
  "required": [
    "display_name"
  ],
  "type": "object"
}
```

### ProductDraft

```json
{
  "properties": {
    "display_name": {
      "type": "string"
    }
  },
  "type": "object"
}
```

### Sku

```json
{
  "properties": {
    "quantity": {
      "type": "integer"
    },
    "sku_id": {
      "type": "string"
    }
  },
  "type": "object"
}
```

