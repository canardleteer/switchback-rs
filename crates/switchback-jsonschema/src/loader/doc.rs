//! Loaded document representation.

use std::path::PathBuf;

use serde_json::Value;

/// One YAML or JSON document on disk.
#[derive(Clone, Debug)]
pub struct Doc {
    /// URI relative to the module root (forward slashes).
    pub uri: String,
    /// Absolute filesystem path used to load this document.
    pub path: PathBuf,
    /// Original bytes (lossless for the source layer).
    pub raw_bytes: Vec<u8>,
    /// Parsed JSON value (YAML is converted at load time).
    pub value: Value,
    /// Whether the on-disk format was YAML.
    pub is_yaml: bool,
    /// MIME type for the source layer.
    pub media_type: String,
}

impl Doc {
    pub fn fence_language(&self) -> &'static str {
        if self.is_yaml {
            "yaml"
        } else {
            "json"
        }
    }
}
