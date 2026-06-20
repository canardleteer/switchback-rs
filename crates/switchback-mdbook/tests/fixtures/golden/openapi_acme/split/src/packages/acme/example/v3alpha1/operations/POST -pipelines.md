# /pipelines

**POST** `/pipelines`

Starts an asynchronous pipeline and returns a **long-running operation**
handle ([PipelineRun](../schemas/PipelineRun.md)).

Poll `GET /pipelines/{run_id}` or subscribe with the watch stream.


#### Request body

`application/json`: [StartPipelineRequest](../schemas/StartPipelineRequest.md) (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 202 | Pipeline accepted | application/json | [PipelineRun](../schemas/PipelineRun.md) |
| 400 |  | — | [BadRequest](../responses/BadRequest.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Starts an asynchronous pipeline and returns a **long-running operation**
  handle (`PipelineRun`).
  
  Poll `GET /pipelines/{run_id}` or subscribe with the watch stream.
operationId: startPipeline
requestBody:
  content:
    application/json:
      schema:
        $ref: "#/components/schemas/StartPipelineRequest"
  required: true
responses:
  "202":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/PipelineRun"
    description: Pipeline accepted
  "400":
    $ref: "#/components/responses/BadRequest"
summary: Start a pipeline run (LRO handle)
```

</details>

