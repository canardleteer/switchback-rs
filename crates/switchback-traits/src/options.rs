//! Shared renderer and parser option shapes (data only; no token parsing).

use std::path::PathBuf;

/// Page layout for generated markdown.
///
/// Controls how entity pages are grouped in the output book tree. In-memory only;
/// not serialized in the switchback wire format.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Layout {
    /// One page per package/group with nested entity links.
    #[default]
    Package,
    /// One page per entity.
    Entity,
    /// Split large groups across multiple pages.
    Split,
}

/// How to rewrite HTML-like tags in leading-comment prose.
///
/// Renderer-local option; not serialized on the wire.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EscapeTags {
    /// Leave tags unchanged.
    #[default]
    Off,
    /// Wrap tags in Markdown backticks.
    Backticks,
    /// Escape tags as HTML entities.
    Entities,
}

/// Spine options shared across renderers (subset ported from protobuf-mdbook).
///
/// Passed to [`Contract::link_context`](crate::traits::Contract::link_context) and
/// renderers at output time. In-memory configuration only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Options {
    /// When true, emit initial book scaffolding (SUMMARY, directories).
    pub init: bool,
    /// When true, regenerate the mdBook `SUMMARY.md`.
    pub summary: bool,
    /// Root directory of the mdBook project.
    pub book_root: String,
    /// Relative path from `book_root` to generated markdown pages.
    pub markdown_root: String,
    /// Relative path from `book_root` to `SUMMARY.md`.
    pub summary_path: String,
    /// Optional mdBook book title override.
    pub book: Option<String>,
    /// Optional output directory for the built book.
    pub mdbook_out: Option<String>,
    /// Optional title override for the reference manual.
    pub title: Option<String>,
    /// When true, skip git-based provenance checks.
    pub ignore_git: bool,
    /// Page layout strategy for generated markdown.
    pub layout: Layout,
    /// Extra filesystem paths searched when resolving links or companions.
    pub search_paths: Vec<PathBuf>,
    /// How to escape HTML-like tags in prose comments.
    pub escape_tags: EscapeTags,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            init: false,
            summary: false,
            book_root: ".".into(),
            markdown_root: "src/packages".into(),
            summary_path: "src/SUMMARY.md".into(),
            book: None,
            mdbook_out: None,
            title: None,
            ignore_git: true,
            layout: Layout::default(),
            search_paths: Vec::new(),
            escape_tags: EscapeTags::default(),
        }
    }
}

impl Options {
    /// Returns true when a SUMMARY file should be rendered (`summary` or `init`).
    pub fn render_summary(&self) -> bool {
        self.summary || self.init
    }

    /// Returns true when only package-level SUMMARY entries are needed (`init` mode).
    pub fn package_only_summary(&self) -> bool {
        self.init
    }
}
