# gameplay

## Operations

### Get a single board square

**GET** `/board/{row}/{column}`

Retrieves the requested square.

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| [rowParam](components.md#rowparam) | path | [coordinate](components.md#coordinate) | required | Board row (vertical coordinate) |
| [columnParam](components.md#columnparam) | path | [coordinate](components.md#coordinate) | required | Board column (horizontal coordinate) |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | OK | application/json | [mark](components.md#mark) |
| 400 | The provided parameters are incorrect | text/html | [errorMessage](components.md#errormessage) |

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

### Get the whole board

**GET** `/board`

Retrieves the current state of the board and the winner.

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | OK | application/json | [status](components.md#status) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: Retrieves the current state of the board and the winner.
operationId: get-board
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/status"
    description: OK
security:
- defaultApiKey: []
- app2AppOauth:
  - board:read
summary: Get the whole board
tags:
- Gameplay
```

</details>

### Set a single board square

**PUT** `/board/{row}/{column}`

Places a mark on the board and retrieves the whole board and the winner (if any).

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| [rowParam](components.md#rowparam) | path | [coordinate](components.md#coordinate) | required | Board row (vertical coordinate) |
| [columnParam](components.md#columnparam) | path | [coordinate](components.md#coordinate) | required | Board column (horizontal coordinate) |
| `progressUrl` | header | `string` | optional | Progress URL that should be called if asynchronous response is returned |

#### Request body

`application/json`: [mark](components.md#mark) (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | OK | application/json | [status](components.md#status) |
| 202 | Mark operation has not completed. Use callback to check for progress | — | `202` |
| 400 | The provided parameters are incorrect | text/html | [errorMessage](components.md#errormessage) |

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

