//! `ContractFamily` for AsyncAPI.

use serde_json::Value;
use switchback_traits::{
    CompanionDiscovery, CompanionStrategy, ContractFamily, RawDoc, SpecVersion, SupportedVersion,
    VersionStatus,
};

use crate::category::AsyncApiCategory;
use crate::link::AsyncApiLinkExtractor;
use crate::meta_schemas;
use crate::populate::parse_asyncapi_version;

#[derive(Clone, Copy, Debug, Default)]
pub struct AsyncApiCompanion;

impl CompanionStrategy for AsyncApiCompanion {
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

    fn detect_version(&self, raw: &RawDoc) -> Option<SpecVersion> {
        let value: Value = if raw.media_type.contains("yaml") {
            serde_saphyr::from_slice(&raw.bytes).ok()?
        } else {
            serde_json::from_slice(&raw.bytes).ok()?
        };
        parse_asyncapi_version(&value).ok()
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

    fn supported_protocols(&self) -> &'static [&'static str] {
        &["kafka", "amqp", "mqtt", "http", "websockets"]
    }

    fn default_protocol(&self) -> &'static str {
        "kafka"
    }
}
