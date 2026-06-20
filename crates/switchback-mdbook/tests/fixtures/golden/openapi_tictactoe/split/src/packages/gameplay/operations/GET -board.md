# Get the whole board

**GET** `/board`

Retrieves the current state of the board and the winner.

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | OK | application/json | [status](../../components/schemas/status.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: Retrieves the current state of the board and the winner.
operationId: get-board
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/status"
    description: OK
security:
- defaultApiKey: []
- app2AppOauth:
  - board:read
summary: Get the whole board
tags:
- Gameplay
```

</details>

