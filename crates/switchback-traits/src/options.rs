//! Shared renderer and parser option shapes (data only; no token parsing).

use std::path::PathBuf;

/// Page layout for generated markdown.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Layout {
    #[default]
    Package,
    Entity,
    Split,
}

/// How to rewrite HTML-like tags in leading-comment prose.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EscapeTags {
    #[default]
    Off,
    Backticks,
    Entities,
}

/// Spine options shared across renderers (subset ported from protobuf-mdbook).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Options {
    pub init: bool,
    pub summary: bool,
    pub book_root: String,
    pub markdown_root: String,
    pub summary_path: String,
    pub book: Option<String>,
    pub mdbook_out: Option<String>,
    pub title: Option<String>,
    pub ignore_git: bool,
    pub layout: Layout,
    pub search_paths: Vec<PathBuf>,
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
    pub fn render_summary(&self) -> bool {
        self.summary || self.init
    }

    pub fn package_only_summary(&self) -> bool {
        self.init
    }
}
