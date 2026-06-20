# /gateway/relay/frames

**POST** `/gateway/relay/frames`

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | header | `string` | required | Caller correlation id echoed in logs and problem details. |
| `x-session-token` | header | `string` | required |  |

#### Request body

`application/json`: [RelayFrame](../schemas/RelayFrame.md) (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 202 | Frame accepted | — | `202` |
| 404 |  | — | [NotFound](../responses/NotFound.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
operationId: relaySendFrame
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
        $ref: "#/components/schemas/RelayFrame"
  required: true
responses:
  "202":
    description: Frame accepted
  "404":
    $ref: "#/components/responses/NotFound"
summary: Send one inbound relay frame
```

</details>

