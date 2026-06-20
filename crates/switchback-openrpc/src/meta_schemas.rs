//! Vendored OpenRPC JSON Schema meta-schemas from [open-rpc/spec](https://github.com/open-rpc/spec).

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// A vendored JSON Schema asset (SHA-256 in `meta-schemas.lock.toml`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetaSchemaAsset {
    pub id: &'static str,
    pub relative_path: &'static str,
    pub source_url: &'static str,
}

mod assets_inner {
    use super::MetaSchemaAsset;
    include!("meta_schemas_assets.rs");
}

pub use assets_inner::*;

/// Find a vendored asset by its relative path under `meta-schemas/`.
pub fn asset_by_path(path: &str) -> Option<&'static MetaSchemaAsset> {
    ALL.iter().find(|asset| asset.relative_path == path)
}

/// Root directory containing vendored meta-schema files.
pub fn manifest_dir() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/meta-schemas"))
}

/// Resolve a vendored asset path on disk.
pub fn resolve_path(asset: &MetaSchemaAsset) -> PathBuf {
    manifest_dir().join(asset.relative_path)
}

/// Read a vendored asset as UTF-8 text.
pub fn read(asset: &MetaSchemaAsset) -> io::Result<String> {
    fs::read_to_string(resolve_path(asset))
}

const META_SCHEMA_OPENRPC_1_4: &[u8] = include_bytes!("../meta-schemas/spec/1.4/schema.json");
const META_SCHEMA_OPENRPC_1_3: &[u8] = include_bytes!("../meta-schemas/spec/1.3/schema.json");

/// Returns vendored document meta-schema bytes for a supported OpenRPC version label.
pub fn meta_schema_bytes(version: &str) -> Option<&'static [u8]> {
    if version == "1.4" || version.starts_with("1.4.") {
        Some(META_SCHEMA_OPENRPC_1_4)
    } else if version == "1.3" || version.starts_with("1.3.") {
        Some(META_SCHEMA_OPENRPC_1_3)
    } else {
        None
    }
}
