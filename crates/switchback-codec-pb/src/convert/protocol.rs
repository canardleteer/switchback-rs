//! Protocol attachment mapping.

use switchback_traits::ProtocolAttachment;

use crate::pb;

pub fn protocol_attachment_to_proto(attachment: &ProtocolAttachment) -> pb::ProtocolAttachment {
    pb::ProtocolAttachment {
        protocol_id: attachment.protocol_id.clone(),
        payload: attachment.payload.clone(),
        ..Default::default()
    }
}

pub fn protocol_attachment_from_proto(attachment: pb::ProtocolAttachment) -> ProtocolAttachment {
    ProtocolAttachment {
        protocol_id: attachment.protocol_id,
        payload: attachment.payload,
    }
}

pub fn protocols_to_proto(protocols: &[ProtocolAttachment]) -> Vec<pb::ProtocolAttachment> {
    protocols.iter().map(protocol_attachment_to_proto).collect()
}

pub fn protocols_from_proto(protocols: Vec<pb::ProtocolAttachment>) -> Vec<ProtocolAttachment> {
    protocols
        .into_iter()
        .map(protocol_attachment_from_proto)
        .collect()
}
