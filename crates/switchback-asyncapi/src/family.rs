//! `ContractFamily` stub for AsyncAPI (parser behavior deferred).

use switchback_traits::{
    CompanionDiscovery, CompanionStrategy, ContractFamily, RawDoc, SpecVersion, SupportedVersion,
    VersionStatus,
};

use crate::category::AsyncApiCategory;
use crate::link::AsyncApiLinkExtractor;
use crate::meta_schemas;

#[derive(Clone, Copy, Debug, Default)]
pub struct AsyncApiCompanion;

impl CompanionStrategy for AsyncApiCompanion {
    fn discovery(&self) -> CompanionDiscovery {
        CompanionDiscovery::Beside
    }

    fn output_name(&self, _source_dir: &[&str], stem: &str) -> String {
        format!("{stem}.md")
    }

    fn companion_media_types(&self) -> &'static [&'static str] {
        &["text/markdown"]
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AsyncApiFamily;

impl ContractFamily for AsyncApiFamily {
    type Category = AsyncApiCategory;
    type LinkExtractor = AsyncApiLinkExtractor;
    type CompanionStrategy = AsyncApiCompanion;

    fn name(&self) -> &'static str {
        "asyncapi"
    }

    fn fence_language(&self) -> &'static str {
        "yaml"
    }

    fn default_title(&self) -> &'static str {
        "AsyncAPI documentation"
    }

    fn default_markdown_root(&self) -> &'static str {
        "contracts"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &[".yaml", ".yml", ".json"]
    }

    fn companion_strategy(&self) -> &Self::CompanionStrategy {
        &AsyncApiCompanion
    }

    fn category_names(&self) -> &'static [&'static str] {
        &[
            "channel",
            "operation",
            "message",
            "schema",
            "parameter",
            "security-scheme",
        ]
    }

    fn detect_version(&self, _raw: &RawDoc) -> Option<SpecVersion> {
        None
    }

    fn supported_versions(&self) -> &'static [SupportedVersion] {
        &[
            SupportedVersion {
                version: "3.0.0",
                status: VersionStatus::Latest,
            },
            SupportedVersion {
                version: "2.6.0",
                status: VersionStatus::WidelyDeployed,
            },
        ]
    }

    fn meta_schema(&self, version: &SpecVersion) -> Option<&'static [u8]> {
        meta_schemas::meta_schema_bytes(version.as_str())
    }

    fn link_extractor(&self) -> &Self::LinkExtractor {
        &AsyncApiLinkExtractor
    }
}
