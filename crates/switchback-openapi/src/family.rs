//! `ContractFamily` for OpenAPI.

use serde_json::Value;
use switchback_traits::{
    CompanionDiscovery, CompanionStrategy, ContractFamily, RawDoc, SpecVersion, SupportedVersion,
    VersionStatus,
};

use crate::category::OpenApiCategory;
use crate::link::OpenApiLinkExtractor;
use crate::meta_schemas;
use crate::populate::parse_openapi_version;

#[derive(Clone, Copy, Debug, Default)]
pub struct OpenApiCompanion;

impl CompanionStrategy for OpenApiCompanion {
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
pub struct OpenApiFamily;

impl ContractFamily for OpenApiFamily {
    type Category = OpenApiCategory;
    type LinkExtractor = OpenApiLinkExtractor;
    type CompanionStrategy = OpenApiCompanion;

    fn name(&self) -> &'static str {
        "openapi"
    }

    fn fence_language(&self) -> &'static str {
        "yaml"
    }

    fn default_title(&self) -> &'static str {
        "OpenAPI documentation"
    }

    fn default_markdown_root(&self) -> &'static str {
        "src/contracts"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &[".yaml", ".yml", ".json"]
    }

    fn companion_strategy(&self) -> &Self::CompanionStrategy {
        &OpenApiCompanion
    }

    fn category_names(&self) -> &'static [&'static str] {
        &[
            "schema",
            "operation",
            "parameter",
            "response",
            "request-body",
            "security-scheme",
        ]
    }

    fn detect_version(&self, raw: &RawDoc) -> Option<SpecVersion> {
        let value: Value = if raw.media_type.contains("yaml") {
            serde_saphyr::from_slice(&raw.bytes).ok()?
        } else {
            serde_json::from_slice(&raw.bytes).ok()?
        };
        parse_openapi_version(&value).ok()
    }

    fn supported_versions(&self) -> &'static [SupportedVersion] {
        &[
            SupportedVersion {
                version: "3.1.0",
                status: VersionStatus::Latest,
            },
            SupportedVersion {
                version: "3.0.3",
                status: VersionStatus::WidelyDeployed,
            },
            SupportedVersion {
                version: "2.0",
                status: VersionStatus::Legacy,
            },
        ]
    }

    fn meta_schema(&self, version: &SpecVersion) -> Option<&'static [u8]> {
        meta_schemas::meta_schema_bytes(version.as_str())
    }

    fn link_extractor(&self) -> &Self::LinkExtractor {
        &OpenApiLinkExtractor
    }

    fn supported_protocols(&self) -> &'static [&'static str] {
        &["http"]
    }

    fn default_protocol(&self) -> &'static str {
        "http"
    }
}
