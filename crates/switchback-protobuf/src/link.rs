//! Minimal protobuf link extractor (intra-links deferred).

use switchback_traits::{Entity, EntityCategory, IntraLink, LinkExtractor, ResolvedManual};

use crate::family::ProtobufFamily;

#[derive(Clone, Copy, Debug, Default)]
pub struct ProtobufLinkExtractor;

impl LinkExtractor for ProtobufLinkExtractor {
    type Family = ProtobufFamily;

    fn extract<C: EntityCategory>(
        &self,
        _entity: &Entity<C>,
        _manual: &ResolvedManual,
    ) -> Vec<IntraLink> {
        Vec::new()
    }
}
