# acme.example.v1

Echo, streaming, and gateway entry for the Acme documentation fixture.



## Operations

### Echo a message (unary)

**echoUnary**(`request`: [EchoUnaryRequest](#echounaryrequest), `XRequestId`: `string`, `Authorization`: `string`) -> [EchoUnaryResponse](#echounaryresponse)

Round-trips a JSON payload for latency and auth testing.

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `request` | param | [EchoUnaryRequest](#echounaryrequest) | required |  |
| `XRequestId` | param | `string` | required | Caller correlation id echoed in logs and problem details. |
| `Authorization` | param | `string` | optional |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| response |  | — | [EchoUnaryResponse](#echounaryresponse) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "description": "Round-trips a JSON payload for latency and auth testing.",
  "name": "echoUnary",
  "params": [
    {
      "name": "request",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/EchoUnaryRequest"
      }
    },
    {
      "$ref": "#/components/contentDescriptors/XRequestId"
    },
    {
      "$ref": "#/components/contentDescriptors/Authorization"
    }
  ],
  "result": {
    "name": "response",
    "schema": {
      "$ref": "#/components/schemas/EchoUnaryResponse"
    }
  },
  "summary": "Echo a message (unary)"
}
```

</details>

### Open relay session (server SSE + client POST frames)

**relayConnect**(`XRequestId`: `string`, `session_token`: `string`, `open`: [RelayOpen](#relayopen)) -> [RelayFrame](#relayframe)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | param | `string` | required | Caller correlation id echoed in logs and problem details. |
| `session_token` | param | `string` | required |  |
| `open` | param | [RelayOpen](#relayopen) | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| frame |  | — | [RelayFrame](#relayframe) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "relayConnect",
  "params": [
    {
      "$ref": "#/components/contentDescriptors/XRequestId"
    },
    {
      "name": "session_token",
      "required": true,
      "schema": {
        "type": "string"
      }
    },
    {
      "name": "open",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RelayOpen"
      }
    }
  ],
  "result": {
    "name": "frame",
    "schema": {
      "$ref": "#/components/schemas/RelayFrame"
    }
  },
  "summary": "Open relay session (server SSE + client POST frames)"
}
```

</details>

### Proxy list products (v2 catalog types)

**listCatalogProducts**(`XRequestId`: `string`, `Authorization`: `string`, `page_size`: `integer`) -> [ListProductsResponse](acme.example.v2.md#listproductsresponse)

Gateway proxy to the v2 catalog list operation.

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | param | `string` | required | Caller correlation id echoed in logs and problem details. |
| `Authorization` | param | `string` | optional |  |
| `page_size` | param | `integer` | optional |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| page |  | — | [ListProductsResponse](acme.example.v2.md#listproductsresponse) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "description": "Gateway proxy to the v2 catalog list operation.",
  "name": "listCatalogProducts",
  "params": [
    {
      "$ref": "#/components/contentDescriptors/XRequestId"
    },
    {
      "$ref": "#/components/contentDescriptors/Authorization"
    },
    {
      "name": "page_size",
      "schema": {
        "default": 20,
        "maximum": 100,
        "minimum": 1,
        "type": "integer"
      }
    }
  ],
  "result": {
    "name": "page",
    "schema": {
      "$ref": "../v2/openrpc.json#/components/schemas/ListProductsResponse"
    }
  },
  "summary": "Proxy list products (v2 catalog types)"
}
```

</details>

### Send one inbound relay frame

**relaySendFrame**(`XRequestId`: `string`, `session_token`: `string`, `frame`: [RelayFrame](#relayframe)) -> `boolean`

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | param | `string` | required | Caller correlation id echoed in logs and problem details. |
| `session_token` | param | `string` | required |  |
| `frame` | param | [RelayFrame](#relayframe) | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| accepted |  | — | `boolean` |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "relaySendFrame",
  "params": [
    {
      "$ref": "#/components/contentDescriptors/XRequestId"
    },
    {
      "name": "session_token",
      "required": true,
      "schema": {
        "type": "string"
      }
    },
    {
      "name": "frame",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RelayFrame"
      }
    }
  ],
  "result": {
    "name": "accepted",
    "schema": {
      "type": "boolean"
    }
  },
  "summary": "Send one inbound relay frame"
}
```

</details>

### Stream echo chunks (SSE)

**echoServerStream**(`XRequestId`: `string`, `message`: `string`, `chunk_count`: `integer`) -> [EchoStreamChunk](#echostreamchunk)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | param | `string` | required | Caller correlation id echoed in logs and problem details. |
| `message` | param | `string` | required |  |
| `chunk_count` | param | `integer` | optional |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| chunk |  | — | [EchoStreamChunk](#echostreamchunk) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "echoServerStream",
  "params": [
    {
      "$ref": "#/components/contentDescriptors/XRequestId"
    },
    {
      "name": "message",
      "required": true,
      "schema": {
        "type": "string"
      }
    },
    {
      "name": "chunk_count",
      "schema": {
        "default": 3,
        "minimum": 1,
        "type": "integer"
      }
    }
  ],
  "result": {
    "name": "chunk",
    "schema": {
      "$ref": "#/components/schemas/EchoStreamChunk"
    }
  },
  "summary": "Stream echo chunks (SSE)"
}
```

</details>

### Upload echo parts (octet-stream)

**echoClientStream**(`XRequestId`: `string`, `parts`: `string`) -> [EchoUploadResponse](#echouploadresponse)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | param | `string` | required | Caller correlation id echoed in logs and problem details. |
| `parts` | param | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| response |  | — | [EchoUploadResponse](#echouploadresponse) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "echoClientStream",
  "params": [
    {
      "$ref": "#/components/contentDescriptors/XRequestId"
    },
    {
      "name": "parts",
      "required": true,
      "schema": {
        "contentMediaType": "application/octet-stream",
        "type": "string"
      }
    }
  ],
  "result": {
    "name": "response",
    "schema": {
      "$ref": "#/components/schemas/EchoUploadResponse"
    }
  },
  "summary": "Upload echo parts (octet-stream)"
}
```

</details>

### Watch echo events (NDJSON)

**watchEcho**(`XRequestId`: `string`, `topic`: `string`) -> [WatchEchoEvent](#watchechoevent)

#### Parameters

| Name | In | Type | Required | Description |
| --- | --- | --- | --- | --- |
| `XRequestId` | param | `string` | required | Caller correlation id echoed in logs and problem details. |
| `topic` | param | `string` | required |  |

#### Responses

| Status | Description | Media type | Schema |
| --- | --- | --- | --- |
| event |  | — | [WatchEchoEvent](#watchechoevent) |

<details>
<summary>Operation definition (YAML)</summary>

```json
{
  "name": "watchEcho",
  "params": [
    {
      "$ref": "#/components/contentDescriptors/XRequestId"
    },
    {
      "name": "topic",
      "required": true,
      "schema": {
        "type": "string"
      }
    }
  ],
  "result": {
    "name": "event",
    "schema": {
      "$ref": "#/components/schemas/WatchEchoEvent"
    }
  },
  "summary": "Watch echo events (NDJSON)"
}
```

</details>

## Schemas

### EchoStreamChunk

```json
{
  "properties": {
    "payload": {
      "type": "string"
    },
    "sequence": {
      "type": "integer"
    }
  },
  "type": "object"
}
```

### EchoUnaryRequest

```json
{
  "properties": {
    "locale": {
      "type": "string"
    },
    "message": {
      "type": "string"
    },
    "tags": {
      "items": {
        "type": "string"
      },
      "type": "array"
    }
  },
  "required": [
    "message"
  ],
  "type": "object"
}
```

### EchoUnaryResponse

```json
{
  "properties": {
    "echoed_at": {
      "format": "date-time",
      "type": "string"
    },
    "message": {
      "type": "string"
    }
  },
  "type": "object"
}
```

### EchoUploadResponse

```json
{
  "properties": {
    "completed_at": {
      "format": "date-time",
      "type": "string"
    },
    "joined_message": {
      "type": "string"
    },
    "parts_received": {
      "type": "integer"
    }
  },
  "type": "object"
}
```

### Problem

```json
{
  "$ref": "../shared/schemas.json#/Problem"
}
```

### RelayFrame

```json
{
  "properties": {
    "fin": {
      "type": "boolean"
    },
    "payload": {
      "contentEncoding": "base64",
      "type": "string"
    },
    "sequence": {
      "format": "int64",
      "type": "integer"
    }
  },
  "type": "object"
}
```

### RelayOpen

```json
{
  "properties": {
    "session_name": {
      "type": "string"
    }
  },
  "required": [
    "session_name"
  ],
  "type": "object"
}
```

### WatchEchoEvent

```json
{
  "properties": {
    "event_id": {
      "type": "string"
    },
    "observed_at": {
      "format": "date-time",
      "type": "string"
    },
    "payload": {
      "$ref": "#/components/schemas/EchoUnaryResponse"
    }
  },
  "type": "object"
}
```

## Parameters

### Authorization

```json
{
  "type": "string"
}
```

### XRequestId

Caller correlation id echoed in logs and problem details.

```json
{
  "minLength": 1,
  "type": "string"
}
```

