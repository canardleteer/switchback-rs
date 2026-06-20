# /products/drafts

**PUT** `/products/drafts` — request stream

Uploads raw draft bytes for a product work-in-progress. The body is
`application/octet-stream`; metadata lives in query parameters only.

Useful for exercising **binary request bodies** in rendered docs.


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `draft_id` | query | `string` | required | Client-generated draft id; idempotent per draft. |

#### Request body

`application/octet-stream`: `string` (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Draft upload accepted | application/json | [UploadDraftsResponse](../schemas/UploadDraftsResponse.md) |
| 400 |  | — | [BadRequest](../responses/BadRequest.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Uploads raw draft bytes for a product work-in-progress. The body is
  `application/octet-stream`; metadata lives in query parameters only.
  
  Useful for exercising **binary request bodies** in rendered docs.
operationId: uploadDrafts
parameters:
- description: Client-generated draft id; idempotent per draft.
  in: query
  name: draft_id
  required: true
  schema:
    type: string
requestBody:
  content:
    application/octet-stream:
      schema:
        format: binary
        type: string
  required: true
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/UploadDraftsResponse"
    description: Draft upload accepted
  "400":
    $ref: "#/components/responses/BadRequest"
summary: Upload product draft bytes (octet-stream)
```

</details>

