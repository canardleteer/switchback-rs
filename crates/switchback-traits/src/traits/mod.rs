//! Seam trait definitions.

mod codec;
mod companion;
mod contract;
mod contract_family;
mod entity_category;
mod link;
mod renderer;

pub use codec::{SwitchbackCodec, SyncSwitchbackCodec};
pub use companion::{AsyncCompanionStrategy, CompanionDiscovery, CompanionStrategy};
pub use contract::{companion_files_to_stored, Contract, Entity};
pub use contract_family::{
    AsyncContractLoader, ContractFamily, RawDoc, SupportedVersion, VersionStatus,
};
pub use entity_category::{EntityCategory, GenericCategory};
pub use link::{AsyncLinkExtractor, LinkExtractor, LinkFormatter};
pub use renderer::{OutputFile, Renderer, SyncRenderer};
