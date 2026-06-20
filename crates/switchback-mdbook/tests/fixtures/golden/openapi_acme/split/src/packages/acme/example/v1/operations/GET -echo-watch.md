# /echo/watch

**GET** `/echo/watch` — response stream

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | header | `string` | required | Caller correlation id echoed in logs and problem details. |
| `topic` | query | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Newline-delimited echo events | application/x-ndjson | [WatchEchoEvent](../schemas/WatchEchoEvent.md) |
| 404 |  | — | [NotFound](../responses/NotFound.md) |
| 500 |  | — | [InternalError](../responses/InternalError.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
operationId: watchEcho
parameters:
- $ref: "#/components/parameters/XRequestId"
- in: query
  name: topic
  required: true
  schema:
    type: string
responses:
  "200":
    content:
      application/x-ndjson:
        schema:
          $ref: "#/components/schemas/WatchEchoEvent"
    description: Newline-delimited echo events
  "404":
    $ref: "#/components/responses/NotFound"
  "500":
    $ref: "#/components/responses/InternalError"
summary: Watch echo events (NDJSON)
```

</details>

