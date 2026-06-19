//! Companion document discovery (sync + async varieties).

use std::path::Path;

use crate::{CompanionFile, Result};

/// Where a family searches for companion documents relative to contract inputs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompanionDiscovery {
    /// Search in the same directory as the contract file.
    Beside,
    /// Walk ancestor directories from the contract file location.
    Ancestors,
    /// Search a `docs/` subdirectory relative to the contract root.
    DocsSubdir,
}

/// How a family discovers, names, and places companion docs (sync, local).
///
/// Associated with [`ContractFamily`](super::contract_family::ContractFamily) via
/// [`ContractFamily::CompanionStrategy`](super::contract_family::ContractFamily::CompanionStrategy).
pub trait CompanionStrategy: Send + Sync {
    /// Returns the filesystem search strategy for companion files.
    fn discovery(&self) -> CompanionDiscovery;

    /// Computes the output filename for a companion given its source directory path segments and stem.
    fn output_name(&self, source_dir: &[&str], stem: &str) -> String;

    /// MIME types accepted for companion documents in this family.
    fn companion_media_types(&self) -> &'static [&'static str];

    /// Discovers companions on the local filesystem relative to contract inputs.
    ///
    /// Default returns empty; families with companion docs override.
    fn discover_local(&self, contract_root: &Path) -> Result<Vec<CompanionFile>> {
        let _ = contract_root;
        Ok(Vec::new())
    }
}

/// Async companion fetch (remote registries, URL-backed docs).
///
/// Use when companion discovery requires network I/O beyond local filesystem search.
pub trait AsyncCompanionStrategy: Send + Sync {
    /// Returns the search strategy (local layout hint for remote-backed companions).
    fn discovery(&self) -> CompanionDiscovery;

    /// Computes the output filename for a companion given its source directory path segments and stem.
    fn output_name(&self, source_dir: &[&str], stem: &str) -> String;

    /// MIME types accepted for companion documents in this family.
    fn companion_media_types(&self) -> &'static [&'static str];

    /// Discovers companions asynchronously (remote fetch, registry lookup, etc.).
    async fn discover(&self, contract_root: &Path) -> Result<Vec<CompanionFile>>;
}
