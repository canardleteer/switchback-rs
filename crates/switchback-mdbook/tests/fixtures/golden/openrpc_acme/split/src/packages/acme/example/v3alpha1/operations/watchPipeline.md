# Watch pipeline events

**watchPipeline**(`run_id`: `string`) -> [PipelineEvent](../schemas/PipelineEvent.md)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `run_id` | param | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| event |  | — | [PipelineEvent](../schemas/PipelineEvent.md) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "watchPipeline",
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
    "name": "event",
    "schema": {
      "$ref": "#/components/schemas/PipelineEvent"
    }
  },
  "summary": "Watch pipeline events"
}
```

</details>

