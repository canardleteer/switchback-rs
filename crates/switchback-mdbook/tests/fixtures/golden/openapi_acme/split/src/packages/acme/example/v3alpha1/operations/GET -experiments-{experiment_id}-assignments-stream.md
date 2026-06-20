# /experiments/{experiment_id}/assignments/stream

**GET** `/experiments/{experiment_id}/assignments/stream` — response stream

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `ExperimentId` | path | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Assignment events | text/event-stream | `200` |
| 404 |  | — | [NotFound](../responses/NotFound.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
operationId: streamAssignments
parameters:
- $ref: "#/components/parameters/ExperimentId"
responses:
  "200":
    content:
      text/event-stream:
        schema:
          type: string
    description: Assignment events
  "404":
    $ref: "#/components/responses/NotFound"
summary: Stream experiment assignments (SSE)
```

</details>

