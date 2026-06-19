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
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Catch-all for family-specific failures without pulling in `anyhow`.
    #[error("{0}")]
    Other(String),
}

impl SwitchbackError {
    pub fn codec(message: impl Into<String>) -> Self {
        Self::Codec(message.into())
    }

    pub fn render(message: impl Into<String>) -> Self {
        Self::Render(message.into())
    }

    pub fn load(message: impl Into<String>) -> Self {
        Self::Load(message.into())
    }

    pub fn link(message: impl Into<String>) -> Self {
        Self::Link(message.into())
    }

    pub fn companion(message: impl Into<String>) -> Self {
        Self::Companion(message.into())
    }

    pub fn other(message: impl Into<String>) -> Self {
        Self::Other(message.into())
    }
}

pub type Result<T> = std::result::Result<T, SwitchbackError>;
