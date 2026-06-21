//! MQTT protocol implementation.

use switchback_codec_pb::canardleteer::switchback::protocol::mqtt::v1alpha1::{
    MqttChannelMeta, MqttContractMeta, MqttMessageMeta, MqttOperationMeta, MqttPayload,
};
use switchback_codec_pb::canardleteer::switchback::protocol::mqtt::v1alpha1::__buffa::oneof::mqtt_payload::Kind;
use switchback_traits::ProtocolAttachment;

use crate::traits::Protocol;
use crate::wire::encode_message;

/// Built-in MQTT protocol (`"mqtt"`).
#[derive(Clone, Copy, Debug, Default)]
pub struct MqttProtocol;

impl Protocol for MqttProtocol {
    fn id(&self) -> &'static str {
        "mqtt"
    }
}

impl MqttProtocol {
    /// Attach contract-level MQTT metadata.
    pub fn attach_contract(&self, meta: &MqttContractMeta) -> ProtocolAttachment {
        attachment_from_payload(MqttPayload {
            kind: Some(Kind::Contract(Box::new(meta.clone()))),
            ..Default::default()
        })
    }

    /// Attach channel-level MQTT metadata.
    pub fn attach_channel(&self, meta: &MqttChannelMeta) -> ProtocolAttachment {
        attachment_from_payload(MqttPayload {
            kind: Some(Kind::Channel(Box::new(meta.clone()))),
            ..Default::default()
        })
    }

    /// Attach operation-level MQTT metadata.
    pub fn attach_operation(&self, meta: &MqttOperationMeta) -> ProtocolAttachment {
        attachment_from_payload(MqttPayload {
            kind: Some(Kind::Operation(Box::new(meta.clone()))),
            ..Default::default()
        })
    }

    /// Attach message-level MQTT metadata.
    pub fn attach_message(&self, meta: &MqttMessageMeta) -> ProtocolAttachment {
        attachment_from_payload(MqttPayload {
            kind: Some(Kind::Message(Box::new(meta.clone()))),
            ..Default::default()
        })
    }
}

fn attachment_from_payload(payload: MqttPayload) -> ProtocolAttachment {
    let protocol = MqttProtocol;
    ProtocolAttachment {
        protocol_id: protocol.id().to_string(),
        payload: encode_message(&payload),
    }
}
