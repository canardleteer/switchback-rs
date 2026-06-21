# 17. Kafka, AMQP, and MQTT protocol wire packages

Date: 2026-06-21

## Status

Proposed

Relates to
[16. AsyncAPI parser library MVP in switchback-asyncapi](0016-asyncapi-parser-library-mvp-in-switchback-asyncapi.md)

Relates to
[11. Protocol layer and contract family binding](0011-protocol-layer-and-contract-family-binding.md)

## Context

ADR 0011 introduces `ProtocolAttachment` with HTTP and gRPC built-in packages.
AsyncAPI MVP requires typed kafka, amqp, and mqtt bindings on channels and
operations. Binding JSON Schemas live in asyncapi/spec-json-schemas
(`bindings/kafka`, `bindings/amqp`, `bindings/mqtt`).

## Decision

Add three protobuf packages under
`canardleteer.switchback.protocol.{kafka,amqp,mqtt}.v1alpha1` with built-in
protocol ids `"kafka"`, `"amqp"`, `"mqtt"`.

Each package defines a top-level `*Payload` oneof mirroring HTTP/gRPC:

| Package | Payload oneof arms | Primary AsyncAPI binding fields |
| --- | --- | --- |
| kafka | `contract`, `channel`, `operation`, `message` | `topic`, `partitions`, `replicas`; `groupId`, `clientId` |
| amqp | `contract`, `channel`, `operation`, `message` | `is` (queue/routingKey), `exchange.*`, `queue.*` |
| mqtt | `contract`, `channel`, `operation`, `message` | `topic` (channel), `qos`, `retain`, `messageExpiryInterval` |

**Entity attachment matrix (AsyncAPI populate):**

| IR node | kafka arm | amqp arm | mqtt arm |
| --- | --- | --- | --- |
| Contract | `KafkaContractMeta` (optional broker hints) | `AmqpContractMeta` | `MqttContractMeta` |
| ChannelBody | `KafkaChannelMeta` (`topic`, partition/replica counts) | `AmqpChannelMeta` | `MqttChannelMeta` (`topic`) |
| OperationBody | `KafkaOperationMeta` | `AmqpOperationMeta` | `MqttOperationMeta` |
| MessageBody | `KafkaMessageMeta` | `AmqpMessageMeta` | `MqttMessageMeta` |

`switchback-protocols` implements encode/decode and extends
`ProtocolRegistry::decode_attachment` with `DecodedAttachment::Kafka|Amqp|Mqtt`
arms. Binding versions targeted at populate: kafka `0.5.0`, amqp `0.3.0`, mqtt
`0.2.0` (latest in vendored asyncapi meta-schemas).

Non-built-in binding keys (NATS, Pub/Sub, Solace, etc.) remain YAML fence
summaries without typed attachments.

## Consequences

- Additive v1alpha1 wire in `switchback-codec-pb`; `switchback-protocols`
  round-trip tests mirror HTTP/gRPC matrix.
- AsyncAPI `populate/protocol_attach.rs` maps `bindings.kafka|amqp|mqtt` to
  attachments per matrix above.
- mdBook renders protocol labels from attachments for kafka/amqp/mqtt.
