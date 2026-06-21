# Upsert a flag override

****upsertFlagOverride**** `(override) -> FlagOverride`

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `override` | param | [FlagOverride](../schemas/FlagOverride.md) | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| override |  | — | [FlagOverride](../schemas/FlagOverride.md) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "upsertFlagOverride",
  "params": [
    {
      "name": "override",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/FlagOverride"
      }
    }
  ],
  "result": {
    "name": "override",
    "schema": {
      "$ref": "#/components/schemas/FlagOverride"
    }
  },
  "summary": "Upsert a flag override"
}
```

</details>

