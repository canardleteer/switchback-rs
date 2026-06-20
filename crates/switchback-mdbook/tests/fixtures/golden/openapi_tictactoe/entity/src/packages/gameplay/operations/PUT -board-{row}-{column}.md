# /board/{row}/{column}

**PUT** `/board/{row}/{column}`

Places a mark on the board and retrieves the whole board and the winner (if any).

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| [rowParam](../../components/parameters/rowParam.md) | path | [coordinate](../../components/schemas/coordinate.md) | required | Board row (vertical coordinate) |
| [columnParam](../../components/parameters/columnParam.md) | path | [coordinate](../../components/schemas/coordinate.md) | required | Board column (horizontal coordinate) |
| `progressUrl` | header | `string` | optional | Progress URL that should be called if asynchronous response is returned |

#### Request body

`application/json`: [mark](../../components/schemas/mark.md) (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | OK | application/json | [status](../../components/schemas/status.md) |
| 202 | Mark operation has not completed. Use callback to check for progress | — | `202` |
| 400 | The provided parameters are incorrect | text/html | [errorMessage](../../components/schemas/errorMessage.md) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
callbacks:
  statusCallback:
    "{$request.header.progressUrl}":
      post:
        description: Provides the status of the mark operation
        operationId: markOperationCallback
        requestBody:
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/status"
        responses:
          "200":
            description: Mark operation status received
        summary: Status of mark operation
description: >-
  Places a mark on the board and retrieves the whole board and the winner (if
  any).
operationId: put-square
parameters:
- description: Progress URL that should be called if asynchronous response is returned
  in: header
  name: progressUrl
  required: false
  schema:
    type: string
requestBody:
  content:
    application/json:
      schema:
        $ref: "#/components/schemas/mark"
  required: true
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/status"
    description: OK
  "202":
    description: Mark operation has not completed. Use callback to check for progress
    headers:
      Location:
        description: Callback URL
        schema:
          type: string
  "400":
    content:
      text/html:
        examples:
          illegalCoordinates:
            value: Illegal coordinates.
          invalidMark:
            value: Invalid Mark (X or O).
          notEmpty:
            value: Square is not empty.
        schema:
          $ref: "#/components/schemas/errorMessage"
    description: The provided parameters are incorrect
security:
- bearerHttpAuthentication: []
- user2AppOauth:
  - board:write
summary: Set a single board square
tags:
- Gameplay
```

</details>

