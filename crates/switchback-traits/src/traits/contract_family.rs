//! `ContractFamily` identity and capability metadata.

use crate::error::Result;
use crate::ids::SpecVersion;
use crate::traits::companion::CompanionStrategy;
use crate::traits::contract::Contract;
use crate::traits::entity_category::EntityCategory;
use crate::traits::link::LinkExtractor;

/// Raw document bytes before full parse, used for version detection.
///
/// In-memory only; not serialized on the wire.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawDoc {
    /// Optional filesystem path hint for error messages and version heuristics.
    pub path_hint: Option<String>,
    /// MIME type of the raw bytes (e.g. `"application/yaml"`).
    pub media_type: String,
    /// Unparsed contract source bytes.
    pub bytes: Vec<u8>,
}

/// One supported spec version entry in a family's version matrix.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SupportedVersion {
    /// Version label string (e.g. `"3.1.0"`).
    pub version: &'static str,
    /// Deployment status hint for tooling and UI.
    pub status: VersionStatus,
}

/// Lifecycle status of a supported contract spec version.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VersionStatus {
    /// Current recommended version for new work.
    Latest,
    /// Common in production but not the newest release.
    WidelyDeployed,
    /// Still parsed but no longer recommended.
    Legacy,
    /// Scheduled for removal; parsers may warn.
    Deprecated,
}

/// Identity and capability metadata for one contract family.
///
/// Each parser crate provides a zero-sized or lightweight implementor describing
/// OpenAPI, Protobuf, AsyncAPI, etc. Associated types wire up family-specific
/// link extraction and companion discovery.
pub trait ContractFamily: Send + Sync + Sized + 'static {
    /// Entity category enum or newtype for this family.
    type Category: EntityCategory;
    /// Family-specific [`LinkExtractor`] implementation.
    type LinkExtractor: LinkExtractor<Family = Self> + Send + Sync;
    /// Family-specific [`CompanionStrategy`] implementation.
    type CompanionStrategy: CompanionStrategy + Send + Sync;

    /// Short family name (e.g. `"openapi"`, `"protobuf"`).
    fn name(&self) -> &'static str;

    /// Default fenced-code language tag for this family's entities.
    fn fence_language(&self) -> &'static str;

    /// Default manual title when the input does not specify one.
    fn default_title(&self) -> &'static str;

    /// Default markdown output root relative to the book root.
    fn default_markdown_root(&self) -> &'static str;

    /// Filename extensions recognized for contract inputs (e.g. `["yaml", "yml"]`).
    fn extensions(&self) -> &'static [&'static str];

    /// Filename extensions searched for companion documents.
    ///
    /// Defaults to [`Self::extensions`] when companions share input extensions.
    fn companion_extensions(&self) -> &'static [&'static str] {
        self.extensions()
    }

    /// Returns the companion discovery strategy for this family.
    fn companion_strategy(&self) -> &Self::CompanionStrategy;

    /// All category slug strings this family emits.
    fn category_names(&self) -> &'static [&'static str];

    /// Detects the spec version from raw bytes, when possible.
    fn detect_version(&self, raw: &RawDoc) -> Option<SpecVersion>;

    /// Supported spec versions and their lifecycle status.
    fn supported_versions(&self) -> &'static [SupportedVersion];

    /// Returns optional meta-schema bytes for validation at `version`, when available.
    fn meta_schema(&self, version: &SpecVersion) -> Option<&'static [u8]> {
        let _ = version;
        None
    }

    /// Returns the link extractor instance for this family.
    fn link_extractor(&self) -> &Self::LinkExtractor;

    /// Additional CLI or config option tokens understood by this family's tooling.
    fn extra_option_tokens(&self) -> &'static [&'static str] {
        &[]
    }
}

/// Async loading of remote or multi-document contracts (I/O primary API).
///
/// Parser crates implement this for fetch-and-parse workflows. The loaded value
/// implements [`Contract`] for in-memory traversal.
pub trait AsyncContractLoader: Send + Sync {
    /// Contract family metadata for the loader.
    type Family: ContractFamily;
    /// Loaded contract type produced by this loader.
    type Loaded: Contract<Family = Self::Family> + Send + Sync;

    /// Fetches inputs and returns a fully loaded contract.
    async fn load(&self) -> Result<Self::Loaded>;
}
