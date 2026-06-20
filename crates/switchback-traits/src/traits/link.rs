//! Intra-link extraction and formatting.

use crate::error::Result;
use crate::link_context::LinkContext;
use crate::traits::contract::Entity;
use crate::traits::contract_family::ContractFamily;
use crate::traits::entity_category::EntityCategory;
use crate::{IntraLink, LinkTarget, ResolvedManual};

/// Extract intra-links from entity prose (sync, in-memory resolution).
///
/// Family parser crates implement this trait. Extraction may produce
/// [`LinkTarget::Unresolved`] for unknown targets; strip those before wire serialize.
pub trait LinkExtractor: Send + Sync {
    /// Contract family this extractor belongs to.
    type Family: ContractFamily;

    /// Short extractor name (e.g. `"protobuf-fqn"`).
    fn name(&self) -> &'static str;

    /// Extracts prose-level links from `entity` against the `manual` address space.
    fn extract<C: EntityCategory>(
        &self,
        entity: &Entity<C>,
        manual: &ResolvedManual,
    ) -> Vec<IntraLink>;
}

/// Extract intra-links with async cross-manual or remote resolution.
///
/// Use when link targets require network I/O or external manual fetches.
pub trait AsyncLinkExtractor: Send + Sync {
    /// Contract family this extractor belongs to.
    type Family: ContractFamily;

    /// Extracts prose-level links, resolving remote or cross-manual targets asynchronously.
    async fn extract<C: EntityCategory>(
        &self,
        entity: &Entity<C>,
        manual: &ResolvedManual,
    ) -> Result<Vec<IntraLink>>;
}

/// Format a resolved link target for one output format (sync, no I/O).
///
/// Renderer crates implement this to turn [`LinkTarget`] values into markdown links,
/// using [`LinkContext`] for relative path resolution.
pub trait LinkFormatter: Send + Sync {
    /// Short formatter name (e.g. `"markdown-relative"`).
    fn name(&self) -> &'static str;

    /// Formats `target` as a link string appropriate for the output format.
    fn format(&self, target: &LinkTarget, ctx: &LinkContext) -> String;
}
