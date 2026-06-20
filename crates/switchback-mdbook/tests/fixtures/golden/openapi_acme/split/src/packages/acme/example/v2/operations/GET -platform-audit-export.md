# /platform/audit/export

**GET** `/platform/audit/export`

Returns a **paginated snapshot** of audit records for compliance
exports.

Callers must hold the `platform.audit.export` permission. Results are
eventually consistent with the live audit log (typically under 30
seconds).

## Pagination

Use `page_size` to limit rows per response. When `page.next_page_token`
is non-empty, repeat the request with that token until exhausted.

## Related types

- Response body: [ExportAuditBatchResponse](../schemas/ExportAuditBatchResponse.md)
- Nested batch: [AuditBatch](../schemas/AuditBatch.md)


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `tenant_id` | query | `string` | required | Tenant scope; must match the caller's organization. |
| `page_size` | query | `integer` | optional | Maximum records per page (default 50). |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Audit export page | application/json | [ExportAuditBatchResponse](../schemas/ExportAuditBatchResponse.md) |
| 403 |  | — | [Forbidden](../responses/Forbidden.md) |
| 500 |  | — | [InternalError](../responses/InternalError.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Returns a **paginated snapshot** of audit records for compliance
  exports.
  
  Callers must hold the `platform.audit.export` permission. Results are
  eventually consistent with the live audit log (typically under 30
  seconds).
  
  ## Pagination
  
  Use `page_size` to limit rows per response. When `page.next_page_token`
  is non-empty, repeat the request with that token until exhausted.
  
  ## Related types
  
  - Response body: `ExportAuditBatchResponse`
  - Nested batch: `AuditBatch`
operationId: exportAuditBatch
parameters:
- description: Tenant scope; must match the caller's organization.
  in: query
  name: tenant_id
  required: true
  schema:
    type: string
- description: Maximum records per page (default 50).
  in: query
  name: page_size
  schema:
    default: 50
    type: integer
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/ExportAuditBatchResponse"
    description: Audit export page
  "403":
    $ref: "#/components/responses/Forbidden"
  "500":
    $ref: "#/components/responses/InternalError"
summary: Export audit batch snapshot
```

</details>

