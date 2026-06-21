//! `ContractFamily` for OpenRPC.

use serde_json::Value;
use switchback_traits::{
    CompanionDiscovery, CompanionStrategy, ContractFamily, RawDoc, SpecVersion, SupportedVersion,
    VersionStatus,
};

use crate::category::OpenRpcCategory;
use crate::link::OpenRpcLinkExtractor;
use crate::meta_schemas;
use crate::populate::parse_openrpc_version;

#[derive(Clone, Copy, Debug, Default)]
pub struct OpenRpcCompanion;

impl CompanionStrategy for OpenRpcCompanion {
    fn discovery(&self) -> CompanionDiscovery {
        CompanionDiscovery::Beside
    }

    fn output_name(&self, source_dir: &[&str], stem: &str) -> String {
        switchback_traits::companion_output_name_from_segments(source_dir, stem)
    }

    fn companion_media_types(&self) -> &'static [&'static str] {
        &["text/markdown"]
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct OpenRpcFamily;

impl ContractFamily for OpenRpcFamily {
    type Category = OpenRpcCategory;
    type LinkExtractor = OpenRpcLinkExtractor;
    type CompanionStrategy = OpenRpcCompanion;

    fn name(&self) -> &'static str {
        "openrpc"
    }

    fn fence_language(&self) -> &'static str {
        "json"
    }

    fn default_title(&self) -> &'static str {
        "OpenRPC documentation"
    }

    fn default_markdown_root(&self) -> &'static str {
        "contracts"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &[".json", ".yaml", ".yml"]
    }

    fn companion_strategy(&self) -> &Self::CompanionStrategy {
        &OpenRpcCompanion
    }

    fn category_names(&self) -> &'static [&'static str] {
        &["operation", "schema", "parameter"]
    }

    fn detect_version(&self, raw: &RawDoc) -> Option<SpecVersion> {
        let value: Value = if raw.media_type.contains("yaml") {
            serde_saphyr::from_slice(&raw.bytes).ok()?
        } else {
            serde_json::from_slice(&raw.bytes).ok()?
        };
        parse_openrpc_version(&value).ok()
    }

    fn supported_versions(&self) -> &'static [SupportedVersion] {
        &[
            SupportedVersion {
                version: "1.4",
                status: VersionStatus::Latest,
            },
            SupportedVersion {
                version: "1.3",
                status: VersionStatus::WidelyDeployed,
            },
        ]
    }

    fn meta_schema(&self, version: &SpecVersion) -> Option<&'static [u8]> {
        meta_schemas::meta_schema_bytes(version.as_str())
    }

    fn link_extractor(&self) -> &Self::LinkExtractor {
        &OpenRpcLinkExtractor
    }
}
