//! Intra-links, structural references, and link targets.

use switchback_traits::{
    Anchor, ContractRef, EntityRef, ExternalUrl, GroupRef, IntraLink, LinkTarget, ManualRef,
    ManualRefInner, ModuleRef, RefKind, Reference,
};

use crate::convert::{codec_err, missing_link_target};
use crate::pb;
use crate::pb::__buffa::oneof::link_target::Target as PbLinkTarget;
use crate::pb::__buffa::oneof::manual_ref::Inner as PbManualRefInner;

pub fn reference_to_proto(reference: &Reference) -> switchback_traits::Result<pb::Reference> {
    Ok(pb::Reference {
        target: buffa::MessageField::some(entity_ref_to_proto(&reference.target)),
        kind: buffa::EnumValue::from(ref_kind_to_proto(reference.kind)),
        ..Default::default()
    })
}

pub fn reference_from_proto(reference: pb::Reference) -> switchback_traits::Result<Reference> {
    Ok(Reference {
        target: entity_ref_from_proto(&reference.target),
        kind: ref_kind_from_proto(&reference.kind)?,
    })
}

pub fn intra_link_to_proto(link: &IntraLink) -> switchback_traits::Result<pb::IntraLink> {
    Ok(pb::IntraLink {
        anchor: buffa::MessageField::some(anchor_to_proto(&link.anchor)),
        target: buffa::MessageField::some(link_target_to_proto(&link.target)?),
        raw: link.raw.clone(),
        ..Default::default()
    })
}

pub fn intra_link_from_proto(link: pb::IntraLink) -> switchback_traits::Result<IntraLink> {
    let target = link.target.into_option().ok_or_else(missing_link_target)?;
    Ok(IntraLink {
        anchor: anchor_from_proto(&link.anchor),
        target: link_target_from_proto(target)?,
        raw: link.raw,
    })
}

fn anchor_to_proto(anchor: &Anchor) -> pb::Anchor {
    pb::Anchor {
        field: anchor.field.clone(),
        byte_start: anchor.byte_start as i32,
        byte_end: anchor.byte_end as i32,
        ..Default::default()
    }
}

fn anchor_from_proto(anchor: &pb::Anchor) -> Anchor {
    Anchor {
        field: anchor.field.clone(),
        byte_start: anchor.byte_start as u32,
        byte_end: anchor.byte_end as u32,
    }
}

fn link_target_to_proto(target: &LinkTarget) -> switchback_traits::Result<pb::LinkTarget> {
    let target = match target {
        LinkTarget::Entity(entity_ref) => {
            PbLinkTarget::EntityRef(Box::new(entity_ref_to_proto(entity_ref)))
        }
        LinkTarget::Group(group_ref) => {
            PbLinkTarget::GroupRef(Box::new(group_ref_to_proto(group_ref)))
        }
        LinkTarget::Contract(contract_ref) => {
            PbLinkTarget::ContractRef(Box::new(contract_ref_to_proto(contract_ref)))
        }
        LinkTarget::Module(module_ref) => {
            PbLinkTarget::ModuleRef(Box::new(module_ref_to_proto(module_ref)))
        }
        LinkTarget::Manual(manual_ref) => {
            PbLinkTarget::ManualRef(Box::new(manual_ref_to_proto(manual_ref)))
        }
        LinkTarget::External(external) => PbLinkTarget::Url(Box::new(pb::ExternalUrl {
            url: external.url.clone(),
            ..Default::default()
        })),
        LinkTarget::Unresolved => {
            return Err(codec_err("cannot serialize unresolved link target"));
        }
    };
    Ok(pb::LinkTarget {
        target: Some(target),
        ..Default::default()
    })
}

fn link_target_from_proto(target: pb::LinkTarget) -> switchback_traits::Result<LinkTarget> {
    let Some(target) = target.target else {
        return Err(missing_link_target());
    };
    Ok(match target {
        PbLinkTarget::EntityRef(entity_ref) => {
            LinkTarget::Entity(entity_ref_from_proto(&entity_ref))
        }
        PbLinkTarget::GroupRef(group_ref) => LinkTarget::Group(group_ref_from_proto(*group_ref)),
        PbLinkTarget::ContractRef(contract_ref) => {
            LinkTarget::Contract(contract_ref_from_proto(*contract_ref))
        }
        PbLinkTarget::ModuleRef(module_ref) => {
            LinkTarget::Module(module_ref_from_proto(*module_ref))
        }
        PbLinkTarget::ManualRef(manual_ref) => {
            LinkTarget::Manual(manual_ref_from_proto(*manual_ref))
        }
        PbLinkTarget::Url(url) => LinkTarget::External(ExternalUrl { url: url.url }),
    })
}

fn entity_ref_to_proto(entity_ref: &EntityRef) -> pb::EntityRef {
    pb::EntityRef {
        module: entity_ref.module.clone(),
        group: entity_ref.group.clone(),
        category: entity_ref.category.clone(),
        name: entity_ref.name.clone(),
        ..Default::default()
    }
}

fn entity_ref_from_proto(entity_ref: &pb::EntityRef) -> EntityRef {
    EntityRef {
        module: entity_ref.module.clone(),
        group: entity_ref.group.clone(),
        category: entity_ref.category.clone(),
        name: entity_ref.name.clone(),
    }
}

fn group_ref_to_proto(group_ref: &GroupRef) -> pb::GroupRef {
    pb::GroupRef {
        module: group_ref.module.clone(),
        group: group_ref.group.clone(),
        ..Default::default()
    }
}

fn group_ref_from_proto(group_ref: pb::GroupRef) -> GroupRef {
    GroupRef {
        module: group_ref.module,
        group: group_ref.group,
    }
}

