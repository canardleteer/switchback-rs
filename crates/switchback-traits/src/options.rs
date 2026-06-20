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
    /// Selected link formatter name (default `mdbook-relative`).
    pub link_format: Option<String>,
    /// When true, skip copying companion proto markdown files.
    pub no_proto_markdown: bool,
    /// When true, skip protobuf syntax highlighting in init scaffold.
    pub no_proto_highlight: bool,
    /// When true, skip CEL syntax highlighting in init scaffold.
    pub no_cel_highlight: bool,
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
            link_format: None,
            no_proto_markdown: false,
            no_proto_highlight: false,
            no_cel_highlight: false,
        }
    }
}

impl Options {
    /// Default link formatter name.
    pub fn link_format_name(&self) -> &str {
        self.link_format.as_deref().unwrap_or("mdbook-relative")
    }

    /// Join `book_root` with a relative output path.
    pub fn output_path(&self, rel: &str) -> String {
        let rel = rel.trim_start_matches('/');
        if self.book_root == "." || self.book_root.is_empty() {
            rel.to_string()
        } else {
            format!("{}/{rel}", self.book_root.trim_end_matches('/'))
        }
    }

    /// Returns true when protobuf highlight preprocessor should be configured.
    pub fn proto_highlight(&self) -> bool {
        self.init && !self.no_proto_highlight
    }

    /// Returns true when CEL highlight preprocessor should be configured.
    pub fn cel_highlight(&self) -> bool {
        self.init && !self.no_cel_highlight
    }
    /// Returns true when a SUMMARY file should be rendered (`summary` or `init`).
    pub fn render_summary(&self) -> bool {
        self.summary || self.init
    }

    /// Returns true when only package-level SUMMARY entries are needed (`init` mode).
    pub fn package_only_summary(&self) -> bool {
        self.init
    }
}
