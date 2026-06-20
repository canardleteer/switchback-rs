//! Shared wire encode/decode helpers.

use buffa::Message;
use switchback_traits::{Result, SwitchbackError};

pub fn encode_message<M: Message>(message: &M) -> Vec<u8> {
    message.encode_to_vec()
}

pub fn decode_message<M: Message>(bytes: &[u8]) -> Result<M> {
    M::decode_from_slice(bytes).map_err(|e| SwitchbackError::codec(format!("protocol decode: {e}")))
}
