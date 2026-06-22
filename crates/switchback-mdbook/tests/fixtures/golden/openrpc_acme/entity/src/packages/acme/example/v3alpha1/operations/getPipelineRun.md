# Get pipeline run status

**getPipelineRun**(`run_id`: `string`) -> [PipelineRun](../schemas/PipelineRun.md)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `run_id` | param | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| run |  | — | [PipelineRun](../schemas/PipelineRun.md) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "getPipelineRun",
  "params": [
    {
      "name": "run_id",
      "required": true,
      "schema": {
        "type": "string"
      }
    }
  ],
  "result": {
    "name": "run",
    "schema": {
      "$ref": "#/components/schemas/PipelineRun"
    }
  },
  "summary": "Get pipeline run status"
}
```

</details>

