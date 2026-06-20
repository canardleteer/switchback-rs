//! Contract, group, and companion mapping.

use std::path::PathBuf;

use switchback_traits::{Companion, Group, GroupId, ManualContract, SpecVersion};

use crate::convert::{manual, opt_string, string_opt};
use crate::pb;

use super::entity;

pub fn contract_to_proto(contract: &ManualContract) -> switchback_traits::Result<pb::Contract> {
    Ok(pb::Contract {
        family: contract.family.clone(),
        version: contract.version.as_str().to_string(),
        groups: contract
            .groups
            .iter()
            .map(group_to_proto)
            .collect::<switchback_traits::Result<_>>()?,
        companions: contract.companions.iter().map(companion_to_proto).collect(),
        ..Default::default()
    })
}

pub fn contract_from_proto(contract: pb::Contract) -> switchback_traits::Result<ManualContract> {
    Ok(ManualContract {
        family: contract.family,
        version: SpecVersion::from(contract.version),
        groups: contract
            .groups
            .into_iter()
            .map(group_from_proto)
            .collect::<switchback_traits::Result<_>>()?,
        companions: contract
            .companions
            .into_iter()
            .map(companion_from_proto)
            .collect(),
    })
}

fn group_to_proto(group: &Group) -> switchback_traits::Result<pb::Group> {
    Ok(pb::Group {
        id: group.id.as_str().to_string(),
        dir: group.dir.clone(),
        title: group.title.clone(),
        overview: opt_string(&group.overview),
        source: group
            .source
            .as_ref()
            .map(manual::source_to_proto)
            .map(buffa::MessageField::some)
            .unwrap_or_default(),
        entities: group
            .entities
            .iter()
            .map(entity::entity_to_proto)
            .collect::<switchback_traits::Result<_>>()?,
        ..Default::default()
    })
}

fn group_from_proto(group: pb::Group) -> switchback_traits::Result<Group> {
    Ok(Group {
        id: GroupId::from(group.id),
        dir: group.dir,
        title: group.title,
        overview: string_opt(group.overview),
        source: if group.source.is_set() {
            Some(manual::source_from_proto(&group.source))
        } else {
            None
        },
        entities: group
            .entities
            .into_iter()
            .map(entity::entity_from_proto)
            .collect::<switchback_traits::Result<_>>()?,
        source_path: PathBuf::new(),
    })
}

fn companion_to_proto(companion: &Companion) -> pb::Companion {
    pb::Companion {
        output_name: companion.output_name.clone(),
        bytes: companion.bytes.clone(),
        media_type: companion.media_type.clone(),
        title: companion.title.clone(),
        source_dir: companion.source_dir.clone(),
        stem: companion.stem.clone(),
        ..Default::default()
    }
}

fn companion_from_proto(companion: pb::Companion) -> Companion {
    Companion {
        output_name: companion.output_name,
        bytes: companion.bytes,
        media_type: companion.media_type,
        title: companion.title,
        source_dir: companion.source_dir,
        stem: companion.stem,
    }
}
