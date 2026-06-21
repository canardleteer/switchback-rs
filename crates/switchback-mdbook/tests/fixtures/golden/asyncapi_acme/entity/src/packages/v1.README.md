# Acme Events v1

`acme.example.v1` — echo and gateway **event** surfaces for AsyncAPI populate
and mdBook link tests.

## Echo

Unary echo and chunk streams mirror the OpenAPI `/echo` operations and protobuf
`EchoService` RPC shapes as publish/subscribe channels with MQTT and Kafka
bindings.

## Catalog proxy

`catalog/products` publishes list events whose payloads reference v2 catalog
schemas via external `$ref`.
