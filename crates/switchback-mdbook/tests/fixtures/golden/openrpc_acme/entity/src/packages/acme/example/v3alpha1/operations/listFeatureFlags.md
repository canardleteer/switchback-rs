# List feature flags

**listFeatureFlags**(`namespace`: `string`) -> [FeatureFlag](../schemas/FeatureFlag.md)[]

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `namespace` | param | `string` | optional |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| flags |  | — | [FeatureFlag](../schemas/FeatureFlag.md)[] |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "listFeatureFlags",
  "params": [
    {
      "name": "namespace",
      "schema": {
        "type": "string"
      }
    }
  ],
  "result": {
    "name": "flags",
    "schema": {
      "items": {
        "$ref": "#/components/schemas/FeatureFlag"
      },
      "type": "array"
    }
  },
  "summary": "List feature flags"
}
```

</details>

