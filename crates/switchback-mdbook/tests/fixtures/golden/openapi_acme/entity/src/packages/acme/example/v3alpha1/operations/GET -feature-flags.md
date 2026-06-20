# /feature-flags

**GET** `/feature-flags`

Lists feature flags for a tenant, optionally filtered by **release
channel**.

Alpha APIs may change without notice; pin clients to explicit flag keys.


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `tenant_id` | query | `string` | required | Tenant whose flags are listed. |
| `page_size` | query | `integer` | optional |  |
| `page_token` | query | `string` | optional |  |
| `channel` | query | [ReleaseChannel](../schemas/ReleaseChannel.md) | optional |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Flag page | application/json | [ListFeatureFlagsResponse](../schemas/ListFeatureFlagsResponse.md) |
| 400 |  | — | [BadRequest](../responses/BadRequest.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Lists feature flags for a tenant, optionally filtered by **release
  channel**.
  
  Alpha APIs may change without notice; pin clients to explicit flag keys.
operationId: listFeatureFlags
parameters:
- description: Tenant whose flags are listed.
  in: query
  name: tenant_id
  required: true
  schema:
    type: string
- in: query
  name: page_size
  schema:
    default: 20
    type: integer
- in: query
  name: page_token
  schema:
    type: string
- in: query
  name: channel
  schema:
    $ref: "#/components/schemas/ReleaseChannel"
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/ListFeatureFlagsResponse"
    description: Flag page
  "400":
    $ref: "#/components/responses/BadRequest"
summary: List feature flags (paginated)
```

</details>

