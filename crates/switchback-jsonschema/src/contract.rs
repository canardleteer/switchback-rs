//! Loaded JSON Schema catalog contract view.

use std::collections::BTreeMap;

use switchback_traits::{
    CompanionFile, Contract, Entity, Group, GroupId, LinkContext, Options, Reference,
};

use crate::category::JsonSchemaCategory;
use crate::family::JsonSchemaFamily;
use crate::populate::{PopulatedContract, PopulatedEntity};

pub struct JsonSchemaContract {
    family: JsonSchemaFamily,
    version: switchback_traits::SpecVersion,
    groups: Vec<Group>,
    entities_by_group: BTreeMap<GroupId, Vec<PopulatedEntity>>,
    companions: Vec<CompanionFile>,
}

impl JsonSchemaContract {
    pub fn from_populated(populated: PopulatedContract) -> Self {
        Self {
            family: JsonSchemaFamily,
            version: populated.version,
            groups: populated.groups,
            entities_by_group: populated.entities_by_group,
            companions: populated.companions,
        }
    }

    pub fn populated_entities(&self, group: &Group) -> &[PopulatedEntity] {
        self.entities_by_group
            .get(&group.id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
}

impl Contract for JsonSchemaContract {
    type Family = JsonSchemaFamily;
    type Category = JsonSchemaCategory;

    fn family(&self) -> &Self::Family {
        &self.family
    }

    fn version(&self) -> &switchback_traits::SpecVersion {
        &self.version
    }

    fn groups(&self) -> &[Group] {
        &self.groups
    }

    fn entities(&self, _group: &Group) -> &[Entity<Self::Category>] {
        &[]
    }

    fn link_context(&self, opts: &Options) -> LinkContext {
        LinkContext::empty(opts.layout, &opts.book_root, &opts.markdown_root)
    }

    fn companions(&self) -> &[CompanionFile] {
        &self.companions
    }
}

impl JsonSchemaContract {
    pub fn entity_refs_for_group(
        &self,
        group: &Group,
    ) -> Vec<(&Entity<JsonSchemaCategory>, &[Reference])> {
        self.populated_entities(group)
            .iter()
            .map(|pe| (&pe.entity, pe.refs.as_slice()))
            .collect()
    }
}
