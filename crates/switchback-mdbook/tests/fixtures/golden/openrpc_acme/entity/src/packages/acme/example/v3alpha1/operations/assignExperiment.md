# Assign an experiment cohort

**assignExperiment** (subject_id, experiment_id) -> [ExperimentAssignment](../schemas/ExperimentAssignment.md)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `subject_id` | param | `string` | required |  |
| `experiment_id` | param | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| assignment |  | — | [ExperimentAssignment](../schemas/ExperimentAssignment.md) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "assignExperiment",
  "params": [
    {
      "name": "subject_id",
      "required": true,
      "schema": {
        "type": "string"
      }
    },
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
  "summary": "Assign an experiment cohort"
}
```

</details>

