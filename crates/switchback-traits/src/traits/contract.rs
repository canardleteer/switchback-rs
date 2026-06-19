//! Loaded contract instance view (sync API on an in-memory loaded contract).

use crate::ids::{EntityId, SpecVersion};
use crate::link_context::LinkContext;
use crate::options::Options;
use crate::traits::contract_family::ContractFamily;
use crate::traits::entity_category::EntityCategory;
use crate::{Companion, CompanionFile, EntityBody, Group, Span};

/// Parser-side entity with a family-typed category.
///
/// In-memory view produced by contract-family parsers. Convert to wire
/// [`StoredEntity`](crate::StoredEntity) when building a [`ReferenceManual`](crate::ReferenceManual).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entity<C: EntityCategory> {
    /// Entity address within the loaded contract.
    pub id: EntityId,
    /// Family-specific typed category marker.
    pub category: C,
    /// Human-readable entity title.
    pub title: String,
    /// Leading documentation prose, when present.
    pub doc: Option<String>,
    /// Source span within the input document, when available.
    ///
    /// Parser-local provenance; maps to wire [`Source`](crate::Source) at serialize time.
    pub source_span: Option<Span>,
    /// Category-specific entity payload.
    pub body: EntityBody,
}

/// A loaded, resolved contract ready to render or serialize.
///
/// Parser crates implement this trait after parsing and resolving references.
/// The sync API supports in-memory traversal during manual assembly.
pub trait Contract: Send + Sync {
    /// Contract family metadata and capabilities.
    type Family: ContractFamily;

    /// Returns the contract family identity for this loaded instance.
    fn family(&self) -> &Self::Family;

    /// Returns the parsed spec version for this contract instance.
    fn version(&self) -> &SpecVersion;

    /// Family-specific entity category type.
    type Category: EntityCategory;

    /// Returns all groups in this contract (entities are queried per group).
    fn groups(&self) -> &[Group];

    /// Returns entities belonging to `group`.
    fn entities(&self, group: &Group) -> &[Entity<Self::Category>];

    /// Builds a [`LinkContext`] for cross-reference formatting under `opts`.
    fn link_context(&self, opts: &Options) -> LinkContext;

    /// Returns optional overview prose for `group`, when the family provides it.
    fn group_overview(&self, group: &Group) -> Option<&str> {
        let _ = group;
        None
    }

    /// Returns parser-side companion files discovered for this contract.
    ///
    /// Default empty; families with companion docs override. Convert to wire
    /// [`Companion`](crate::Companion) via [`companion_files_to_stored`](crate::traits::companion_files_to_stored).
    fn companions(&self) -> &[CompanionFile] {
        &[]
    }
}

/// Convert parser-side companions into stored switchback companions.
///
/// Strips parser-local fields (e.g. [`CompanionFile::source_path`]) and assigns
/// a uniform `media_type` for the wire representation.
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
