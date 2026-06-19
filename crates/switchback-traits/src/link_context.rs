//! Cross-reference index shared by renderers and link formatters (data shell).

use std::collections::HashMap;
use std::path::PathBuf;

use crate::options::Layout;
use crate::EntityRef;

/// Entity output path index used by [`LinkFormatter`](crate::traits::LinkFormatter).
///
/// Maps resolved [`EntityRef`] addresses to filesystem paths relative to the book
/// layout. Population logic is deferred to helper crates; this crate defines the
/// data shell only. In-memory only; not serialized on the wire.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LinkContext {
    /// Active page layout (controls relative path shape).
    pub layout: Layout,
    /// mdBook project root used when resolving relative links.
    pub book_root: String,
    /// Markdown output root relative to `book_root`.
    pub markdown_root: String,
    /// Resolved output path for each entity in the current render pass.
    pub entity_paths: HashMap<EntityRef, PathBuf>,
}

impl LinkContext {
    /// Creates an empty context with layout and path roots but no entity entries.
    pub fn empty(
        layout: Layout,
        book_root: impl Into<String>,
        markdown_root: impl Into<String>,
    ) -> Self {
        Self {
            layout,
            book_root: book_root.into(),
            markdown_root: markdown_root.into(),
            entity_paths: HashMap::new(),
        }
    }
}
