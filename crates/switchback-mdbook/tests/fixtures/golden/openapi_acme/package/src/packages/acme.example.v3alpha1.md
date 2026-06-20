# acme.example.v3alpha1

Alpha feature flags, experiments, and pipeline orchestration.



## Operations

### /experiments/{experiment_id}/assign

**POST** `/experiments/{experiment_id}/assign`

Assigns a subject to an experiment arm using deterministic hashing.

Supply `exclusion_keys` to respect mutual-exclusion groups across
concurrent experiments.


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `ExperimentId` | path | `string` | required |  |

#### Request body

`application/json`: [AssignExperimentRequest](#assignexperimentrequest) (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Assignment row | application/json | [ExperimentAssignment](#experimentassignment) |
| 404 |  | — | [NotFound](#notfound) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Assigns a subject to an experiment arm using deterministic hashing.
  
  Supply `exclusion_keys` to respect mutual-exclusion groups across
  concurrent experiments.
operationId: assignExperiment
parameters:
- $ref: "#/components/parameters/ExperimentId"
requestBody:
  content:
    application/json:
      schema:
        $ref: "#/components/schemas/AssignExperimentRequest"
  required: true
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/ExperimentAssignment"
    description: Assignment row
  "404":
    $ref: "#/components/responses/NotFound"
summary: Assign a subject to an experiment arm
```

</details>

### /experiments/{experiment_id}/assignments/stream

**GET** `/experiments/{experiment_id}/assignments/stream` — response stream

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `ExperimentId` | path | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Assignment events | text/event-stream | `200` |
| 404 |  | — | [NotFound](#notfound) |

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

### /feature-flags

**GET** `/feature-flags`

Lists feature flags for a tenant, optionally filtered by **release
channel**.

Alpha APIs may change without notice; pin clients to explicit flag keys.


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `tenant_id` | query | `string` | required | Tenant whose flags are listed. |
| `page_size` | query | `integer` | optional |  |
| `page_token` | query | `string` | optional |  |
| `channel` | query | [ReleaseChannel](#releasechannel) | optional |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Flag page | application/json | [ListFeatureFlagsResponse](#listfeatureflagsresponse) |
| 400 |  | — | [BadRequest](#badrequest) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Lists feature flags for a tenant, optionally filtered by **release
  channel**.
  
  Alpha APIs may change without notice; pin clients to explicit flag keys.
operationId: listFeatureFlags
parameters:
- description: Tenant whose flags are listed.
  in: query
  name: tenant_id
  required: true
  schema:
    type: string
- in: query
  name: page_size
  schema:
    default: 20
    type: integer
- in: query
  name: page_token
  schema:
    type: string
- in: query
  name: channel
  schema:
    $ref: "#/components/schemas/ReleaseChannel"
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/ListFeatureFlagsResponse"
    description: Flag page
  "400":
    $ref: "#/components/responses/BadRequest"
summary: List feature flags (paginated)
```

</details>

### /feature-flags

**PUT** `/feature-flags`

Creates or replaces a tenant-specific override for a single flag key.

Overrides take precedence over channel defaults until removed.


#### Request body

`application/json`: [FlagOverride](#flagoverride) (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Stored override | application/json | [FlagOverride](#flagoverride) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Creates or replaces a tenant-specific override for a single flag key.
  
  Overrides take precedence over channel defaults until removed.
operationId: upsertFlagOverride
requestBody:
  content:
    application/json:
      schema:
        $ref: "#/components/schemas/FlagOverride"
  required: true
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/FlagOverride"
    description: Stored override
summary: Upsert a tenant flag override
```

</details>

### /pipelines

**POST** `/pipelines`

Starts an asynchronous pipeline and returns a **long-running operation**
handle ([PipelineRun](#pipelinerun)).

Poll `GET /pipelines/{run_id}` or subscribe with the watch stream.


#### Request body

`application/json`: [StartPipelineRequest](#startpipelinerequest) (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 202 | Pipeline accepted | application/json | [PipelineRun](#pipelinerun) |
| 400 |  | — | [BadRequest](#badrequest) |

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

### /pipelines/{run_id}

**GET** `/pipelines/{run_id}`

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `RunId` | path | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Current run state | application/json | [PipelineRun](#pipelinerun) |
| 404 |  | — | [NotFound](#notfound) |

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

### /pipelines/{run_id}/watch

**GET** `/pipelines/{run_id}/watch` — response stream

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `RunId` | path | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Step result events | text/event-stream | `200` |
| 404 |  | — | [NotFound](#notfound) |

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

## Schemas

### AssignExperimentRequest

```yaml
properties:
  exclusion_keys:
    items:
      type: string
    type: array
  subject_id:
    type: string
required:
- subject_id
type: object
```

### ExperimentAssignment

```yaml
properties:
  arm:
    type: string
  assigned_at:
    format: date-time
    type: string
  experiment_id:
    format: uuid
    type: string
  subject_id:
    type: string
type: object
```

### FeatureFlag

```yaml
properties:
  channel:
    $ref: "#/components/schemas/ReleaseChannel"
  enabled:
    type: boolean
  key:
    type: string
type: object
```

### FlagOverride

```yaml
properties:
  enabled:
    type: boolean
  key:
    type: string
  tenant_id:
    type: string
required:
- tenant_id
- key
- enabled
type: object
```

### ListFeatureFlagsResponse

```yaml
properties:
  flags:
    items:
      $ref: "#/components/schemas/FeatureFlag"
    type: array
  page:
    $ref: "../v2/openapi.yaml#/components/schemas/PageResult"
type: object
```

### PipelineRun

```yaml
properties:
  completed_at:
    format: date-time
    type: string
  pipeline_name:
    type: string
  results:
    items:
      $ref: "#/components/schemas/PipelineStepResult"
    type: array
  run_id:
    format: uuid
    type: string
  started_at:
    format: date-time
    type: string
  status:
    $ref: "#/components/schemas/PipelineStatus"
type: object
```

### PipelineStatus

```yaml
enum:
- queued
- running
- succeeded
- failed
- cancelled
type: string
```

### PipelineStepResult

```yaml
properties:
  finished_at:
    format: date-time
    type: string
  status:
    $ref: "#/components/schemas/PipelineStatus"
  step_name:
    type: string
type: object
```

### Problem

```yaml
$ref: "../shared/schemas.yaml#/Problem"
```

### ReleaseChannel

```yaml
enum:
- stable
- beta
- canary
- internal
type: string
```

### StartPipelineRequest

```yaml
properties:
  actor:
    type: string
  pipeline_name:
    type: string
required:
- pipeline_name
type: object
```

## Parameters

### ExperimentId

Location: `path` (required)

```yaml
format: uuid
type: string
```

### RunId

Location: `path` (required)

```yaml
format: uuid
type: string
```

## Responses

### BadRequest

Status: `Bad request`

Media type: `application/problem+json`

Bad request

```yaml
content:
  application/problem+json:
    schema:
      $ref: "#/components/schemas/Problem"
description: Bad request
```

### NotFound

Status: `Not found`

Media type: `application/problem+json`

Not found

```yaml
content:
  application/problem+json:
    schema:
      $ref: "#/components/schemas/Problem"
description: Not found
```

