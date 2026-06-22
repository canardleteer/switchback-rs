# Watch inventory changes

**watchInventory** (sku) -> [InventoryEvent](../schemas/InventoryEvent.md)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `sku` | param | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| event |  | — | [InventoryEvent](../schemas/InventoryEvent.md) |

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

