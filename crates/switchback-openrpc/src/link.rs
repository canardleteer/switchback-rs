//! OpenRPC link extractor (intra-links deferred).

use switchback_traits::{Entity, EntityCategory, IntraLink, LinkExtractor, ResolvedManual};

use crate::family::OpenRpcFamily;

#[derive(Clone, Copy, Debug, Default)]
pub struct OpenRpcLinkExtractor;

impl LinkExtractor for OpenRpcLinkExtractor {
    type Family = OpenRpcFamily;

    fn name(&self) -> &'static str {
        "openrpc"
    }

    fn extract<C: EntityCategory>(
        &self,
        _entity: &Entity<C>,
        _manual: &ResolvedManual,
    ) -> Vec<IntraLink> {
        Vec::new()
    }
}
