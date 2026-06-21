# Export an audit batch

****exportAuditBatch**** `(since) -> AuditBatch`

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `since` | param | `string` | optional |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| batch |  | — | [AuditBatch](../schemas/AuditBatch.md) |

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

