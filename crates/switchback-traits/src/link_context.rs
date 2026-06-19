//! Cross-reference index shared by renderers and link formatters (data shell).

use std::collections::HashMap;
use std::path::PathBuf;

use crate::options::Layout;
use crate::EntityRef;

/// Entity output path index. Population logic is deferred to helper crates.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LinkContext {
    pub layout: Layout,
    pub book_root: String,
    pub markdown_root: String,
    pub entity_paths: HashMap<EntityRef, PathBuf>,
}

impl LinkContext {
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
