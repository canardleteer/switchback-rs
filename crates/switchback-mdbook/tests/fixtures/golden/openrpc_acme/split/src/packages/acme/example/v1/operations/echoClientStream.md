# Upload echo parts (octet-stream)

**echoClientStream** (XRequestId, parts) -> [EchoUploadResponse](../schemas/EchoUploadResponse.md)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | param | `string` | required | Caller correlation id echoed in logs and problem details. |
| `parts` | param | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| response |  | — | [EchoUploadResponse](../schemas/EchoUploadResponse.md) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "echoClientStream",
  "params": [
    {
      "$ref": "#/components/contentDescriptors/XRequestId"
    },
    {
      "name": "parts",
      "required": true,
      "schema": {
        "contentMediaType": "application/octet-stream",
        "type": "string"
      }
    }
  ],
  "result": {
    "name": "response",
    "schema": {
      "$ref": "#/components/schemas/EchoUploadResponse"
    }
  },
  "summary": "Upload echo parts (octet-stream)"
}
```

</details>

