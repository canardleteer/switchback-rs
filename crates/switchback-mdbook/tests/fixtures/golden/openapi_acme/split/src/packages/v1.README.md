# Acme Example API v1

`acme.example.v1` — echo, streaming, and gateway surfaces for OpenAPI populate
and mdBook link tests.

## Echo

Unary echo, server-sent event streams, octet-stream uploads, and NDJSON watch
subscriptions mirror the protobuf `EchoService` RPC shapes as REST operations.

## Gateway relay

`POST /gateway/relay` models the server half of a relay session: the response is
`text/event-stream` (outbound frames). Clients send inbound frames with
follow-up `POST` requests carrying `RelayFrame` JSON bodies.

**OpenAPI gap:** bidirectional streaming (protobuf `RelayConnect` bidi) has no
first-class operation shape in OpenAPI 3.x. HTTP supports concurrent POST + SSE
just fine; the limitation is the **spec**, not the transport. Companion prose
documents the mapping; do not infer that relay is impossible over HTTP.

## Catalog proxy

`GET /catalog/products` proxies to v2 catalog types via external `$ref` into the
v2 entry document. See `MOVING-TO-V2.md` for migration notes.
