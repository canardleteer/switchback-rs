//! SUMMARY chapter link builders.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use mdbook_summary::{Link, Summary, SummaryItem};
use switchback_traits::{EntityBody, Group, Layout, LinkContext, ProtobufEntityKind, StoredEntity};

use crate::render::openapi::{openapi_category_rank, renderable_openapi_entities, summary_prefix};
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
    group_dir: &str,
    entities: &[StoredEntity],
    family: &str,
    links: &LinkContext,
    summary_from: &Path,
) -> Vec<SummaryItem> {
    if family == "openapi" {
        return openapi_entity_summary_items(package, group_dir, entities, links, summary_from);
    }

    let mut out = Vec::new();
    for entity in entities {
        let Some(kind) = protobuf_entity_kind(entity) else {
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

fn openapi_entity_summary_items(
    package: &str,
    group_dir: &str,
    entities: &[StoredEntity],
    links: &LinkContext,
    summary_from: &Path,
) -> Vec<SummaryItem> {
    let mut renderable: Vec<_> = renderable_openapi_entities(entities);
    renderable.sort_by(|a, b| {
        openapi_category_rank(&a.category)
            .cmp(&openapi_category_rank(&b.category))
            .then_with(|| a.title.cmp(&b.title))
    });

    let mut out = Vec::new();
    for entity in renderable {
        let p = links
            .entity_paths
            .iter()
            .find(|(key, _)| {
                key.group == package && key.category == entity.category && key.name == entity.name
            })
            .map(|(_, path)| path.clone())
            .unwrap_or_else(|| {
                let rel = switchback_traits::entity_rel_path(
                    group_dir,
                    switchback_traits::entity_category_dir(&entity.category),
                    &entity.name,
                );
                PathBuf::from(format!("{}/{}", links.markdown_root, rel))
            });
        let path = render_md::link_path_for_summary(summary_from, &p);
        let title = format!("{} {}", summary_prefix(&entity.category), entity.name);
        out.push(SummaryItem::Link(Link::new(title, path)));
    }
    out
}

#[allow(clippy::too_many_arguments)]
pub fn flat_package_chapter(
    package: &str,
    display_title: &str,
    group_dir: &str,
    entities: Vec<StoredEntity>,
    family: &str,
    layout: Layout,
    package_only: bool,
    links: &LinkContext,
    summary_from: &Path,
) -> SummaryItem {
    match layout {
        Layout::Package => {
            let target = links.package_page_rel(package);
            let path = render_md::link_path_for_summary(summary_from, &target);
            SummaryItem::Link(Link::new(display_title, path))
        }
        Layout::Split | Layout::Entity if package_only => {
            let target = links.package_index_rel(package);
            let path = render_md::link_path_for_summary(summary_from, &target);
            SummaryItem::Link(Link::new(display_title, path))
        }
        Layout::Split => {
            let target = links.package_index_rel(package);
            let path = render_md::link_path_for_summary(summary_from, &target);
            let mut link = Link::new(display_title, path);
            link.nested_items =
                entity_summary_items(package, group_dir, &entities, family, links, summary_from);
            SummaryItem::Link(link)
        }
        Layout::Entity => {
            let mut link = Link::default();
            link.name = display_title.to_string();
            link.location = None;
            link.nested_items =
                entity_summary_items(package, group_dir, &entities, family, links, summary_from);
            SummaryItem::Link(link)
        }
    }
}

pub fn build_openapi_summary(
    h1: &str,
    packages: &[super::nav_tree::PackageAtDir<'_>],
    layout: Layout,
    package_only: bool,
    links: &LinkContext,
    summary_from: &Path,
) -> Summary {
    let mut summary = Summary::default();
    summary.title = Some(h1.to_string());
    summary.numbered_chapters = packages
        .iter()
        .map(|pkg| {
            flat_package_chapter(
                pkg.package,
                pkg.title,
                pkg.group_dir,
                pkg.entities.to_vec(),
                pkg.family,
                layout,
                package_only,
                links,
                summary_from,
            )
        })
        .collect();
    summary
}

pub fn build_flat_summary(
    h1: &str,
    packages: &BTreeMap<String, (String, Group, Vec<StoredEntity>)>,
    layout: Layout,
    package_only: bool,
    links: &LinkContext,
    summary_from: &Path,
) -> Summary {
    let mut summary = Summary::default();
    summary.title = Some(h1.to_string());
    summary.numbered_chapters = packages
        .iter()
        .map(|(pkg, (family, group, entities))| {
            flat_package_chapter(
                pkg,
                &group.title,
                &group.dir,
                entities.clone(),
                family,
                layout,
                package_only,
                links,
                summary_from,
            )
        })
        .collect();
    summary
}

fn protobuf_entity_kind(entity: &StoredEntity) -> Option<ProtobufEntityKind> {
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
