//! Renderer seam (async primary + sync secondary).

use crate::{ReferenceManual, Result};

/// One rendered output file (path relative to book root unless absolute).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputFile {
    /// Output path relative to the mdBook root (or absolute when required).
    pub path: String,
    /// Rendered file content bytes (typically UTF-8 markdown).
    pub content: Vec<u8>,
}

/// Async renderer for service-side and streaming pipelines.
///
/// Primary API per [ADR 0002](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0002-async-first-traits-with-synchronous-secondary-apis-in-switchback-traits.md).
/// Renderer crates implement this trait against a family-specific [`Options`](crate::Options) type.
pub trait Renderer: Send + Sync {
    /// Renderer-specific options (layout, paths, escaping, etc.).
    type Opts: Send + Sync;

    /// Renders a [`ReferenceManual`] into one or more output files.
    async fn render(&self, manual: &ReferenceManual, opts: &Self::Opts) -> Result<Vec<OutputFile>>;
}

/// Synchronous compatibility API for callers that cannot wrap [`Renderer`].
///
/// Secondary API per ADR 0002. Prefer [`Renderer`] for async pipelines.
pub trait SyncRenderer: Send + Sync {
    /// Renderer-specific options (layout, paths, escaping, etc.).
    type Opts: Send + Sync;

    /// Renders a [`ReferenceManual`] into one or more output files.
    fn render(&self, manual: &ReferenceManual, opts: &Self::Opts) -> Result<Vec<OutputFile>>;
}
