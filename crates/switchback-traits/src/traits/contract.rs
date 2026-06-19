//! Loaded contract instance view (sync API on an in-memory loaded contract).

use crate::ids::{EntityId, SpecVersion};
use crate::link_context::LinkContext;
use crate::options::Options;
use crate::traits::contract_family::ContractFamily;
use crate::traits::entity_category::EntityCategory;
use crate::{Companion, CompanionFile, EntityBody, Group, Span};

/// Parser-side entity with a family-typed category.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entity<C: EntityCategory> {
    pub id: EntityId,
    pub category: C,
    pub title: String,
    pub doc: Option<String>,
    pub source_span: Option<Span>,
    pub body: EntityBody,
}

/// A loaded, resolved contract ready to render or serialize.
pub trait Contract: Send + Sync {
    type Family: ContractFamily;

    fn family(&self) -> &Self::Family;
    fn version(&self) -> &SpecVersion;
    type Category: EntityCategory;

    fn groups(&self) -> &[Group];
    fn entities(&self, group: &Group) -> &[Entity<Self::Category>];

    fn link_context(&self, opts: &Options) -> LinkContext;

    fn group_overview(&self, group: &Group) -> Option<&str> {
        let _ = group;
        None
    }

    fn companions(&self) -> &[CompanionFile] {
        &[]
    }
}

/// Convert parser-side companions into stored switchback companions.
pub fn companion_files_to_stored(files: &[CompanionFile], media_type: &str) -> Vec<Companion> {
    files
        .iter()
        .map(|file| Companion {
            output_name: file.output_name.clone(),
            bytes: file.bytes.clone(),
            media_type: media_type.to_string(),
        })
        .collect()
}
