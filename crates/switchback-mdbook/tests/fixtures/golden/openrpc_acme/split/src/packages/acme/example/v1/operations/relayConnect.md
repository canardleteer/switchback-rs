# Open relay session (server SSE + client POST frames)

**relayConnect** (XRequestId, session_token, open) -> [RelayFrame](../schemas/RelayFrame.md)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | param | `string` | required | Caller correlation id echoed in logs and problem details. |
| `session_token` | param | `string` | required |  |
| `open` | param | [RelayOpen](../schemas/RelayOpen.md) | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| frame |  | — | [RelayFrame](../schemas/RelayFrame.md) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "relayConnect",
  "params": [
    {
      "$ref": "#/components/contentDescriptors/XRequestId"
    },
    {
      "name": "session_token",
      "required": true,
      "schema": {
        "type": "string"
      }
    },
    {
      "name": "open",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RelayOpen"
      }
    }
  ],
  "result": {
    "name": "frame",
    "schema": {
      "$ref": "#/components/schemas/RelayFrame"
    }
  },
  "summary": "Open relay session (server SSE + client POST frames)"
}
```

</details>

