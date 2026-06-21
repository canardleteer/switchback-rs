# Watch echo events (NDJSON)

****watchEcho**** `(XRequestId, topic) -> WatchEchoEvent`

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | param | `string` | required | Caller correlation id echoed in logs and problem details. |
| `topic` | param | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| event |  | — | [WatchEchoEvent](../schemas/WatchEchoEvent.md) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "watchEcho",
  "params": [
    {
      "$ref": "#/components/contentDescriptors/XRequestId"
    },
    {
      "name": "topic",
      "required": true,
      "schema": {
        "type": "string"
      }
    }
  ],
  "result": {
    "name": "event",
    "schema": {
      "$ref": "#/components/schemas/WatchEchoEvent"
    }
  },
  "summary": "Watch echo events (NDJSON)"
}
```

</details>

