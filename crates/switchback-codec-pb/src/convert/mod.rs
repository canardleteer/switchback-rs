//! Bidirectional mapping between seam model types and protobuf messages.

/// Contract, group, and companion mapping.
mod contract;
/// Entity bodies and stored entity mapping.
mod entity;
/// Intra-links, structural references, and link targets.
mod link;
/// Manual, document, module, and source mapping.
mod manual;

use switchback_traits::{ReferenceManual, Result, SwitchbackError};

use crate::pb;

/// Wire schema version written by [`crate::ProtobufCodec`].
pub const WIRE_VERSION: &str = "v1alpha1";

/// Convert a [`ReferenceManual`] to its protobuf representation.
///
/// Rejects manuals containing [`switchback_traits::LinkTarget::Unresolved`] intra-links.
pub fn to_proto(manual: &ReferenceManual) -> Result<pb::ReferenceManual> {
    validate_resolved_links(manual)?;
    manual::reference_manual_to_proto(manual)
}

/// Convert a protobuf [`ReferenceManual`] into the seam model.
///
/// Rejects wire values whose `switchback_version` does not start with [`WIRE_VERSION`].
pub fn from_proto(manual: pb::ReferenceManual) -> Result<ReferenceManual> {
    validate_wire_version(&manual.switchback_version)?;
    manual::reference_manual_from_proto(manual)
}

fn validate_wire_version(version: &str) -> Result<()> {
    if version.starts_with(WIRE_VERSION) {
        Ok(())
    } else {
        Err(SwitchbackError::codec(format!(
            "unsupported switchback_version {version:?}; expected prefix {WIRE_VERSION:?}"
        )))
    }
}

fn validate_resolved_links(manual: &ReferenceManual) -> Result<()> {
    for module in &manual.modules {
        for contract in &module.contracts {
            for group in &contract.groups {
                for entity in &group.entities {
                    for link in &entity.intra_links {
                        if matches!(link.target, switchback_traits::LinkTarget::Unresolved) {
                            return Err(SwitchbackError::codec(
                                "cannot serialize manual with unresolved intra-link",
                            ));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

pub(crate) fn opt_string(value: &Option<String>) -> String {
    value.clone().unwrap_or_default()
}

pub(crate) fn string_opt(value: String) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

pub(crate) fn codec_err(message: impl Into<String>) -> SwitchbackError {
    SwitchbackError::codec(message)
}

pub(crate) fn missing_link_target() -> SwitchbackError {
    codec_err("link target missing on wire")
}

pub(crate) fn missing_entity_body() -> SwitchbackError {
    codec_err("entity body missing on wire")
}
