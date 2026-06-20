# components

## Schemas

### board

```yaml
items:
  items:
    $ref: "#/components/schemas/mark"
  maxItems: 3
  minItems: 3
  type: array
maxItems: 3
minItems: 3
type: array
```

### coordinate

```yaml
example: 1
maximum: 3
minimum: 1
type: integer
```

### errorMessage

A text message describing an error

```yaml
description: A text message describing an error
maxLength: 256
type: string
```

### mark

Possible values for a board square. `.` means empty square.

```yaml
description: Possible values for a board square. `.` means empty square.
enum:
- '.'
- X
- O
example: '.'
type: string
```

### status

```yaml
properties:
  board:
    $ref: "#/components/schemas/board"
  winner:
    $ref: "#/components/schemas/winner"
type: object
```

### winner

Winner of the game. `.` means nobody has won yet.

```yaml
description: Winner of the game. `.` means nobody has won yet.
enum:
- '.'
- X
- O
example: '.'
type: string
```

## Parameters

### columnParam

Location: `path` (required)

Board column (horizontal coordinate)

```yaml
$ref: "#/components/schemas/coordinate"
```

### rowParam

Location: `path` (required)

Board row (vertical coordinate)

```yaml
$ref: "#/components/schemas/coordinate"
```

## Security schemes

### app2AppOauth

Type: `oauth2`

```yaml
flows:
  clientCredentials:
    scopes:
      "board:read": Read the board
    tokenUrl: https://learn.openapis.org/oauth/2.0/token
type: oauth2
```

### basicHttpAuthentication

Type: `http`

Basic HTTP Authentication

```yaml
description: Basic HTTP Authentication
scheme: Basic
type: http
```

### bearerHttpAuthentication

Type: `http`

Bearer token using a JWT

```yaml
bearerFormat: JWT
description: Bearer token using a JWT
scheme: Bearer
type: http
```

### defaultApiKey

Type: `apiKey`

API key provided in console

```yaml
description: API key provided in console
in: header
name: api-key
type: apiKey
```

### user2AppOauth

Type: `oauth2`

```yaml
flows:
  authorizationCode:
    authorizationUrl: https://learn.openapis.org/oauth/2.0/auth
    scopes:
      "board:read": Read the board
      "board:write": Write to the board
    tokenUrl: https://learn.openapis.org/oauth/2.0/token
type: oauth2
```

