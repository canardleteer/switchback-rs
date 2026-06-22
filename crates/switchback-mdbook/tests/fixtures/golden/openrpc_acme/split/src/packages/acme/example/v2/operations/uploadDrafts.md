# Upload draft product payloads

**uploadDrafts** (drafts) -> —

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

