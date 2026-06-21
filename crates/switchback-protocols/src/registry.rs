//! Protocol registry for encode/decode of attachments.

use switchback_codec_pb::canardleteer::switchback::protocol::amqp::v1alpha1::AmqpPayload;
use switchback_codec_pb::canardleteer::switchback::protocol::amqp::v1alpha1::__buffa::oneof::amqp_payload::Kind as AmqpKind;
use switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::GrpcPayload;
use switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::__buffa::oneof::grpc_payload::Kind as GrpcKind;
use switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::HttpPayload;
use switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::__buffa::oneof::http_payload::Kind as HttpKind;
use switchback_codec_pb::canardleteer::switchback::protocol::kafka::v1alpha1::KafkaPayload;
use switchback_codec_pb::canardleteer::switchback::protocol::kafka::v1alpha1::__buffa::oneof::kafka_payload::Kind as KafkaKind;
use switchback_codec_pb::canardleteer::switchback::protocol::mqtt::v1alpha1::MqttPayload;
use switchback_codec_pb::canardleteer::switchback::protocol::mqtt::v1alpha1::__buffa::oneof::mqtt_payload::Kind as MqttKind;
use switchback_traits::{ProtocolAttachment, Result, SwitchbackError};

use crate::amqp::AmqpProtocol;
use crate::grpc::GrpcProtocol;
use crate::http::HttpProtocol;
use crate::kafka::KafkaProtocol;
use crate::mqtt::MqttProtocol;
use crate::wire::decode_message;

/// Decoded HTTP payload oneof arm.
#[derive(Clone, Debug, PartialEq)]
pub enum HttpPayloadKind {
    /// Contract-level metadata.
    Contract(
        switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::HttpContractMeta,
    ),
    /// Operation invocation metadata.
    Operation(
        switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::HttpOperationMeta,
    ),
    /// Success response metadata.
    Response(
        switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::HttpResponseMeta,
    ),
    /// Error response metadata.
    Error(switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::HttpErrorMeta),
    /// Parameter metadata.
    Parameter(
        switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::HttpParameterMeta,
    ),
}

/// Decoded gRPC payload oneof arm.
#[derive(Clone, Debug, PartialEq)]
pub enum GrpcPayloadKind {
    /// Contract-level metadata.
    Contract(
        switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::GrpcContractMeta,
    ),
    /// Operation invocation metadata.
    Operation(
        switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::GrpcOperationMeta,
    ),
    /// Success status metadata.
    Status(switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::GrpcStatusMeta),
    /// Error metadata.
    Error(switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::GrpcErrorMeta),
    /// Metadata key.
    Metadata(
        switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::GrpcMetadataMeta,
    ),
}

/// Decoded Kafka payload oneof arm.
#[derive(Clone, Debug, PartialEq)]
pub enum KafkaPayloadKind {
    /// Contract-level metadata.
    Contract(
        switchback_codec_pb::canardleteer::switchback::protocol::kafka::v1alpha1::KafkaContractMeta,
    ),
    /// Channel metadata.
    Channel(
        switchback_codec_pb::canardleteer::switchback::protocol::kafka::v1alpha1::KafkaChannelMeta,
    ),
    /// Operation metadata.
    Operation(
        switchback_codec_pb::canardleteer::switchback::protocol::kafka::v1alpha1::KafkaOperationMeta,
    ),
    /// Message metadata.
    Message(
        switchback_codec_pb::canardleteer::switchback::protocol::kafka::v1alpha1::KafkaMessageMeta,
    ),
}

/// Decoded AMQP payload oneof arm.
#[derive(Clone, Debug, PartialEq)]
pub enum AmqpPayloadKind {
    /// Contract-level metadata.
    Contract(
        switchback_codec_pb::canardleteer::switchback::protocol::amqp::v1alpha1::AmqpContractMeta,
    ),
    /// Channel metadata.
    Channel(
        switchback_codec_pb::canardleteer::switchback::protocol::amqp::v1alpha1::AmqpChannelMeta,
    ),
    /// Operation metadata.
    Operation(
        switchback_codec_pb::canardleteer::switchback::protocol::amqp::v1alpha1::AmqpOperationMeta,
    ),
    /// Message metadata.
    Message(
        switchback_codec_pb::canardleteer::switchback::protocol::amqp::v1alpha1::AmqpMessageMeta,
    ),
}

