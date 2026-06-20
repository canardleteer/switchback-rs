# /inventory/watch

**GET** `/inventory/watch` — response stream

Opens a **server-sent events** stream of inventory adjustments for one
warehouse.

Each event carries an [InventoryAdjustment](../schemas/InventoryAdjustment.md) JSON payload in the `data`
field. Clients should reconnect with exponential backoff on disconnect.


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `warehouse_id` | query | `string` | required | Warehouse identifier to watch (must exist). |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Inventory adjustment events | text/event-stream | `200` |
| 404 |  | — | [NotFound](../responses/NotFound.md) |

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

