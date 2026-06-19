//! `ContractFamily` and companion strategy for protobuf.

use switchback_traits::{
    CompanionDiscovery, CompanionStrategy, ContractFamily, RawDoc, SupportedVersion, VersionStatus,
};

use crate::category::ProtobufCategory;
use crate::link::ProtobufLinkExtractor;

#[derive(Clone, Copy, Debug, Default)]
pub struct ProtobufCompanion;

impl CompanionStrategy for ProtobufCompanion {
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
pub struct ProtobufFamily;

impl ContractFamily for ProtobufFamily {
    type Category = ProtobufCategory;
    type LinkExtractor = ProtobufLinkExtractor;
    type CompanionStrategy = ProtobufCompanion;

    fn name(&self) -> &'static str {
        "protobuf"
    }

    fn fence_language(&self) -> &'static str {
        "protobuf"
    }

    fn default_title(&self) -> &'static str {
        "Protobuf documentation"
    }

    fn default_markdown_root(&self) -> &'static str {
        "packages"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &[".proto"]
    }

    fn companion_strategy(&self) -> &Self::CompanionStrategy {
        &ProtobufCompanion
    }

    fn category_names(&self) -> &'static [&'static str] {
        &["service", "schema", "operation"]
    }

    fn detect_version(&self, _raw: &RawDoc) -> Option<switchback_traits::SpecVersion> {
        None
    }

    fn supported_versions(&self) -> &'static [SupportedVersion] {
        &[
            SupportedVersion {
                version: "proto3",
                status: VersionStatus::Latest,
            },
            SupportedVersion {
                version: "2023",
                status: VersionStatus::Latest,
            },
            SupportedVersion {
                version: "2024",
                status: VersionStatus::Latest,
            },
        ]
    }

    fn link_extractor(&self) -> &Self::LinkExtractor {
        &ProtobufLinkExtractor
    }
}
