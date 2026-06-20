# untagged

## Operations

### markStatus

**POST** `markStatus`

Provides the status of the mark operation on completion

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Mark operation has completed successfully | application/json | [status](components.md#status) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: Provides the status of the mark operation on completion
operationId: markOperationWebhook
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/status"
    description: Mark operation has completed successfully
summary: Status of mark operation
```

</details>

