//! Minimal JSON Schema link extractor (intra-links deferred).

use switchback_traits::{Entity, EntityCategory, IntraLink, LinkExtractor, ResolvedManual};

use crate::family::JsonSchemaFamily;

#[derive(Clone, Copy, Debug, Default)]
pub struct JsonSchemaLinkExtractor;

impl LinkExtractor for JsonSchemaLinkExtractor {
    type Family = JsonSchemaFamily;

    fn name(&self) -> &'static str {
        "jsonschema-stub"
    }

    fn extract<C: EntityCategory>(
        &self,
        _entity: &Entity<C>,
        _manual: &ResolvedManual,
    ) -> Vec<IntraLink> {
        Vec::new()
    }
}
