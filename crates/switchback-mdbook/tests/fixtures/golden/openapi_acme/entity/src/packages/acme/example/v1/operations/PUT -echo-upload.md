# /echo/upload

**PUT** `/echo/upload` — request stream

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | header | `string` | required | Caller correlation id echoed in logs and problem details. |

#### Request body

`application/octet-stream`: `string` (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Aggregated upload result | application/json | [EchoUploadResponse](../schemas/EchoUploadResponse.md) |
| 400 |  | — | [BadRequest](../responses/BadRequest.md) |
| 413 |  | — | [BadRequest](../responses/BadRequest.md) |
| 500 |  | — | [InternalError](../responses/InternalError.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
operationId: echoClientStream
parameters:
- $ref: "#/components/parameters/XRequestId"
requestBody:
  content:
    application/octet-stream:
      schema:
        contentMediaType: application/octet-stream
        type: string
  required: true
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/EchoUploadResponse"
    description: Aggregated upload result
  "400":
    $ref: "#/components/responses/BadRequest"
  "413":
    $ref: "#/components/responses/BadRequest"
  "500":
    $ref: "#/components/responses/InternalError"
summary: Upload echo parts (octet-stream)
```

</details>

