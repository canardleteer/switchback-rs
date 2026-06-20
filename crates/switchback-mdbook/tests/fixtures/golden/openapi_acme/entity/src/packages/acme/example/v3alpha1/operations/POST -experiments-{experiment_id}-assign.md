# /experiments/{experiment_id}/assign

**POST** `/experiments/{experiment_id}/assign`

Assigns a subject to an experiment arm using deterministic hashing.

Supply `exclusion_keys` to respect mutual-exclusion groups across
concurrent experiments.


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `ExperimentId` | path | `string` | required |  |

#### Request body

`application/json`: [AssignExperimentRequest](../schemas/AssignExperimentRequest.md) (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Assignment row | application/json | [ExperimentAssignment](../schemas/ExperimentAssignment.md) |
| 404 |  | — | [NotFound](../responses/NotFound.md) |

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

