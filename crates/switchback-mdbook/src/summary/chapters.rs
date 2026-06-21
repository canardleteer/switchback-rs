//! SUMMARY chapter link builders.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use mdbook_summary::{Link, Summary, SummaryItem};
use switchback_traits::{
    EntityBody, Group, Layout, LinkContext, OpenApiSummaryLabel, ProtobufEntityKind,
    ReferenceManual, StoredEntity, entity_category_dir, entity_rel_path,
};

use crate::companion::CompanionNav;
use crate::render::asyncapi::{
    asyncapi_category_rank, asyncapi_summary_link_text, asyncapi_summary_sort_key,
    renderable_asyncapi_entities,
};
use crate::render::openapi::{
    openapi_category_rank, openapi_summary_link_text, openapi_summary_sort_key,
    renderable_openapi_entities,
};
use crate::summary::nav_tree::{NavInput, build_summary};
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
    openapi_summary_label: OpenApiSummaryLabel,
) -> Vec<SummaryItem> {
    if family == "openapi" {
        return openapi_entity_summary_items(
            package,
            group_dir,
            entities,
            links,
            summary_from,
            openapi_summary_label,
        );
    }

    if family == "asyncapi" {
        return asyncapi_entity_summary_items(package, group_dir, entities, links, summary_from);
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
    summary_label: OpenApiSummaryLabel,
) -> Vec<SummaryItem> {
    let mut renderable: Vec<_> = renderable_openapi_entities(entities);
    renderable.sort_by(|a, b| {
        openapi_category_rank(&a.category)
            .cmp(&openapi_category_rank(&b.category))
            .then_with(|| {
                openapi_summary_sort_key(a, summary_label)
                    .cmp(&openapi_summary_sort_key(b, summary_label))
            })
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
        let title = openapi_summary_link_text(entity, summary_label);
        out.push(SummaryItem::Link(Link::new(title, path)));
    }
    out
}

fn asyncapi_entity_summary_items(
    package: &str,
    group_dir: &str,
    entities: &[StoredEntity],
    links: &LinkContext,
    summary_from: &Path,
) -> Vec<SummaryItem> {
    let mut renderable: Vec<_> = renderable_asyncapi_entities(entities);
    renderable.sort_by(|a, b| {
        asyncapi_category_rank(&a.category)
            .cmp(&asyncapi_category_rank(&b.category))
            .then_with(|| asyncapi_summary_sort_key(a).cmp(&asyncapi_summary_sort_key(b)))
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
                let rel = entity_rel_path(
                    group_dir,
                    entity_category_dir(&entity.category),
                    &entity.name,
                );
                PathBuf::from(format!("{}/{}", links.markdown_root, rel))
            });
        let path = render_md::link_path_for_summary(summary_from, &p);
        let title = asyncapi_summary_link_text(entity);
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
    openapi_summary_label: OpenApiSummaryLabel,
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
            link.nested_items = entity_summary_items(
                package,
                group_dir,
                &entities,
                family,
                links,
                summary_from,
                openapi_summary_label,
            );
            SummaryItem::Link(link)
        }
        Layout::Entity => {
            let mut link = Link::default();
            link.name = display_title.to_string();
            link.location = None;
            link.nested_items = entity_summary_items(
                package,
                group_dir,
                &entities,
                family,
                links,
                summary_from,
                openapi_summary_label,
            );
            SummaryItem::Link(link)
        }
    }
}

pub fn build_asyncapi_summary(
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
                OpenApiSummaryLabel::default(),
            )
        })
        .collect();
    summary
}

pub fn build_openapi_summary(
    h1: &str,
    packages: &[super::nav_tree::PackageAtDir<'_>],
    layout: Layout,
    package_only: bool,
    links: &LinkContext,
    summary_from: &Path,
    openapi_summary_label: OpenApiSummaryLabel,
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
                openapi_summary_label,
            )
        })
        .collect();
    summary
}

/// Top-level SUMMARY with one section per contract family (HTTP + gRPC, etc.).
#[allow(clippy::too_many_arguments)]
pub fn build_mixed_family_summary(
    h1: &str,
    manual: &ReferenceManual,
    packages: &[super::nav_tree::PackageAtDir<'_>],
    companions: &[CompanionNav],
    layout: Layout,
    package_only: bool,
    links: &LinkContext,
    summary_from: &Path,
    openapi_summary_label: OpenApiSummaryLabel,
) -> Summary {
    use mdbook_summary::{Link, Summary, SummaryItem};

    let mut summary = Summary::default();
    summary.title = Some(h1.to_string());

    let sections: [(&str, &str); 3] = [
        ("openapi", "HTTP (OpenAPI)"),
        ("protobuf", "gRPC (Protobuf)"),
        ("asyncapi", "Events (AsyncAPI)"),
    ];

    for (family, section_title) in sections {
        let family_packages: Vec<_> = packages
            .iter()
            .filter(|pkg| pkg.family == family)
            .cloned()
            .collect();
        if family_packages.is_empty() {
            continue;
        }

        let nested = match family {
            "openapi" => {
                build_openapi_summary(
                    "",
                    &family_packages,
                    layout,
                    package_only,
                    links,
                    summary_from,
                    openapi_summary_label,
                )
                .numbered_chapters
            }
            "asyncapi" => {
                build_asyncapi_summary(
                    "",
                    &family_packages,
                    layout,
                    package_only,
                    links,
                    summary_from,
                )
                .numbered_chapters
            }
            _ => {
                let family_companions: Vec<CompanionNav> = manual
                    .modules
                    .iter()
                    .flat_map(|module| &module.contracts)
                    .filter(|contract| contract.family == family)
                    .flat_map(|contract| {
                        contract.companions.iter().map(CompanionNav::from_companion)
                    })
                    .collect();
                build_summary(
                    "",
                    NavInput {
                        companions: &family_companions,
                        packages: family_packages,
                        summary_from,
                        links,
                        openapi_summary_label,
                    },
                    layout,
                    package_only,
                )
                .numbered_chapters
            }
        };

        let mut section = Link::default();
        section.name = section_title.to_string();
        section.location = None;
        section.nested_items = nested;
        summary.numbered_chapters.push(SummaryItem::Link(section));
    }

    let _ = companions;
    summary
}

pub fn build_flat_summary(
    h1: &str,
    packages: &BTreeMap<String, (String, Group, Vec<StoredEntity>)>,
    layout: Layout,
    package_only: bool,
    links: &LinkContext,
    summary_from: &Path,
    openapi_summary_label: OpenApiSummaryLabel,
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
                openapi_summary_label,
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
