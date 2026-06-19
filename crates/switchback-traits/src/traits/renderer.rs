//! Renderer seam (async primary + sync secondary).

use crate::{ReferenceManual, Result};

/// One rendered output file (path relative to book root unless absolute).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputFile {
    pub path: String,
    pub content: Vec<u8>,
}

/// Async renderer for service-side and streaming pipelines.
pub trait Renderer: Send + Sync {
    type Opts: Send + Sync;

    async fn render(&self, manual: &ReferenceManual, opts: &Self::Opts) -> Result<Vec<OutputFile>>;
}

/// Synchronous compatibility API for callers that cannot wrap [`Renderer`].
pub trait SyncRenderer: Send + Sync {
    type Opts: Send + Sync;

    fn render(&self, manual: &ReferenceManual, opts: &Self::Opts) -> Result<Vec<OutputFile>>;
}
