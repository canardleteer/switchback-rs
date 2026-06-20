//! AsyncAPI link extractor (intra-links deferred).

use switchback_traits::{Entity, EntityCategory, IntraLink, LinkExtractor, ResolvedManual};

use crate::family::AsyncApiFamily;

#[derive(Clone, Copy, Debug, Default)]
pub struct AsyncApiLinkExtractor;

impl LinkExtractor for AsyncApiLinkExtractor {
    type Family = AsyncApiFamily;

    fn name(&self) -> &'static str {
        "asyncapi-stub"
    }

    fn extract<C: EntityCategory>(
        &self,
        _entity: &Entity<C>,
        _manual: &ResolvedManual,
    ) -> Vec<IntraLink> {
        Vec::new()
    }
}
