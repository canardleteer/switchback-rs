# Stream assignment updates

**streamAssignments**(`experiment_id`: `string`) -> [ExperimentAssignment](../schemas/ExperimentAssignment.md)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `experiment_id` | param | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| assignment |  | — | [ExperimentAssignment](../schemas/ExperimentAssignment.md) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "streamAssignments",
  "params": [
    {
      "name": "experiment_id",
      "required": true,
      "schema": {
        "type": "string"
      }
    }
  ],
  "result": {
    "name": "assignment",
    "schema": {
      "$ref": "#/components/schemas/ExperimentAssignment"
    }
  },
  "summary": "Stream assignment updates"
}
```

</details>

