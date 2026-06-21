# List feature flags

****listFeatureFlags**** `(namespace) -> array`

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `namespace` | param | `string` | optional |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| flags |  | — | — |

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

