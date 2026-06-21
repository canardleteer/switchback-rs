# 16. AsyncAPI parser library MVP in switchback-asyncapi

Date: 2026-06-21

## Status

Proposed

Relates to
[17. Kafka, AMQP, and MQTT protocol wire packages](0017-kafka-amqp-and-mqtt-protocol-wire-packages.md)

Relates to
[18. Vendored Avro format meta-schemas in switchback-avro](0018-vendored-avro-format-meta-schemas-in-switchback-avro.md)

## Context

ADR 0008 pins per-family grouping; ADR 0011 pins HTTP/gRPC protocol attachments.
AsyncAPI MVP expands beyond a library-first stub: typed kafka/amqp/mqtt
bindings, Avro payload schemas via a new `switchback-avro` sibling to
`switchback-jsonschema`, multi-entry Acme corpus (v1/v2/v3alpha1), assembly in
`switchback-assemble`, and mdBook regression. Parser behavior has not started on
`main`; meta-schema integrity and publish metadata are landed.

## Decision

Implement `switchback-asyncapi` as a library-first parser per ADR 0007, with
prerequisites `switchback-avro` and kafka/amqp/mqtt wire packages (ADR 0017)
landed first.

**Versions:** AsyncAPI 2.x (through 2.6.x) and 3.x (3.0.x / 3.1.x / 3.x alpha).
Reject AsyncAPI 1.x at load. Preserve authored schema shape; no normalization
across spec versions.

**Supported protocols:** `AsyncApiFamily::supported_protocols()` returns
`["kafka", "amqp", "mqtt", "http", "websockets"]`. Built-in kafka/amqp/mqtt
populate via ADR 0017 wire packages; http/websockets use binding YAML fences
plus generic summarization until dedicated wire packages exist.

**Schema layers:** JSON Schema payloads →
`switchback-jsonschema::populate_schema_body`. Avro payloads (`schemaFormat`
`application/vnd.apache.avro+json` or `+yaml`) →
`switchback-avro::populate_avro_schema_body`. AsyncAPI document meta-schemas
stay in `switchback-asyncapi`; Avro **format** meta-schemas stay in
`switchback-avro` (ADR 0018).

**Grouping:** Per-entry group id (`acme.example.v1` / `v2` / `v3alpha1`) or app
`id` slug for single-entry fixtures; dedicated `components` group; tag-based
duplication mirroring OpenAPI (ADR 0010).

**Multi-entry:** `load_acme_example()` with `v1/asyncapi.yaml`,
`v2/asyncapi.yaml`, `v3alpha1/asyncapi.yaml` under `tests/fixtures/micro/acme/`.

**Assembly:** `switchback-assemble` loads AsyncAPI; extend
`examples/reference-manual/module.yaml` with asyncapi Acme slice (HTTP + gRPC +
events).

**Protobuf payload linking:** Message payloads referencing protobuf shapes
resolve structural refs to prefixed protobuf entity ids when FQN/schema id
aligns with Acme protobuf packages (ADR 0014).

**Acme corpus:** Hand-maintained micro fixtures reinterpreting Acme HTTP/gRPC
story as event APIs; v3alpha1 is primary Avro exercise with Kafka bindings on
pipeline channels.

**Publish order:** `switchback-avro` after `switchback-jsonschema`, before
`switchback-asyncapi`.

**Deferred:** CLI bin beyond stub, `xtask parse --parser asyncapi`,
`--validate`, prose `AsyncApiLinkExtractor` intra-links, AsyncAPI 1.x.

## Consequences

- Three new publishable crates or wire surfaces (`switchback-avro`,
  kafka/amqp/mqtt protos) extend CI spec-vendor and publish-check order.
- mdBook gains AsyncAPI render path, streetlights upstream + Acme goldens,
  protocol badge from attachments.
- `examples/mdbook-asyncapi` default `acme-api` book depends on Acme micro
  corpus.
- Golden output and link-check must stay green as protocol rendering adds
  detail.
