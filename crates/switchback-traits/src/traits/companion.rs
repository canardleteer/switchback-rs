//! Companion document discovery (sync + async varieties).

use std::path::Path;

use crate::{CompanionFile, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompanionDiscovery {
    Beside,
    Ancestors,
    DocsSubdir,
}

/// How a family discovers, names, and places companion docs (sync, local).
pub trait CompanionStrategy: Send + Sync {
    fn discovery(&self) -> CompanionDiscovery;
    fn output_name(&self, source_dir: &[&str], stem: &str) -> String;
    fn companion_media_types(&self) -> &'static [&'static str];

    /// Discover companions on the local filesystem relative to contract inputs.
    fn discover_local(&self, contract_root: &Path) -> Result<Vec<CompanionFile>> {
        let _ = contract_root;
        Ok(Vec::new())
    }
}

/// Async companion fetch (remote registries, URL-backed docs).
pub trait AsyncCompanionStrategy: Send + Sync {
    fn discovery(&self) -> CompanionDiscovery;
    fn output_name(&self, source_dir: &[&str], stem: &str) -> String;
    fn companion_media_types(&self) -> &'static [&'static str];

    async fn discover(&self, contract_root: &Path) -> Result<Vec<CompanionFile>>;
}
