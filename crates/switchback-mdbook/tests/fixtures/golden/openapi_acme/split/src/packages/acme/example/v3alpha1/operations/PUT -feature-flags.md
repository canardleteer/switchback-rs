# /feature-flags

**PUT** `/feature-flags`

Creates or replaces a tenant-specific override for a single flag key.

Overrides take precedence over channel defaults until removed.


#### Request body

`application/json`: [FlagOverride](../schemas/FlagOverride.md) (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Stored override | application/json | [FlagOverride](../schemas/FlagOverride.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Creates or replaces a tenant-specific override for a single flag key.
  
  Overrides take precedence over channel defaults until removed.
operationId: upsertFlagOverride
requestBody:
  content:
    application/json:
      schema:
        $ref: "#/components/schemas/FlagOverride"
  required: true
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/FlagOverride"
    description: Stored override
summary: Upsert a tenant flag override
```

</details>

