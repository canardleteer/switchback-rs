//! Errors surfaced at the switchback-traits seam.

use std::path::PathBuf;

use thiserror::Error;

/// Error type for seam operations (codec, render, load, link extraction).
#[derive(Debug, Error)]
pub enum SwitchbackError {
    /// Binary codec serialize/deserialize failed.
    #[error("codec error: {0}")]
    Codec(String),

    /// Renderer failed to produce output.
    #[error("render error: {0}")]
    Render(String),

    /// Contract or manual loading failed.
    #[error("load error: {0}")]
    Load(String),

    /// Link extraction or formatting failed.
    #[error("link error: {0}")]
    Link(String),

    /// Companion discovery failed.
    #[error("companion error: {0}")]
    Companion(String),

    /// Local filesystem I/O.
    #[error("io error at {path}: {source}")]
    Io {
        /// Path involved in the failed I/O operation.
        path: PathBuf,
        /// Underlying operating-system I/O error.
        #[source]
        source: std::io::Error,
    },

    /// Catch-all for family-specific failures without pulling in `anyhow`.
    #[error("{0}")]
    Other(String),
}

impl SwitchbackError {
    /// Wraps a codec failure message.
    pub fn codec(message: impl Into<String>) -> Self {
        Self::Codec(message.into())
    }

    /// Wraps a renderer failure message.
    pub fn render(message: impl Into<String>) -> Self {
        Self::Render(message.into())
    }

    /// Wraps a contract or manual load failure message.
    pub fn load(message: impl Into<String>) -> Self {
        Self::Load(message.into())
    }

    /// Wraps a link extraction or formatting failure message.
    pub fn link(message: impl Into<String>) -> Self {
        Self::Link(message.into())
    }

    /// Wraps a companion discovery failure message.
    pub fn companion(message: impl Into<String>) -> Self {
        Self::Companion(message.into())
    }

    /// Wraps an uncategorized failure message.
    pub fn other(message: impl Into<String>) -> Self {
        Self::Other(message.into())
    }
}

/// Result alias for seam operations that surface [`SwitchbackError`].
pub type Result<T> = std::result::Result<T, SwitchbackError>;
