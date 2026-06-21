//! AMQP protocol implementation.

use switchback_codec_pb::canardleteer::switchback::protocol::amqp::v1alpha1::{
    AmqpChannelMeta, AmqpContractMeta, AmqpMessageMeta, AmqpOperationMeta, AmqpPayload,
};
use switchback_codec_pb::canardleteer::switchback::protocol::amqp::v1alpha1::__buffa::oneof::amqp_payload::Kind;
use switchback_traits::ProtocolAttachment;

use crate::traits::Protocol;
use crate::wire::encode_message;

/// Built-in AMQP protocol (`"amqp"`).
#[derive(Clone, Copy, Debug, Default)]
pub struct AmqpProtocol;

impl Protocol for AmqpProtocol {
    fn id(&self) -> &'static str {
        "amqp"
    }
}

impl AmqpProtocol {
    /// Attach contract-level AMQP metadata.
    pub fn attach_contract(&self, meta: &AmqpContractMeta) -> ProtocolAttachment {
        attachment_from_payload(AmqpPayload {
            kind: Some(Kind::Contract(Box::new(meta.clone()))),
            ..Default::default()
        })
    }

    /// Attach channel-level AMQP metadata.
    pub fn attach_channel(&self, meta: &AmqpChannelMeta) -> ProtocolAttachment {
        attachment_from_payload(AmqpPayload {
            kind: Some(Kind::Channel(Box::new(meta.clone()))),
            ..Default::default()
        })
    }

    /// Attach operation-level AMQP metadata.
    pub fn attach_operation(&self, meta: &AmqpOperationMeta) -> ProtocolAttachment {
        attachment_from_payload(AmqpPayload {
            kind: Some(Kind::Operation(Box::new(meta.clone()))),
            ..Default::default()
        })
    }

    /// Attach message-level AMQP metadata.
    pub fn attach_message(&self, meta: &AmqpMessageMeta) -> ProtocolAttachment {
        attachment_from_payload(AmqpPayload {
            kind: Some(Kind::Message(Box::new(meta.clone()))),
            ..Default::default()
        })
    }
}

fn attachment_from_payload(payload: AmqpPayload) -> ProtocolAttachment {
    let protocol = AmqpProtocol;
    ProtocolAttachment {
        protocol_id: protocol.id().to_string(),
        payload: encode_message(&payload),
    }
}
