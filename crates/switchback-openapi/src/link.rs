//! OpenAPI link extractor (intra-links deferred).

use switchback_traits::{Entity, EntityCategory, IntraLink, LinkExtractor, ResolvedManual};

use crate::family::OpenApiFamily;

#[derive(Clone, Copy, Debug, Default)]
pub struct OpenApiLinkExtractor;

impl LinkExtractor for OpenApiLinkExtractor {
    type Family = OpenApiFamily;

    fn name(&self) -> &'static str {
        "openapi-stub"
    }

    fn extract<C: EntityCategory>(
        &self,
        _entity: &Entity<C>,
        _manual: &ResolvedManual,
    ) -> Vec<IntraLink> {
        Vec::new()
    }
}
