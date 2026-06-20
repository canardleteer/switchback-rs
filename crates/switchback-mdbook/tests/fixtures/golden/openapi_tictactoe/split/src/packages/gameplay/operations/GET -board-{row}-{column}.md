# /board/{row}/{column}

**GET** `/board/{row}/{column}`

Retrieves the requested square.

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| [rowParam](../../components/parameters/rowParam.md) | path | [coordinate](../../components/schemas/coordinate.md) | required | Board row (vertical coordinate) |
| [columnParam](../../components/parameters/columnParam.md) | path | [coordinate](../../components/schemas/coordinate.md) | required | Board column (horizontal coordinate) |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | OK | application/json | [mark](../../components/schemas/mark.md) |
| 400 | The provided parameters are incorrect | text/html | [errorMessage](../../components/schemas/errorMessage.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: Retrieves the requested square.
operationId: get-square
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/mark"
    description: OK
    links:
      markSquare:
        description: Operation to use if the mark value returned indicates the square is empty
        operationId: put-square
        parameters:
          column: $request.path.column
          row: $request.path.row
  "400":
    content:
      text/html:
        example: Illegal coordinates
        schema:
          $ref: "#/components/schemas/errorMessage"
    description: The provided parameters are incorrect
security:
- bearerHttpAuthentication: []
- user2AppOauth:
  - board:read
summary: Get a single board square
tags:
- Gameplay
```

</details>

