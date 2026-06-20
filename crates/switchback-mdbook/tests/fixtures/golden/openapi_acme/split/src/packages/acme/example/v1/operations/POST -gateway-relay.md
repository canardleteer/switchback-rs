# /gateway/relay

**POST** `/gateway/relay` — response stream

Returns outbound relay frames as SSE. Send inbound frames via separate POST requests; see v1 README for the OpenAPI bidi modeling gap.


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | header | `string` | required | Caller correlation id echoed in logs and problem details. |
| `x-session-token` | header | `string` | required |  |

#### Request body

`application/json`: [RelayOpen](../schemas/RelayOpen.md) (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Relay outbound frame stream | text/event-stream | `200` |
| 403 |  | — | [Forbidden](../responses/Forbidden.md) |
| 500 |  | — | [InternalError](../responses/InternalError.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Returns outbound relay frames as SSE. Send inbound frames via separate POST requests; see v1 README for the OpenAPI bidi modeling gap.
operationId: relayConnect
parameters:
- $ref: "#/components/parameters/XRequestId"
- in: header
  name: x-session-token
  required: true
  schema:
    type: string
requestBody:
  content:
    application/json:
      schema:
        $ref: "#/components/schemas/RelayOpen"
  required: true
responses:
  "200":
    content:
      text/event-stream:
        schema:
          type: string
    description: Relay outbound frame stream
  "403":
    $ref: "#/components/responses/Forbidden"
  "500":
    $ref: "#/components/responses/InternalError"
summary: Open relay session (server SSE + client POST frames)
```

</details>

