# /echo

**POST** `/echo`

Round-trips a JSON payload for latency and auth testing.

Requires a valid **Bearer** token when `security` is enforced. Include
`x-request-id` on every call for trace correlation.


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | header | `string` | required | Caller correlation id echoed in logs and problem details. |
| `Authorization` | header | `string` | optional |  |

#### Request body

`application/json`: [EchoUnaryRequest](../schemas/EchoUnaryRequest.md) (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Echoed payload | application/json | [EchoUnaryResponse](../schemas/EchoUnaryResponse.md) |
| 400 |  | — | [BadRequest](../responses/BadRequest.md) |
| 401 |  | — | [Unauthorized](../responses/Unauthorized.md) |
| 403 |  | — | [Forbidden](../responses/Forbidden.md) |
| 404 |  | — | [NotFound](../responses/NotFound.md) |
| 429 |  | — | [TooManyRequests](../responses/TooManyRequests.md) |
| 500 |  | — | [InternalError](../responses/InternalError.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Round-trips a JSON payload for latency and auth testing.
  
  Requires a valid **Bearer** token when `security` is enforced. Include
  `x-request-id` on every call for trace correlation.
operationId: echoUnary
parameters:
- $ref: "#/components/parameters/XRequestId"
- $ref: "#/components/parameters/Authorization"
requestBody:
  content:
    application/json:
      schema:
        $ref: "#/components/schemas/EchoUnaryRequest"
  required: true
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/EchoUnaryResponse"
    description: Echoed payload
  "400":
    $ref: "#/components/responses/BadRequest"
  "401":
    $ref: "#/components/responses/Unauthorized"
  "403":
    $ref: "#/components/responses/Forbidden"
  "404":
    $ref: "#/components/responses/NotFound"
  "429":
    $ref: "#/components/responses/TooManyRequests"
  "500":
    $ref: "#/components/responses/InternalError"
summary: Echo a message (unary)
```

</details>

