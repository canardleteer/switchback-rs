//! Provenance and source-layer types.

use std::path::PathBuf;

/// Byte span within a source document (1-based line/column coordinates).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Span {
    /// Start line (1-based).
    pub start_line: u32,
    /// Start column (1-based).
    pub start_col: u32,
    /// End line (1-based).
    pub end_line: u32,
    /// End column (1-based).
    pub end_col: u32,
}

/// Pointer into a [`Document`] by URI plus optional span.
///
/// Serialized on the wire on [`Group`](super::contract::Group) and
/// [`StoredEntity`](super::entity::StoredEntity) provenance fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Source {
    /// URI of the source [`Document`] in the manual's source layer.
    pub file: String,
    /// Optional line/column span within the document.
    pub span: Option<Span>,
}

/// Stable provenance for a raw input document.
///
/// Serialized on the wire as part of the switchback source layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceRef {
    /// Canonical URI for the input document.
    pub uri: String,
    /// Source control commit hash, when known.
    pub commit: String,
    /// Content hash for change detection.
    pub content_hash: String,
}

/// A single input document carried verbatim in the switchback source layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Document {
    /// Stable provenance metadata for this document.
    pub source_ref: SourceRef,
    /// MIME type of the raw content (e.g. `"application/yaml"`).
    pub media_type: String,
    /// Raw input bytes as read from disk or fetched remotely.
    pub content: Vec<u8>,
}

/// Top-level switchback artifact every parser emits and every renderer reads.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReferenceManual {
    /// Switchback container format version (distinct from contract spec versions).
    pub switchback_version: String,
    /// Human-readable title for the reference manual.
    pub title: String,
    /// Verbatim source documents preserved for provenance and re-parse.
    pub sources: Vec<Document>,
    /// Top-level documentation modules grouping contracts.
    pub modules: Vec<Module>,
}

/// A cohesive documentation unit that may span contract families.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Module {
    /// Stable module identifier ([`ModuleId`](crate::ModuleId)).
    pub id: crate::ids::ModuleId,
    /// Human-readable module title.
    pub title: String,
    /// Overview prose for the module landing page.
    pub overview: String,
    /// Contracts belonging to this module.
    pub contracts: Vec<super::contract::ManualContract>,
}

/// Parser-side companion file discovered beside contract inputs.
///
/// Used during parse and companion discovery; converted to wire [`Companion`](super::contract::Companion)
/// via [`companion_files_to_stored`](crate::traits::companion_files_to_stored). The
/// `source_path` field is parser-local and not serialized on the wire.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompanionFile {
    /// Output filename for the companion in the rendered book.
    pub output_name: String,
    /// Raw companion file bytes.
    pub bytes: Vec<u8>,
    /// Filesystem path where the companion was discovered.
    ///
    /// Parser-local provenance only; not serialized on the wire.
    pub source_path: PathBuf,
}
