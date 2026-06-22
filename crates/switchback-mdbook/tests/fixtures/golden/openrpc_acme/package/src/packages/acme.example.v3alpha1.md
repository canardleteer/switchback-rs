# acme.example.v3alpha1

Feature flags, experiments, and pipeline preview for the Acme fixture.



## Operations

### Assign an experiment cohort

**assignExperiment** (subject_id, experiment_id) -> [ExperimentAssignment](#experimentassignment)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `subject_id` | param | `string` | required |  |
| `experiment_id` | param | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| assignment |  | — | [ExperimentAssignment](#experimentassignment) |

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

### Get pipeline run status

**getPipelineRun** (run_id) -> [PipelineRun](#pipelinerun)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `run_id` | param | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| run |  | — | [PipelineRun](#pipelinerun) |

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

### List feature flags

**listFeatureFlags** (namespace) -> —

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `namespace` | param | `string` | optional |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| flags |  | — | — |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "listFeatureFlags",
  "params": [
    {
      "name": "namespace",
      "schema": {
        "type": "string"
      }
    }
  ],
  "result": {
    "name": "flags",
    "schema": {
      "items": {
        "$ref": "#/components/schemas/FeatureFlag"
      },
      "type": "array"
    }
  },
  "summary": "List feature flags"
}
```

</details>

### Start a pipeline run

**startPipeline** (pipeline) -> [PipelineRun](#pipelinerun)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `pipeline` | param | [PipelineSpec](#pipelinespec) | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| run |  | — | [PipelineRun](#pipelinerun) |

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

### Stream assignment updates

**streamAssignments** (experiment_id) -> [ExperimentAssignment](#experimentassignment)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `experiment_id` | param | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| assignment |  | — | [ExperimentAssignment](#experimentassignment) |

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

### Upsert a flag override

**upsertFlagOverride** (override) -> [FlagOverride](#flagoverride)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `override` | param | [FlagOverride](#flagoverride) | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| override |  | — | [FlagOverride](#flagoverride) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "upsertFlagOverride",
  "params": [
    {
      "name": "override",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/FlagOverride"
      }
    }
  ],
  "result": {
    "name": "override",
    "schema": {
      "$ref": "#/components/schemas/FlagOverride"
    }
  },
  "summary": "Upsert a flag override"
}
```

</details>

### Watch pipeline events

**watchPipeline** (run_id) -> [PipelineEvent](#pipelineevent)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `run_id` | param | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| event |  | — | [PipelineEvent](#pipelineevent) |

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

## Schemas

### ExperimentAssignment

```json
{
  "properties": {
    "cohort": {
      "type": "string"
    },
    "experiment_id": {
      "type": "string"
    },
    "subject_id": {
      "type": "string"
    }
  },
  "type": "object"
}
```

### FeatureFlag

```json
{
  "properties": {
    "enabled": {
      "type": "boolean"
    },
    "key": {
      "type": "string"
    }
  },
  "type": "object"
}
```

### FlagOverride

```json
{
  "properties": {
    "key": {
      "type": "string"
    },
    "subject_id": {
      "type": "string"
    },
    "value": {
      "type": "boolean"
    }
  },
  "type": "object"
}
```

### PipelineEvent

```json
{
  "properties": {
    "run_id": {
      "type": "string"
    },
    "state": {
      "type": "string"
    },
    "step": {
      "type": "string"
    }
  },
  "type": "object"
}
```

### PipelineRun

```json
{
  "properties": {
    "run_id": {
      "type": "string"
    },
    "status": {
      "type": "string"
    }
  },
  "type": "object"
}
```

### PipelineSpec

```json
{
  "properties": {
    "name": {
      "type": "string"
    },
    "steps": {
      "items": {
        "type": "string"
      },
      "type": "array"
    }
  },
  "type": "object"
}
```

### Problem

```json
{
  "$ref": "../shared/schemas.json#/Problem"
}
```

