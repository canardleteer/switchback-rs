# /pipelines/{run_id}

**GET** `/pipelines/{run_id}`

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `RunId` | path | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Current run state | application/json | [PipelineRun](../schemas/PipelineRun.md) |
| 404 |  | — | [NotFound](../responses/NotFound.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
operationId: getPipelineRun
parameters:
- $ref: "#/components/parameters/RunId"
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/PipelineRun"
    description: Current run state
  "404":
    $ref: "#/components/responses/NotFound"
summary: Poll pipeline run status (LRO)
```

</details>

