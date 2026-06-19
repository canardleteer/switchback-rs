//! `ContractFamily` identity and capability metadata.

use crate::error::Result;
use crate::ids::SpecVersion;
use crate::traits::companion::CompanionStrategy;
use crate::traits::contract::Contract;
use crate::traits::entity_category::EntityCategory;
use crate::traits::link::LinkExtractor;

/// Raw document bytes before full parse, used for version detection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawDoc {
    pub path_hint: Option<String>,
    pub media_type: String,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SupportedVersion {
    pub version: &'static str,
    pub status: VersionStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VersionStatus {
    Latest,
    WidelyDeployed,
    Legacy,
    Deprecated,
}

/// Identity and capability metadata for one contract family.
pub trait ContractFamily: Send + Sync + Sized + 'static {
    type Category: EntityCategory;
    type LinkExtractor: LinkExtractor<Family = Self> + Send + Sync;
    type CompanionStrategy: CompanionStrategy + Send + Sync;

    fn name(&self) -> &'static str;
    fn fence_language(&self) -> &'static str;
    fn default_title(&self) -> &'static str;
    fn default_markdown_root(&self) -> &'static str;
    fn extensions(&self) -> &'static [&'static str];

    fn companion_extensions(&self) -> &'static [&'static str] {
        self.extensions()
    }

    fn companion_strategy(&self) -> &Self::CompanionStrategy;

    fn category_names(&self) -> &'static [&'static str];

    fn detect_version(&self, raw: &RawDoc) -> Option<SpecVersion>;

    fn supported_versions(&self) -> &'static [SupportedVersion];

    fn meta_schema(&self, version: &SpecVersion) -> Option<&'static [u8]> {
        let _ = version;
        None
    }

    fn link_extractor(&self) -> &Self::LinkExtractor;

    fn extra_option_tokens(&self) -> &'static [&'static str] {
        &[]
    }
}

/// Async loading of remote or multi-document contracts (I/O primary API).
pub trait AsyncContractLoader: Send + Sync {
    type Family: ContractFamily;
    type Loaded: Contract<Family = Self::Family> + Send + Sync;

    async fn load(&self) -> Result<Self::Loaded>;
}
