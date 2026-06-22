# Echo a message (unary)

**echoUnary**(`request`: [EchoUnaryRequest](../schemas/EchoUnaryRequest.md), `XRequestId`: `string`, `Authorization`: `string`) -> [EchoUnaryResponse](../schemas/EchoUnaryResponse.md)

Round-trips a JSON payload for latency and auth testing.

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `request` | param | [EchoUnaryRequest](../schemas/EchoUnaryRequest.md) | required |  |
| `XRequestId` | param | `string` | required | Caller correlation id echoed in logs and problem details. |
| `Authorization` | param | `string` | optional |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| response |  | — | [EchoUnaryResponse](../schemas/EchoUnaryResponse.md) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "description": "Round-trips a JSON payload for latency and auth testing.",
  "name": "echoUnary",
  "params": [
    {
      "name": "request",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/EchoUnaryRequest"
      }
    },
    {
      "$ref": "#/components/contentDescriptors/XRequestId"
    },
    {
      "$ref": "#/components/contentDescriptors/Authorization"
    }
  ],
  "result": {
    "name": "response",
    "schema": {
      "$ref": "#/components/schemas/EchoUnaryResponse"
    }
  },
  "summary": "Echo a message (unary)"
}
```

</details>

