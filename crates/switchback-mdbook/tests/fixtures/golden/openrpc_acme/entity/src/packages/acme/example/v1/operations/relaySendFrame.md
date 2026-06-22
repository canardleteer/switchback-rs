# Send one inbound relay frame

**relaySendFrame** (XRequestId, session_token, frame) -> —

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | param | `string` | required | Caller correlation id echoed in logs and problem details. |
| `session_token` | param | `string` | required |  |
| `frame` | param | [RelayFrame](../schemas/RelayFrame.md) | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| accepted |  | — | — |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "relaySendFrame",
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
      "name": "frame",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RelayFrame"
      }
    }
  ],
  "result": {
    "name": "accepted",
    "schema": {
      "type": "boolean"
    }
  },
  "summary": "Send one inbound relay frame"
}
```

</details>

