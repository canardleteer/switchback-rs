# openapi.acme.example.v1

Echo, streaming, and gateway entry for the Acme documentation fixture.




## Operations

### /catalog/products

**GET** `/catalog/products`

Gateway proxy to the v2 catalog list operation. Response bodies use
**v2 schemas** even though the path lives under v1.

See `GET /products` in the v2 spec for pagination semantics.


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | header | `string` | required | Caller correlation id echoed in logs and problem details. |
| `Authorization` | header | `string` | optional |  |
| `page_size` | query | `integer` | optional |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Product page from v2 catalog schema | application/json | `ListProductsResponse` |
| 401 |  | — | [Unauthorized](#unauthorized) |
| 403 |  | — | [Forbidden](#forbidden) |
| 500 |  | — | [InternalError](#internalerror) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Gateway proxy to the v2 catalog list operation. Response bodies use
  **v2 schemas** even though the path lives under v1.
  
  See `GET /products` in the v2 spec for pagination semantics.
operationId: listCatalogProducts
parameters:
- $ref: "#/components/parameters/XRequestId"
- $ref: "#/components/parameters/Authorization"
- in: query
  name: page_size
  schema:
    default: 20
    maximum: 100
    minimum: 1
    type: integer
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "../v2/openapi.yaml#/components/schemas/ListProductsResponse"
    description: Product page from v2 catalog schema
  "401":
    $ref: "#/components/responses/Unauthorized"
  "403":
    $ref: "#/components/responses/Forbidden"
  "500":
    $ref: "#/components/responses/InternalError"
summary: Proxy list products (v2 catalog types)
```

</details>

### /echo

**POST** `/echo`

Round-trips a JSON payload for latency and auth testing.

Requires a valid **Bearer** token when `security` is enforced. Include
`x-request-id` on every call for trace correlation.


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | header | `string` | required | Caller correlation id echoed in logs and problem details. |
| `Authorization` | header | `string` | optional |  |

#### Request body

`application/json`: [EchoUnaryRequest](#echounaryrequest) (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Echoed payload | application/json | [EchoUnaryResponse](#echounaryresponse) |
| 400 |  | — | [BadRequest](#badrequest) |
| 401 |  | — | [Unauthorized](#unauthorized) |
| 403 |  | — | [Forbidden](#forbidden) |
| 404 |  | — | [NotFound](#notfound) |
| 429 |  | — | [TooManyRequests](#toomanyrequests) |
| 500 |  | — | [InternalError](#internalerror) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Round-trips a JSON payload for latency and auth testing.
  
  Requires a valid **Bearer** token when `security` is enforced. Include
  `x-request-id` on every call for trace correlation.
operationId: echoUnary
parameters:
- $ref: "#/components/parameters/XRequestId"
- $ref: "#/components/parameters/Authorization"
requestBody:
  content:
    application/json:
      schema:
        $ref: "#/components/schemas/EchoUnaryRequest"
  required: true
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/EchoUnaryResponse"
    description: Echoed payload
  "400":
    $ref: "#/components/responses/BadRequest"
  "401":
    $ref: "#/components/responses/Unauthorized"
  "403":
    $ref: "#/components/responses/Forbidden"
  "404":
    $ref: "#/components/responses/NotFound"
  "429":
    $ref: "#/components/responses/TooManyRequests"
  "500":
    $ref: "#/components/responses/InternalError"
summary: Echo a message (unary)
```

</details>

### /echo/stream

**GET** `/echo/stream` — response stream

Streams `chunk_count` echo fragments as **SSE** frames. Each `data` line
contains JSON matching `EchoStreamChunk`.

Tune `chunk_count` (default 3) to stress long-lived connections in docs
previews.


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | header | `string` | required | Caller correlation id echoed in logs and problem details. |
| `message` | query | `string` | required |  |
| `chunk_count` | query | `integer` | optional |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Server-sent echo chunks | text/event-stream | `200` |
| 401 |  | — | [Unauthorized](#unauthorized) |
| 429 |  | — | [TooManyRequests](#toomanyrequests) |
| 500 |  | — | [InternalError](#internalerror) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Streams `chunk_count` echo fragments as **SSE** frames. Each `data` line
  contains JSON matching `EchoStreamChunk`.
  
  Tune `chunk_count` (default 3) to stress long-lived connections in docs
  previews.
operationId: echoServerStream
parameters:
- $ref: "#/components/parameters/XRequestId"
- in: query
  name: message
  required: true
  schema:
    type: string
- in: query
  name: chunk_count
  schema:
    default: 3
    minimum: 1
    type: integer
responses:
  "200":
    content:
      text/event-stream:
        schema:
          description: |
            SSE frames carrying EchoStreamChunk JSON in `data` fields.
          type: string
    description: Server-sent echo chunks
  "401":
    $ref: "#/components/responses/Unauthorized"
  "429":
    $ref: "#/components/responses/TooManyRequests"
  "500":
    $ref: "#/components/responses/InternalError"
summary: Stream echo chunks (SSE)
```

</details>

### /echo/upload

**PUT** `/echo/upload` — request stream

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | header | `string` | required | Caller correlation id echoed in logs and problem details. |

#### Request body

`application/octet-stream`: `string` (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Aggregated upload result | application/json | [EchoUploadResponse](#echouploadresponse) |
| 400 |  | — | [BadRequest](#badrequest) |
| 413 |  | — | [BadRequest](#badrequest) |
| 500 |  | — | [InternalError](#internalerror) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
operationId: echoClientStream
parameters:
- $ref: "#/components/parameters/XRequestId"
requestBody:
  content:
    application/octet-stream:
      schema:
        contentMediaType: application/octet-stream
        type: string
  required: true
responses:
  "200":
    content:
      application/json:
        schema:
          $ref: "#/components/schemas/EchoUploadResponse"
    description: Aggregated upload result
  "400":
    $ref: "#/components/responses/BadRequest"
  "413":
    $ref: "#/components/responses/BadRequest"
  "500":
    $ref: "#/components/responses/InternalError"
summary: Upload echo parts (octet-stream)
```

</details>

### /echo/watch

**GET** `/echo/watch` — response stream

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | header | `string` | required | Caller correlation id echoed in logs and problem details. |
| `topic` | query | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Newline-delimited echo events | application/x-ndjson | [WatchEchoEvent](#watchechoevent) |
| 404 |  | — | [NotFound](#notfound) |
| 500 |  | — | [InternalError](#internalerror) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
operationId: watchEcho
parameters:
- $ref: "#/components/parameters/XRequestId"
- in: query
  name: topic
  required: true
  schema:
    type: string
responses:
  "200":
    content:
      application/x-ndjson:
        schema:
          $ref: "#/components/schemas/WatchEchoEvent"
    description: Newline-delimited echo events
  "404":
    $ref: "#/components/responses/NotFound"
  "500":
    $ref: "#/components/responses/InternalError"
summary: Watch echo events (NDJSON)
```

</details>

### /gateway/relay

**POST** `/gateway/relay` — response stream

Returns outbound relay frames as SSE. Send inbound frames via separate POST requests; see v1 README for the OpenAPI bidi modeling gap.


#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | header | `string` | required | Caller correlation id echoed in logs and problem details. |
| `x-session-token` | header | `string` | required |  |

#### Request body

`application/json`: [RelayOpen](#relayopen) (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 200 | Relay outbound frame stream | text/event-stream | `200` |
| 403 |  | — | [Forbidden](#forbidden) |
| 500 |  | — | [InternalError](#internalerror) |

<details>
<summary>Operation definition (YAML)</summary>

```yaml
description: |
  Returns outbound relay frames as SSE. Send inbound frames via separate POST requests; see v1 README for the OpenAPI bidi modeling gap.
operationId: relayConnect
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
        $ref: "#/components/schemas/RelayOpen"
  required: true
responses:
  "200":
    content:
      text/event-stream:
        schema:
          type: string
    description: Relay outbound frame stream
  "403":
    $ref: "#/components/responses/Forbidden"
  "500":
    $ref: "#/components/responses/InternalError"
summary: Open relay session (server SSE + client POST frames)
```

</details>

### /gateway/relay/frames

**POST** `/gateway/relay/frames`

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | header | `string` | required | Caller correlation id echoed in logs and problem details. |
| `x-session-token` | header | `string` | required |  |

#### Request body

`application/json`: [RelayFrame](#relayframe) (required)


#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| 202 | Frame accepted | — | `202` |
| 404 |  | — | [NotFound](#notfound) |

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

## Schemas

### EchoUnaryRequest

```yaml
properties:
  locale:
    type: string
  message:
    type: string
  tags:
    items:
      type: string
    type: array
required:
- message
type: object
```

### EchoUnaryResponse

```yaml
properties:
  echoed_at:
    format: date-time
    type: string
  message:
    type: string
type: object
```

### EchoUploadResponse

```yaml
properties:
  completed_at:
    format: date-time
    type: string
  joined_message:
    type: string
  parts_received:
    type: integer
type: object
```

### Problem

```yaml
$ref: "../shared/schemas.yaml#/Problem"
```

### RelayFrame

```yaml
properties:
  fin:
    type: boolean
  payload:
    contentEncoding: base64
    type: string
  sequence:
    format: int64
    type: integer
type: object
```

### RelayOpen

```yaml
properties:
  session_name:
    type: string
required:
- session_name
type: object
```

### WatchEchoEvent

```yaml
properties:
  event_id:
    type: string
  observed_at:
    format: date-time
    type: string
  payload:
    $ref: "#/components/schemas/EchoUnaryResponse"
type: object
```

## Parameters

### Authorization

Location: `header` (optional)

```yaml
type: string
```

### XRequestId

Location: `header` (required)

Caller correlation id echoed in logs and problem details.

```yaml
minLength: 1
type: string
```

## Responses

### BadRequest

Status: `Bad request`

Media type: `application/problem+json`

Bad request

```yaml
content:
  application/problem+json:
    schema:
      $ref: "#/components/schemas/Problem"
description: Bad request
```

### Forbidden

Status: `Authenticated but not permitted`

Media type: `application/problem+json`

Authenticated but not permitted

```yaml
content:
  application/problem+json:
    schema:
      $ref: "#/components/schemas/Problem"
description: Authenticated but not permitted
```

### InternalError

Status: `Unexpected server error`

Media type: `application/problem+json`

Unexpected server error

```yaml
content:
  application/problem+json:
    schema:
      $ref: "#/components/schemas/Problem"
description: Unexpected server error
```

### NotFound

Status: `Resource not found`

Media type: `application/problem+json`

Resource not found

```yaml
content:
  application/problem+json:
    schema:
      $ref: "#/components/schemas/Problem"
description: Resource not found
```

### TooManyRequests

Status: `Rate limit exceeded`

Media type: `application/problem+json`

Rate limit exceeded

```yaml
content:
  application/problem+json:
    schema:
      $ref: "#/components/schemas/Problem"
description: Rate limit exceeded
```

### Unauthorized

Status: `Missing or invalid credentials`

Media type: `application/problem+json`

Missing or invalid credentials

```yaml
content:
  application/problem+json:
    schema:
      $ref: "#/components/schemas/Problem"
description: Missing or invalid credentials
```

## Security schemes

### bearerAuth

Type: `http`

```yaml
bearerFormat: JWT
scheme: bearer
type: http
```

