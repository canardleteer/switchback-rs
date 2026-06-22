# Start a pipeline run

**startPipeline**(`pipeline`: [PipelineSpec](../schemas/PipelineSpec.md)) -> [PipelineRun](../schemas/PipelineRun.md)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `pipeline` | param | [PipelineSpec](../schemas/PipelineSpec.md) | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| run |  | — | [PipelineRun](../schemas/PipelineRun.md) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "startPipeline",
  "params": [
    {
      "name": "pipeline",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/PipelineSpec"
      }
    }
  ],
  "result": {
    "name": "run",
    "schema": {
      "$ref": "#/components/schemas/PipelineRun"
    }
  },
  "summary": "Start a pipeline run"
}
```

</details>

