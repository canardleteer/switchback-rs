# Acme Example API v1

`acme.example.v1` — echo, streaming, and gateway surfaces for OpenRPC populate
and mdBook link tests.

## Echo

Unary echo, server stream chunks, octet-stream uploads, and NDJSON watch
subscriptions mirror the protobuf `EchoService` RPC shapes as JSON-RPC methods.

## Gateway relay

`relayConnect` models the server half of a relay session: the result stream
carries outbound frames. Clients send inbound frames with follow-up
`relaySendFrame` calls.

## Catalog proxy

`listCatalogProducts` proxies to v2 catalog types via external `$ref` into the
v2 entry document. See `MOVING-TO-V2.md` for migration notes.