fn contract_ref_to_proto(contract_ref: &ContractRef) -> pb::ContractRef {
    pb::ContractRef {
        module: contract_ref.module.clone(),
        family: contract_ref.family.clone(),
        version: contract_ref.version.clone(),
        ..Default::default()
    }
}

fn contract_ref_from_proto(contract_ref: pb::ContractRef) -> ContractRef {
    ContractRef {
        module: contract_ref.module,
        family: contract_ref.family,
        version: contract_ref.version,
    }
}

fn module_ref_to_proto(module_ref: &ModuleRef) -> pb::ModuleRef {
    pb::ModuleRef {
        module: module_ref.module.clone(),
        ..Default::default()
    }
}

fn module_ref_from_proto(module_ref: pb::ModuleRef) -> ModuleRef {
    ModuleRef {
        module: module_ref.module,
    }
}

fn manual_ref_to_proto(manual_ref: &ManualRef) -> pb::ManualRef {
    pb::ManualRef {
        uri: manual_ref.uri.clone(),
        version: manual_ref.version.clone(),
        inner: manual_ref.inner.as_ref().map(|inner| match inner {
            ManualRefInner::Entity(entity_ref) => {
                PbManualRefInner::EntityRef(Box::new(entity_ref_to_proto(entity_ref)))
            }
            ManualRefInner::Group(group_ref) => {
                PbManualRefInner::GroupRef(Box::new(group_ref_to_proto(group_ref)))
            }
        }),
        ..Default::default()
    }
}

fn manual_ref_from_proto(manual_ref: pb::ManualRef) -> ManualRef {
    ManualRef {
        uri: manual_ref.uri,
        version: manual_ref.version,
        inner: manual_ref.inner.map(|inner| match inner {
            PbManualRefInner::EntityRef(entity_ref) => {
                ManualRefInner::Entity(entity_ref_from_proto(&entity_ref))
            }
            PbManualRefInner::GroupRef(group_ref) => {
                ManualRefInner::Group(group_ref_from_proto(*group_ref))
            }
        }),
    }
}

fn ref_kind_to_proto(kind: RefKind) -> pb::RefKind {
    match kind {
        RefKind::Unspecified => pb::RefKind::REF_KIND_UNSPECIFIED,
        RefKind::Internal => pb::RefKind::INTERNAL,
        RefKind::External => pb::RefKind::EXTERNAL,
        RefKind::Component => pb::RefKind::COMPONENT,
        RefKind::Inline => pb::RefKind::INLINE,
    }
}

fn ref_kind_from_proto(kind: &buffa::EnumValue<pb::RefKind>) -> switchback_traits::Result<RefKind> {
    match kind.as_known() {
        Some(pb::RefKind::REF_KIND_UNSPECIFIED) | None => Ok(RefKind::Unspecified),
        Some(pb::RefKind::INTERNAL) => Ok(RefKind::Internal),
        Some(pb::RefKind::EXTERNAL) => Ok(RefKind::External),
        Some(pb::RefKind::COMPONENT) => Ok(RefKind::Component),
        Some(pb::RefKind::INLINE) => Ok(RefKind::Inline),
    }
}

pub fn parameter_ref_to_proto(
    parameter: &switchback_traits::ParameterRef,
) -> switchback_traits::Result<pb::ParameterRef> {
    Ok(pb::ParameterRef {
        name: parameter.name.clone(),
        location: parameter.location.clone(),
        required: parameter.required,
        schema_ref: buffa::MessageField::some(reference_to_proto(&parameter.schema_ref)?),
        ..Default::default()
    })
}

pub fn parameter_ref_from_proto(
    parameter: pb::ParameterRef,
) -> switchback_traits::Result<switchback_traits::ParameterRef> {
    let schema_ref = parameter
        .schema_ref
        .into_option()
        .ok_or_else(|| codec_err("parameter schema_ref missing on wire"))?;
    Ok(switchback_traits::ParameterRef {
        name: parameter.name,
        location: parameter.location,
        required: parameter.required,
        schema_ref: reference_from_proto(schema_ref)?,
    })
}

pub fn response_ref_to_proto(
    response: &switchback_traits::ResponseRef,
) -> switchback_traits::Result<pb::ResponseRef> {
    Ok(pb::ResponseRef {
        status: response.status.clone(),
        schema_ref: buffa::MessageField::some(reference_to_proto(&response.schema_ref)?),
        media_type: response.media_type.clone(),
        ..Default::default()
    })
}

pub fn response_ref_from_proto(
    response: pb::ResponseRef,
) -> switchback_traits::Result<switchback_traits::ResponseRef> {
    let schema_ref = response
        .schema_ref
        .into_option()
        .ok_or_else(|| codec_err("response schema_ref missing on wire"))?;
    Ok(switchback_traits::ResponseRef {
        status: response.status,
        schema_ref: reference_from_proto(schema_ref)?,
        media_type: response.media_type,
    })
}

pub fn property_to_proto(
    property: &switchback_traits::Property,
) -> switchback_traits::Result<pb::Property> {
    Ok(pb::Property {
        name: property.name.clone(),
        schema_ref: buffa::MessageField::some(reference_to_proto(&property.schema_ref)?),
        required: property.required,
        ..Default::default()
    })
}

pub fn property_from_proto(
    property: pb::Property,
) -> switchback_traits::Result<switchback_traits::Property> {
    let schema_ref = property
        .schema_ref
        .into_option()
        .ok_or_else(|| codec_err("property schema_ref missing on wire"))?;
    Ok(switchback_traits::Property {
        name: property.name,
        schema_ref: reference_from_proto(schema_ref)?,
        required: property.required,
    })
}