/// Decoded MQTT payload oneof arm.
#[derive(Clone, Debug, PartialEq)]
pub enum MqttPayloadKind {
    /// Contract-level metadata.
    Contract(
        switchback_codec_pb::canardleteer::switchback::protocol::mqtt::v1alpha1::MqttContractMeta,
    ),
    /// Channel metadata.
    Channel(
        switchback_codec_pb::canardleteer::switchback::protocol::mqtt::v1alpha1::MqttChannelMeta,
    ),
    /// Operation metadata.
    Operation(
        switchback_codec_pb::canardleteer::switchback::protocol::mqtt::v1alpha1::MqttOperationMeta,
    ),
    /// Message metadata.
    Message(
        switchback_codec_pb::canardleteer::switchback::protocol::mqtt::v1alpha1::MqttMessageMeta,
    ),
}

/// Result of decoding a [`ProtocolAttachment`].
#[derive(Clone, Debug, PartialEq)]
pub enum DecodedAttachment {
    /// Known HTTP payload arm.
    Http(HttpPayloadKind),
    /// Known gRPC payload arm.
    Grpc(GrpcPayloadKind),
    /// Known Kafka payload arm.
    Kafka(KafkaPayloadKind),
    /// Known AMQP payload arm.
    Amqp(AmqpPayloadKind),
    /// Known MQTT payload arm.
    Mqtt(MqttPayloadKind),
    /// Unknown or custom protocol; bytes round-trip opaquely.
    Opaque {
        /// Protocol slug from the attachment envelope.
        protocol_id: String,
        /// Opaque payload bytes.
        payload: Vec<u8>,
    },
}

/// Registry of built-in protocol decoders.
#[derive(Clone, Debug, Default)]
pub struct ProtocolRegistry {
    http: HttpProtocol,
    grpc: GrpcProtocol,
    kafka: KafkaProtocol,
    amqp: AmqpProtocol,
    mqtt: MqttProtocol,
}

impl ProtocolRegistry {
    /// Built-in registry with `http`, `grpc`, `kafka`, `amqp`, and `mqtt` registered.
    pub fn with_builtins() -> Self {
        Self::default()
    }

    /// HTTP protocol implementation.
    pub fn http(&self) -> &HttpProtocol {
        &self.http
    }

    /// gRPC protocol implementation.
    pub fn grpc(&self) -> &GrpcProtocol {
        &self.grpc
    }

    /// Kafka protocol implementation.
    pub fn kafka(&self) -> &KafkaProtocol {
        &self.kafka
    }

    /// AMQP protocol implementation.
    pub fn amqp(&self) -> &AmqpProtocol {
        &self.amqp
    }

    /// MQTT protocol implementation.
    pub fn mqtt(&self) -> &MqttProtocol {
        &self.mqtt
    }

    /// Decode a protocol attachment envelope.
    ///
    /// Built-in ids deserialize to the matching [`DecodedAttachment`] variant;
    /// other ids return [`DecodedAttachment::Opaque`] with bytes unchanged.
    pub fn decode_attachment(&self, attachment: &ProtocolAttachment) -> Result<DecodedAttachment> {
        match attachment.protocol_id.as_str() {
            "http" => decode_http(&attachment.payload).map(DecodedAttachment::Http),
            "grpc" => decode_grpc(&attachment.payload).map(DecodedAttachment::Grpc),
            "kafka" => decode_kafka(&attachment.payload).map(DecodedAttachment::Kafka),
            "amqp" => decode_amqp(&attachment.payload).map(DecodedAttachment::Amqp),
            "mqtt" => decode_mqtt(&attachment.payload).map(DecodedAttachment::Mqtt),
            other => Ok(DecodedAttachment::Opaque {
                protocol_id: other.to_string(),
                payload: attachment.payload.clone(),
            }),
        }
    }

