//! SUMMARY chapter link builders.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use mdbook_summary::{Link, Summary, SummaryItem};
use switchback_traits::{EntityBody, Group, Layout, LinkContext, ProtobufEntityKind, StoredEntity};

use crate::summary::render_md;

/// Target markdown path for a package page under the current layout.
pub fn package_target(links: &LinkContext, layout: Layout, package: &str) -> PathBuf {
    match layout {
        Layout::Package => links.package_page_rel(package),
        Layout::Entity | Layout::Split => links.package_index_rel(package),
    }
}

pub fn entity_summary_items(
    package: &str,
    entities: &[StoredEntity],
    links: &LinkContext,
    summary_from: &Path,
) -> Vec<SummaryItem> {
    let mut out = Vec::new();
    for entity in entities {
        let Some(kind) = entity_kind(entity) else {
            continue;
        };
        let p = links
            .layout_entity_path(package, kind, &entity.name)
            .expect("entity");
        let path = render_md::link_path_for_summary(summary_from, p);
        let title = match kind {
            ProtobufEntityKind::Message => format!("Message {}", entity.name),
            ProtobufEntityKind::Enum => format!("Enum {}", entity.name),
            ProtobufEntityKind::Service => format!("Service {}", entity.name),
        };
        out.push(SummaryItem::Link(Link::new(title, path)));
    }
    out
}

pub fn flat_package_chapter(
    package: &str,
    entities: Vec<StoredEntity>,
    layout: Layout,
    package_only: bool,
    links: &LinkContext,
    summary_from: &Path,
) -> SummaryItem {
    match layout {
        Layout::Package => {
            let target = links.package_page_rel(package);
            let path = render_md::link_path_for_summary(summary_from, &target);
            SummaryItem::Link(Link::new(package, path))
        }
        Layout::Split | Layout::Entity if package_only => {
            let target = links.package_index_rel(package);
            let path = render_md::link_path_for_summary(summary_from, &target);
            SummaryItem::Link(Link::new(package, path))
        }
        Layout::Split => {
            let target = links.package_index_rel(package);
            let path = render_md::link_path_for_summary(summary_from, &target);
            let mut link = Link::new(package, path);
            link.nested_items = entity_summary_items(package, &entities, links, summary_from);
            SummaryItem::Link(link)
        }
        Layout::Entity => {
            let mut link = Link::default();
            link.name = package.to_string();
            link.location = None;
            link.nested_items = entity_summary_items(package, &entities, links, summary_from);
            SummaryItem::Link(link)
        }
    }
}

pub fn build_flat_summary(
    h1: &str,
    packages: &BTreeMap<String, (Group, Vec<StoredEntity>)>,
    layout: Layout,
    package_only: bool,
    links: &LinkContext,
    summary_from: &Path,
) -> Summary {
    let mut summary = Summary::default();
    summary.title = Some(h1.to_string());
    summary.numbered_chapters = packages
        .iter()
        .map(|(pkg, (_group, entities))| {
            flat_package_chapter(
                pkg,
                entities.clone(),
                layout,
                package_only,
                links,
                summary_from,
            )
        })
        .collect();
    summary
}

fn entity_kind(entity: &StoredEntity) -> Option<ProtobufEntityKind> {
    match &entity.body {
        EntityBody::Service(_) => Some(ProtobufEntityKind::Service),
        EntityBody::Schema(body) => {
            if body.fence_body.starts_with("enum ") {
                Some(ProtobufEntityKind::Enum)
            } else {
                Some(ProtobufEntityKind::Message)
            }
        }
        EntityBody::Operation(_) => None,
        _ => None,
    }
}
