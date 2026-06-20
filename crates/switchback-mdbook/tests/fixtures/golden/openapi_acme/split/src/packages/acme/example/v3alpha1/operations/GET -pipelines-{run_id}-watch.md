# /pipelines/{run_id}/watch

**GET** `/pipelines/{run_id}/watch` — response stream

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `RunId` | path | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Step result events | text/event-stream | `200` |
| 404 |  | — | [NotFound](../responses/NotFound.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
operationId: watchPipeline
parameters:
- $ref: "#/components/parameters/RunId"
responses:
  "200":
    content:
      text/event-stream:
        schema:
          type: string
    description: Step result events
  "404":
    $ref: "#/components/responses/NotFound"
summary: Watch pipeline step events (SSE)
```

</details>