    /// Find the first HTTP operation meta on an operation body's attachments.
    pub fn http_operation_from_attachments(
        &self,
        protocols: &[ProtocolAttachment],
    ) -> Option<
        switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::HttpOperationMeta,
    > {
        for attachment in protocols {
            if let Ok(DecodedAttachment::Http(HttpPayloadKind::Operation(meta))) =
                self.decode_attachment(attachment)
            {
                return Some(meta);
            }
        }
        None
    }

    /// Find the first gRPC operation meta on an operation body's attachments.
    pub fn grpc_operation_from_attachments(
        &self,
        protocols: &[ProtocolAttachment],
    ) -> Option<
        switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::GrpcOperationMeta,
    > {
        for attachment in protocols {
            if let Ok(DecodedAttachment::Grpc(GrpcPayloadKind::Operation(meta))) =
                self.decode_attachment(attachment)
            {
                return Some(meta);
            }
        }
        None
    }
}

fn decode_http(bytes: &[u8]) -> Result<HttpPayloadKind> {
    let payload: HttpPayload = decode_message(bytes)?;
    match payload.kind {
        Some(HttpKind::Contract(v)) => Ok(HttpPayloadKind::Contract(*v)),
        Some(HttpKind::Operation(v)) => Ok(HttpPayloadKind::Operation(*v)),
        Some(HttpKind::Response(v)) => Ok(HttpPayloadKind::Response(*v)),
        Some(HttpKind::Error(v)) => Ok(HttpPayloadKind::Error(*v)),
        Some(HttpKind::Parameter(v)) => Ok(HttpPayloadKind::Parameter(*v)),
        None => Err(SwitchbackError::codec("empty HttpPayload")),
    }
}

fn decode_grpc(bytes: &[u8]) -> Result<GrpcPayloadKind> {
    let payload: GrpcPayload = decode_message(bytes)?;
    match payload.kind {
        Some(GrpcKind::Contract(v)) => Ok(GrpcPayloadKind::Contract(*v)),
        Some(GrpcKind::Operation(v)) => Ok(GrpcPayloadKind::Operation(*v)),
        Some(GrpcKind::Status(v)) => Ok(GrpcPayloadKind::Status(*v)),
        Some(GrpcKind::Error(v)) => Ok(GrpcPayloadKind::Error(*v)),
        Some(GrpcKind::Metadata(v)) => Ok(GrpcPayloadKind::Metadata(*v)),
        None => Err(SwitchbackError::codec("empty GrpcPayload")),
    }
}

fn decode_kafka(bytes: &[u8]) -> Result<KafkaPayloadKind> {
    let payload: KafkaPayload = decode_message(bytes)?;
    match payload.kind {
        Some(KafkaKind::Contract(v)) => Ok(KafkaPayloadKind::Contract(*v)),
        Some(KafkaKind::Channel(v)) => Ok(KafkaPayloadKind::Channel(*v)),
        Some(KafkaKind::Operation(v)) => Ok(KafkaPayloadKind::Operation(*v)),
        Some(KafkaKind::Message(v)) => Ok(KafkaPayloadKind::Message(*v)),
        None => Err(SwitchbackError::codec("empty KafkaPayload")),
    }
}

fn decode_amqp(bytes: &[u8]) -> Result<AmqpPayloadKind> {
    let payload: AmqpPayload = decode_message(bytes)?;
    match payload.kind {
        Some(AmqpKind::Contract(v)) => Ok(AmqpPayloadKind::Contract(*v)),
        Some(AmqpKind::Channel(v)) => Ok(AmqpPayloadKind::Channel(*v)),
        Some(AmqpKind::Operation(v)) => Ok(AmqpPayloadKind::Operation(*v)),
        Some(AmqpKind::Message(v)) => Ok(AmqpPayloadKind::Message(*v)),
        None => Err(SwitchbackError::codec("empty AmqpPayload")),
    }
}

