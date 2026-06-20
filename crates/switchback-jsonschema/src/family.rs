//! `ContractFamily` and companion strategy for JSON Schema catalogs.

use switchback_traits::{
    CompanionDiscovery, CompanionStrategy, ContractFamily, RawDoc, SupportedVersion, VersionStatus,
};

use crate::category::JsonSchemaCategory;
use crate::link::JsonSchemaLinkExtractor;

#[derive(Clone, Copy, Debug, Default)]
pub struct JsonSchemaCompanion;

impl CompanionStrategy for JsonSchemaCompanion {
    fn discovery(&self) -> CompanionDiscovery {
        CompanionDiscovery::Ancestors
    }

    fn output_name(&self, source_dir: &[&str], stem: &str) -> String {
        crate::companion::companion_output_name_from_segments(source_dir, stem)
    }

    fn companion_media_types(&self) -> &'static [&'static str] {
        &["text/markdown"]
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct JsonSchemaFamily;

impl ContractFamily for JsonSchemaFamily {
    type Category = JsonSchemaCategory;
    type LinkExtractor = JsonSchemaLinkExtractor;
    type CompanionStrategy = JsonSchemaCompanion;

    fn name(&self) -> &'static str {
        "jsonschema"
    }

    fn fence_language(&self) -> &'static str {
        "yaml"
    }

    fn default_title(&self) -> &'static str {
        "JSON Schema documentation"
    }

    fn default_markdown_root(&self) -> &'static str {
        "contracts"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &[".yaml", ".yml", ".json"]
    }

    fn companion_strategy(&self) -> &Self::CompanionStrategy {
        &JsonSchemaCompanion
    }

    fn category_names(&self) -> &'static [&'static str] {
        &["schema"]
    }

    fn detect_version(&self, _raw: &RawDoc) -> Option<switchback_traits::SpecVersion> {
        None
    }

    fn supported_versions(&self) -> &'static [SupportedVersion] {
        &[SupportedVersion {
            version: "2020-12",
            status: VersionStatus::Latest,
        }]
    }

    fn link_extractor(&self) -> &Self::LinkExtractor {
        &JsonSchemaLinkExtractor
    }
}
