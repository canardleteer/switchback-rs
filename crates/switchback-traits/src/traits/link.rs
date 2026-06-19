//! Intra-link extraction and formatting.

use crate::error::Result;
use crate::link_context::LinkContext;
use crate::traits::contract::Entity;
use crate::traits::contract_family::ContractFamily;
use crate::traits::entity_category::EntityCategory;
use crate::{IntraLink, LinkTarget, ResolvedManual};

/// Extract intra-links from entity prose (sync, in-memory resolution).
pub trait LinkExtractor: Send + Sync {
    type Family: ContractFamily;

    fn extract<C: EntityCategory>(
        &self,
        entity: &Entity<C>,
        manual: &ResolvedManual,
    ) -> Vec<IntraLink>;
}

/// Extract intra-links with async cross-manual or remote resolution.
pub trait AsyncLinkExtractor: Send + Sync {
    type Family: ContractFamily;

    async fn extract<C: EntityCategory>(
        &self,
        entity: &Entity<C>,
        manual: &ResolvedManual,
    ) -> Result<Vec<IntraLink>>;
}

/// Format a resolved link target for one output format (sync, no I/O).
pub trait LinkFormatter: Send + Sync {
    fn name(&self) -> &'static str;
    fn format(&self, target: &LinkTarget, ctx: &LinkContext) -> String;
}