fn decode_mqtt(bytes: &[u8]) -> Result<MqttPayloadKind> {
    let payload: MqttPayload = decode_message(bytes)?;
    match payload.kind {
        Some(MqttKind::Contract(v)) => Ok(MqttPayloadKind::Contract(*v)),
        Some(MqttKind::Channel(v)) => Ok(MqttPayloadKind::Channel(*v)),
        Some(MqttKind::Operation(v)) => Ok(MqttPayloadKind::Operation(*v)),
        Some(MqttKind::Message(v)) => Ok(MqttPayloadKind::Message(*v)),
        None => Err(SwitchbackError::codec("empty MqttPayload")),
    }
}

#[cfg(test)]
mod coverage_matrix {
    use super::*;
    use switchback_codec_pb::canardleteer::switchback::protocol::amqp::v1alpha1::{
        AmqpChannelMeta, AmqpContractMeta, AmqpMessageMeta, AmqpOperationMeta,
    };
    use switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::{
        GrpcContractMeta, GrpcErrorMeta, GrpcMetadataMeta, GrpcOperationMeta, GrpcStatusMeta,
    };
    use switchback_codec_pb::canardleteer::switchback::protocol::http::v1alpha1::{
        HttpContractMeta, HttpErrorMeta, HttpOperationMeta, HttpParameterMeta, HttpResponseMeta,
    };
    use switchback_codec_pb::canardleteer::switchback::protocol::kafka::v1alpha1::{
        KafkaChannelMeta, KafkaContractMeta, KafkaMessageMeta, KafkaOperationMeta,
    };
    use switchback_codec_pb::canardleteer::switchback::protocol::mqtt::v1alpha1::{
        MqttChannelMeta, MqttContractMeta, MqttMessageMeta, MqttOperationMeta,
    };

    #[test]
    fn http_matrix_roundtrips() {
        let registry = ProtocolRegistry::with_builtins();
        let http = registry.http();

        let cases: Vec<(HttpPayloadKind, ProtocolAttachment)> = vec![
            (
                HttpPayloadKind::Contract(HttpContractMeta {
                    default_server_url: "https://api.example.com".into(),
                    ..Default::default()
                }),
                http.attach_contract(&HttpContractMeta {
                    default_server_url: "https://api.example.com".into(),
                    ..Default::default()
                }),
            ),
            (
                HttpPayloadKind::Operation(HttpOperationMeta {
                    method: "GET".into(),
                    path_template: "/pets".into(),
                    ..Default::default()
                }),
                http.attach_operation(&HttpOperationMeta {
                    method: "GET".into(),
                    path_template: "/pets".into(),
                    ..Default::default()
                }),
            ),
            (
                HttpPayloadKind::Response(HttpResponseMeta {
                    status_code: 200,
                    ..Default::default()
                }),
                http.attach_response(&HttpResponseMeta {
                    status_code: 200,
                    ..Default::default()
                }),
            ),
            (
                HttpPayloadKind::Error(HttpErrorMeta {
                    status_code: 404,
                    ..Default::default()
                }),
                http.attach_error(&HttpErrorMeta {
                    status_code: 404,
                    ..Default::default()
                }),
            ),
            (
                HttpPayloadKind::Parameter(HttpParameterMeta {
                    name: "id".into(),
                    location: "path".into(),
                    required: true,
                    ..Default::default()
                }),
                http.attach_parameter(&HttpParameterMeta {
                    name: "id".into(),
                    location: "path".into(),
                    required: true,
                    ..Default::default()
                }),
            ),
        ];

        for (expected_kind, attachment) in cases {
            match registry.decode_attachment(&attachment).unwrap() {
                DecodedAttachment::Http(kind) => assert_eq!(kind, expected_kind),
                other => panic!("expected http decode, got {other:?}"),
            }
        }
    }

