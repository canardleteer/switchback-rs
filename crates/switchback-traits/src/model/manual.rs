//! Provenance and source-layer types.

use std::path::PathBuf;

/// Byte span within a source document.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Span {
    pub start_line: u32,
    pub start_col: u32,
    pub end_line: u32,
    pub end_col: u32,
}

/// Pointer into a [`Document`] by URI plus optional span.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Source {
    pub file: String,
    pub span: Option<Span>,
}

/// Stable provenance for a raw input document.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceRef {
    pub uri: String,
    pub commit: String,
    pub content_hash: String,
}

/// A single input document carried verbatim in the switchback source layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Document {
    pub source_ref: SourceRef,
    pub media_type: String,
    pub content: Vec<u8>,
}

/// Top-level switchback artifact every parser emits and every renderer reads.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReferenceManual {
    pub switchback_version: String,
    pub title: String,
    pub sources: Vec<Document>,
    pub modules: Vec<Module>,
}

/// A cohesive documentation unit that may span contract families.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Module {
    pub id: crate::ids::ModuleId,
    pub title: String,
    pub overview: String,
    pub contracts: Vec<super::contract::ManualContract>,
}

/// Parser-side companion file discovered beside contract inputs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompanionFile {
    pub output_name: String,
    pub bytes: Vec<u8>,
    pub source_path: PathBuf,
}
