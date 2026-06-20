//! Cross-reference index shared by renderers and link formatters.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::layout_paths::{
    heading_slug, layout_entity_rel_path, package_index_rel, package_page_rel,
    relative_path_from_dir, LayoutEntityKey, ProtobufEntityKind,
};
use crate::options::{Layout, Options};
use crate::EntityRef;
use crate::{EntityBody, ReferenceManual, StoredEntity};

/// Entity output path index used by [`LinkFormatter`](crate::traits::LinkFormatter).
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
    /// Layout-aware protobuf entity paths (message/enum/service).
    layout_entities: HashMap<LayoutEntityKey, PathBuf>,
    /// Current page path used when formatting relative entity links.
    pub render_from: Option<PathBuf>,
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
            layout_entities: HashMap::new(),
            render_from: None,
        }
    }

    /// Builds a link context from a reference manual and render options.
    pub fn from_manual(manual: &ReferenceManual, opts: &Options) -> Self {
        let mut ctx = Self::empty(opts.layout, &opts.book_root, &opts.markdown_root);
        for module in &manual.modules {
            for contract in &module.contracts {
                for group in &contract.groups {
                    if group.id.as_str().is_empty() {
                        continue;
                    }
                    for entity in &group.entities {
                        ctx.register_stored_entity(
                            module.id.as_str(),
                            group.id.as_str(),
                            entity,
                            opts.layout,
                            &opts.markdown_root,
                        );
                    }
                }
            }
        }
        ctx
    }

    /// Registers one stored entity in the path index.
    pub fn register_stored_entity(
        &mut self,
        module: &str,
        package: &str,
        entity: &StoredEntity,
        layout: Layout,
        markdown_root: &str,
    ) {
        if let Some(kind) = protobuf_entity_kind(entity) {
            let key = LayoutEntityKey {
                package: package.to_string(),
                kind,
                name: entity.name.clone(),
            };
            let path = layout_entity_rel_path(layout, markdown_root, &key);
            self.layout_entities.insert(key, path);
        }
        let entity_ref = EntityRef {
            module: module.to_string(),
            group: package.to_string(),
            category: entity.category.clone(),
            name: entity.name.clone(),
        };
        let path = layout_entity_rel_path(
            layout,
            markdown_root,
            &LayoutEntityKey {
                package: package.to_string(),
                kind: protobuf_entity_kind(entity).unwrap_or(ProtobufEntityKind::Message),
                name: entity.name.clone(),
            },
        );
        self.entity_paths.insert(entity_ref, path);
    }

    /// Iterate layout entity keys registered in this context.
    pub fn layout_entity_keys(&self) -> impl Iterator<Item = &LayoutEntityKey> {
        self.layout_entities.keys()
    }

    /// Relative path to a package rollup page.
    pub fn package_page_rel(&self, package: &str) -> PathBuf {
        package_page_rel(&self.markdown_root, package)
    }

    /// Relative path to a package index page.
    pub fn package_index_rel(&self, package: &str) -> PathBuf {
        package_index_rel(self.layout, &self.markdown_root, package)
    }

    /// Lookup layout entity path.
    pub fn layout_entity_path(
        &self,
        package: &str,
        kind: ProtobufEntityKind,
        name: &str,
    ) -> Option<&PathBuf> {
        self.layout_entities.get(&LayoutEntityKey {
            package: package.to_string(),
            kind,
            name: name.to_string(),
        })
    }

    /// Format a markdown link to an entity from `from`.
    pub fn link_from(
        &self,
        from: &Path,
        package: &str,
        kind: ProtobufEntityKind,
        name: &str,
    ) -> String {
        let Some(target) = self.layout_entity_path(package, kind, name) else {
            return format!("`.{package}.{name}`");
        };
        match self.layout {
            Layout::Package => self.package_layout_link(from, target, name),
            Layout::Entity | Layout::Split => self.file_link(from, target),
        }
    }

    /// Format a markdown link for a protobuf FQN type reference.
    pub fn link_type(&self, from: &Path, fqn: &str) -> String {
        let Some((pkg, ident)) = split_proto_type_name(fqn) else {
            return format!("`{fqn}`");
        };
        if self
            .layout_entity_path(pkg, ProtobufEntityKind::Message, ident)
            .is_some()
        {
            return self.link_from(from, pkg, ProtobufEntityKind::Message, ident);
        }
        if self
            .layout_entity_path(pkg, ProtobufEntityKind::Enum, ident)
            .is_some()
        {
            return self.link_from(from, pkg, ProtobufEntityKind::Enum, ident);
        }
        format!("`{fqn}`")
    }

    /// SUMMARY nav link from `from` to `target`.
    pub fn summary_link(&self, from: &Path, target: &Path, title: &str) -> String {
        let from_dir = from.parent().unwrap_or(Path::new(""));
        let rel = relative_path_from_dir(from_dir, target);
        format!("[{title}]({rel})")
    }

    fn file_link(&self, from: &Path, target: &Path) -> String {
        let from_dir = from.parent().unwrap_or(Path::new(""));
        let rel = relative_path_from_dir(from_dir, target);
        let label = target.file_stem().unwrap_or_default().to_string_lossy();
        format!("[{label}]({rel})")
    }

    fn package_layout_link(&self, from: &Path, target: &Path, name: &str) -> String {
        if from == target {
            format!("[{name}](#{})", heading_slug(name))
        } else {
            let from_dir = from.parent().unwrap_or(Path::new(""));
            let rel = relative_path_from_dir(from_dir, target);
            format!("[{name}]({rel}#{})", heading_slug(name))
        }
    }
}

fn protobuf_entity_kind(entity: &StoredEntity) -> Option<ProtobufEntityKind> {
    match &entity.body {
        EntityBody::Service(_) => Some(ProtobufEntityKind::Service),
        EntityBody::Operation(_) => None,
        EntityBody::Schema(body) => {
            if body.fence_body.starts_with("enum ") {
                Some(ProtobufEntityKind::Enum)
            } else {
                Some(ProtobufEntityKind::Message)
            }
        }
        _ => None,
    }
}

fn split_proto_type_name(fqn: &str) -> Option<(&str, &str)> {
    let fqn = fqn.strip_prefix('.').unwrap_or(fqn);
    let (pkg, name) = fqn.rsplit_once('.')?;
    if pkg.is_empty() || name.is_empty() {
        return None;
    }
    Some((pkg, name))
}
