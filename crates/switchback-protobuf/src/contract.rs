//! Loaded protobuf contract view.

use std::collections::BTreeMap;

use switchback_traits::{
    CompanionFile, Contract, Entity, EntityCategory, Group, GroupId, LinkContext, Options,
    Reference, StoredEntity,
};

use crate::category::ProtobufCategory;
use crate::family::ProtobufFamily;
use crate::populate::{PopulatedContract, PopulatedEntity};

pub struct ProtobufContract {
    family: ProtobufFamily,
    version: switchback_traits::SpecVersion,
    groups: Vec<Group>,
    entities_by_group: BTreeMap<GroupId, Vec<PopulatedEntity>>,
    companions: Vec<CompanionFile>,
}

impl ProtobufContract {
    pub fn from_populated(populated: PopulatedContract) -> Self {
        Self {
            family: ProtobufFamily,
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

impl Contract for ProtobufContract {
    type Family = ProtobufFamily;
    type Category = ProtobufCategory;

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
        let mut ctx = LinkContext::empty(opts.layout, &opts.book_root, &opts.markdown_root);
        let module_id = self
            .groups
            .first()
            .map(|g| g.id.as_str())
            .unwrap_or("default");
        for group in &self.groups {
            if group.id.as_str().is_empty() {
                continue;
            }
            for pe in self.populated_entities(group) {
                ctx.register_stored_entity(
                    module_id,
                    group.id.as_str(),
                    &stored_entity_from_populated(pe),
                    opts.layout,
                    &opts.markdown_root,
                );
            }
        }
        ctx
    }

    fn companions(&self) -> &[CompanionFile] {
        &self.companions
    }
}

impl ProtobufContract {
    pub fn entity_refs_for_group(
        &self,
        group: &Group,
    ) -> Vec<(&Entity<ProtobufCategory>, &[Reference])> {
        self.populated_entities(group)
            .iter()
            .map(|pe| (&pe.entity, pe.refs.as_slice()))
            .collect()
    }
}

fn stored_entity_from_populated(pe: &PopulatedEntity) -> StoredEntity {
    StoredEntity {
        name: pe.entity.id.name.clone(),
        category: pe.entity.category.as_str().to_string(),
        title: pe.entity.title.clone(),
        doc: pe.entity.doc.clone(),
        source: None,
        refs: pe.refs.clone(),
        intra_links: Vec::new(),
        body: pe.entity.body.clone(),
    }
}
