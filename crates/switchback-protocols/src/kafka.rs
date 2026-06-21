//! Kafka protocol implementation.

use switchback_codec_pb::canardleteer::switchback::protocol::kafka::v1alpha1::{
    KafkaChannelMeta, KafkaContractMeta, KafkaMessageMeta, KafkaOperationMeta, KafkaPayload,
};
use switchback_codec_pb::canardleteer::switchback::protocol::kafka::v1alpha1::__buffa::oneof::kafka_payload::Kind;
use switchback_traits::ProtocolAttachment;

use crate::traits::Protocol;
use crate::wire::encode_message;

/// Built-in Kafka protocol (`"kafka"`).
#[derive(Clone, Copy, Debug, Default)]
pub struct KafkaProtocol;

impl Protocol for KafkaProtocol {
    fn id(&self) -> &'static str {
        "kafka"
    }
}

impl KafkaProtocol {
    /// Attach contract-level Kafka metadata.
    pub fn attach_contract(&self, meta: &KafkaContractMeta) -> ProtocolAttachment {
        attachment_from_payload(KafkaPayload {
            kind: Some(Kind::Contract(Box::new(meta.clone()))),
            ..Default::default()
        })
    }

    /// Attach channel-level Kafka metadata.
    pub fn attach_channel(&self, meta: &KafkaChannelMeta) -> ProtocolAttachment {
        attachment_from_payload(KafkaPayload {
            kind: Some(Kind::Channel(Box::new(meta.clone()))),
            ..Default::default()
        })
    }

    /// Attach operation-level Kafka metadata.
    pub fn attach_operation(&self, meta: &KafkaOperationMeta) -> ProtocolAttachment {
        attachment_from_payload(KafkaPayload {
            kind: Some(Kind::Operation(Box::new(meta.clone()))),
            ..Default::default()
        })
    }

    /// Attach message-level Kafka metadata.
    pub fn attach_message(&self, meta: &KafkaMessageMeta) -> ProtocolAttachment {
        attachment_from_payload(KafkaPayload {
            kind: Some(Kind::Message(Box::new(meta.clone()))),
            ..Default::default()
        })
    }
}

fn attachment_from_payload(payload: KafkaPayload) -> ProtocolAttachment {
    let protocol = KafkaProtocol;
    ProtocolAttachment {
        protocol_id: protocol.id().to_string(),
        payload: encode_message(&payload),
    }
}
