# Stream echo chunks (SSE)

**echoServerStream**(`XRequestId`: `string`, `message`: `string`, `chunk_count`: `integer`) -> [EchoStreamChunk](../schemas/EchoStreamChunk.md)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | param | `string` | required | Caller correlation id echoed in logs and problem details. |
| `message` | param | `string` | required |  |
| `chunk_count` | param | `integer` | optional |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| chunk |  | — | [EchoStreamChunk](../schemas/EchoStreamChunk.md) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "echoServerStream",
  "params": [
    {
      "$ref": "#/components/contentDescriptors/XRequestId"
    },
    {
      "name": "message",
      "required": true,
      "schema": {
        "type": "string"
      }
    },
    {
      "name": "chunk_count",
      "schema": {
        "default": 3,
        "minimum": 1,
        "type": "integer"
      }
    }
  ],
  "result": {
    "name": "chunk",
    "schema": {
      "$ref": "#/components/schemas/EchoStreamChunk"
    }
  },
  "summary": "Stream echo chunks (SSE)"
}
```

</details>