    #[test]
    fn grpc_matrix_roundtrips() {
        let registry = ProtocolRegistry::with_builtins();
        let grpc = registry.grpc();

        let cases: Vec<(GrpcPayloadKind, ProtocolAttachment)> = vec![
            (
                GrpcPayloadKind::Contract(GrpcContractMeta {
                    package_name: "acme.v1".into(),
                    ..Default::default()
                }),
                grpc.attach_contract(&GrpcContractMeta {
                    package_name: "acme.v1".into(),
                    ..Default::default()
                }),
            ),
            (
                GrpcPayloadKind::Operation(GrpcOperationMeta {
                    rpc_name: "GetPet".into(),
                    ..Default::default()
                }),
                grpc.attach_operation(&GrpcOperationMeta {
                    rpc_name: "GetPet".into(),
                    ..Default::default()
                }),
            ),
            (
                GrpcPayloadKind::Status(GrpcStatusMeta {
                    code: 0,
                    message: "OK".into(),
                    ..Default::default()
                }),
                grpc.attach_status(&GrpcStatusMeta {
                    code: 0,
                    message: "OK".into(),
                    ..Default::default()
                }),
            ),
            (
                GrpcPayloadKind::Error(GrpcErrorMeta {
                    code: 5,
                    message: "not found".into(),
                    ..Default::default()
                }),
                grpc.attach_error(&GrpcErrorMeta {
                    code: 5,
                    message: "not found".into(),
                    ..Default::default()
                }),
            ),
            (
                GrpcPayloadKind::Metadata(GrpcMetadataMeta {
                    key: "x-request-id".into(),
                    required: false,
                    ..Default::default()
                }),
                grpc.attach_metadata(&GrpcMetadataMeta {
                    key: "x-request-id".into(),
                    required: false,
                    ..Default::default()
                }),
            ),
        ];

        for (expected_kind, attachment) in cases {
            match registry.decode_attachment(&attachment).unwrap() {
                DecodedAttachment::Grpc(kind) => assert_eq!(kind, expected_kind),
                other => panic!("expected grpc decode, got {other:?}"),
            }
        }
    }

    #[test]
    fn kafka_matrix_roundtrips() {
        let registry = ProtocolRegistry::with_builtins();
        let kafka = registry.kafka();

        let cases: Vec<(KafkaPayloadKind, ProtocolAttachment)> = vec![
            (
                KafkaPayloadKind::Contract(KafkaContractMeta {
                    bootstrap_servers: vec!["kafka:9092".into()],
                    ..Default::default()
                }),
                kafka.attach_contract(&KafkaContractMeta {
                    bootstrap_servers: vec!["kafka:9092".into()],
                    ..Default::default()
                }),
            ),
            (
                KafkaPayloadKind::Channel(KafkaChannelMeta {
                    topic: "orders".into(),
                    partitions: 12,
                    replicas: 3,
                    ..Default::default()
                }),
                kafka.attach_channel(&KafkaChannelMeta {
                    topic: "orders".into(),
                    partitions: 12,
                    replicas: 3,
                    ..Default::default()
                }),
            ),
            (
                KafkaPayloadKind::Operation(KafkaOperationMeta {
                    group_id: "my-group".into(),
                    client_id: "my-client".into(),
                    ..Default::default()
                }),
                kafka.attach_operation(&KafkaOperationMeta {
                    group_id: "my-group".into(),
                    client_id: "my-client".into(),
                    ..Default::default()
                }),
            ),
            (
                KafkaPayloadKind::Message(KafkaMessageMeta {
                    schema_id_location: "payload".into(),
                    ..Default::default()
                }),
                kafka.attach_message(&KafkaMessageMeta {
                    schema_id_location: "payload".into(),
                    ..Default::default()
                }),
            ),
        ];

        for (expected_kind, attachment) in cases {
            match registry.decode_attachment(&attachment).unwrap() {
                DecodedAttachment::Kafka(kind) => assert_eq!(kind, expected_kind),
                other => panic!("expected kafka decode, got {other:?}"),
            }
        }
    }

