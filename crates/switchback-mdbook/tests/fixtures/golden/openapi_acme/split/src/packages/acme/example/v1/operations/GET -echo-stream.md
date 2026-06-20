# /echo/stream

**GET** `/echo/stream` — response stream

Streams `chunk_count` echo fragments as **SSE** frames. Each `data` line
contains JSON matching `EchoStreamChunk`.

Tune `chunk_count` (default 3) to stress long-lived connections in docs
previews.


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | header | `string` | required | Caller correlation id echoed in logs and problem details. |
| `message` | query | `string` | required |  |
| `chunk_count` | query | `integer` | optional |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Server-sent echo chunks | text/event-stream | `200` |
| 401 |  | — | [Unauthorized](../responses/Unauthorized.md) |
| 429 |  | — | [TooManyRequests](../responses/TooManyRequests.md) |
| 500 |  | — | [InternalError](../responses/InternalError.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Streams `chunk_count` echo fragments as **SSE** frames. Each `data` line
  contains JSON matching `EchoStreamChunk`.
  
  Tune `chunk_count` (default 3) to stress long-lived connections in docs
  previews.
operationId: echoServerStream
parameters:
- $ref: "#/components/parameters/XRequestId"
- in: query
  name: message
  required: true
  schema:
    type: string
- in: query
  name: chunk_count
  schema:
    default: 3
    minimum: 1
    type: integer
responses:
  "200":
    content:
      text/event-stream:
        schema:
          description: |
            SSE frames carrying EchoStreamChunk JSON in `data` fields.
          type: string
    description: Server-sent echo chunks
  "401":
    $ref: "#/components/responses/Unauthorized"
  "429":
    $ref: "#/components/responses/TooManyRequests"
  "500":
    $ref: "#/components/responses/InternalError"
summary: Stream echo chunks (SSE)
```

</details>

