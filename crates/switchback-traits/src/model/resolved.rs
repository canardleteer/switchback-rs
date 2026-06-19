//! Parser-side index over a whole manual for link extraction.

use std::collections::HashMap;

use crate::ids::{EntityId, GroupId, ModuleId};
use crate::model::entity::StoredEntity;
use crate::model::link::EntityRef;

/// Flat index entry for one entity in a resolved manual.
///
/// In-memory only; built by parsers before link extraction. Not a wire message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexedEntity {
    /// Module containing the entity.
    pub module_id: ModuleId,
    /// Contract family name for the entity's contract.
    pub contract_family: String,
    /// Group containing the entity.
    pub group_id: GroupId,
    /// Stored entity payload (may be a clone from a parser-side view).
    pub entity: StoredEntity,
}

/// Whole-manual address space used by [`LinkExtractor`](crate::traits::LinkExtractor).
///
/// Population logic lives in parser crates; this crate defines the shell only.
/// In-memory only; not serialized on the wire.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ResolvedManual {
    /// Flat list of all indexed entities across modules and contracts.
    pub entities: Vec<IndexedEntity>,
    /// Reverse lookup from wire [`EntityRef`] to parser-side [`EntityId`].
    pub by_ref: HashMap<EntityRef, EntityId>,
}

impl ResolvedManual {
    /// Builds a resolved manual and populates `by_ref` from the entity list.
    pub fn new(entities: Vec<IndexedEntity>) -> Self {
        let mut by_ref = HashMap::new();
        for indexed in &entities {
            let entity_ref = EntityRef {
                module: indexed.module_id.as_str().to_string(),
                group: indexed.group_id.as_str().to_string(),
                category: indexed.entity.category.clone(),
                name: indexed.entity.name.clone(),
            };
            let entity_id = EntityId::new(
                indexed.group_id.clone(),
                indexed.entity.category.clone(),
                indexed.entity.name.clone(),
            );
            by_ref.insert(entity_ref, entity_id);
        }
        Self { entities, by_ref }
    }
}