    #[test]
    fn amqp_matrix_roundtrips() {
        let registry = ProtocolRegistry::with_builtins();
        let amqp = registry.amqp();

        let cases: Vec<(AmqpPayloadKind, ProtocolAttachment)> = vec![
            (
                AmqpPayloadKind::Contract(AmqpContractMeta {
                    default_vhost: "/events".into(),
                    ..Default::default()
                }),
                amqp.attach_contract(&AmqpContractMeta {
                    default_vhost: "/events".into(),
                    ..Default::default()
                }),
            ),
            (
                AmqpPayloadKind::Channel(AmqpChannelMeta {
                    channel_kind: "routingKey".into(),
                    exchange_name: "events".into(),
                    exchange_type: "topic".into(),
                    exchange_durable: true,
                    ..Default::default()
                }),
                amqp.attach_channel(&AmqpChannelMeta {
                    channel_kind: "routingKey".into(),
                    exchange_name: "events".into(),
                    exchange_type: "topic".into(),
                    exchange_durable: true,
                    ..Default::default()
                }),
            ),
            (
                AmqpPayloadKind::Operation(AmqpOperationMeta {
                    delivery_mode: 2,
                    priority: 5,
                    ..Default::default()
                }),
                amqp.attach_operation(&AmqpOperationMeta {
                    delivery_mode: 2,
                    priority: 5,
                    ..Default::default()
                }),
            ),
            (
                AmqpPayloadKind::Message(AmqpMessageMeta {
                    content_type: "application/json".into(),
                    ..Default::default()
                }),
                amqp.attach_message(&AmqpMessageMeta {
                    content_type: "application/json".into(),
                    ..Default::default()
                }),
            ),
        ];

        for (expected_kind, attachment) in cases {
            match registry.decode_attachment(&attachment).unwrap() {
                DecodedAttachment::Amqp(kind) => assert_eq!(kind, expected_kind),
                other => panic!("expected amqp decode, got {other:?}"),
            }
        }
    }

    #[test]
    fn mqtt_matrix_roundtrips() {
        let registry = ProtocolRegistry::with_builtins();
        let mqtt = registry.mqtt();

        let cases: Vec<(MqttPayloadKind, ProtocolAttachment)> = vec![
            (
                MqttPayloadKind::Contract(MqttContractMeta {
                    broker_urls: vec!["mqtt://broker:1883".into()],
                    ..Default::default()
                }),
                mqtt.attach_contract(&MqttContractMeta {
                    broker_urls: vec!["mqtt://broker:1883".into()],
                    ..Default::default()
                }),
            ),
            (
                MqttPayloadKind::Channel(MqttChannelMeta {
                    topic: "streetlights/1/0/event".into(),
                    ..Default::default()
                }),
                mqtt.attach_channel(&MqttChannelMeta {
                    topic: "streetlights/1/0/event".into(),
                    ..Default::default()
                }),
            ),
            (
                MqttPayloadKind::Operation(MqttOperationMeta {
                    qos: 2,
                    retain: true,
                    message_expiry_interval: 60,
                    ..Default::default()
                }),
                mqtt.attach_operation(&MqttOperationMeta {
                    qos: 2,
                    retain: true,
                    message_expiry_interval: 60,
                    ..Default::default()
                }),
            ),
            (
                MqttPayloadKind::Message(MqttMessageMeta {
                    response_topic: "application/responses".into(),
                    ..Default::default()
                }),
                mqtt.attach_message(&MqttMessageMeta {
                    response_topic: "application/responses".into(),
                    ..Default::default()
                }),
            ),
        ];

        for (expected_kind, attachment) in cases {
            match registry.decode_attachment(&attachment).unwrap() {
                DecodedAttachment::Mqtt(kind) => assert_eq!(kind, expected_kind),
                other => panic!("expected mqtt decode, got {other:?}"),
            }
        }
    }

    #[test]
    fn opaque_custom_protocol_passthrough() {
        let registry = ProtocolRegistry::with_builtins();
        let attachment = ProtocolAttachment {
            protocol_id: "acme/custom".into(),
            payload: vec![1, 2, 3],
        };
        match registry.decode_attachment(&attachment).unwrap() {
            DecodedAttachment::Opaque {
                protocol_id,
                payload,
            } => {
                assert_eq!(protocol_id, "acme/custom");
                assert_eq!(payload, vec![1, 2, 3]);
            }
            other => panic!("expected opaque, got {other:?}"),
        }
    }
}
