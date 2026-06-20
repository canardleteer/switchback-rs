//! Directory tree for SUMMARY with minimal subchaptering.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use mdbook_summary::{Link, Summary, SummaryItem};
use switchback_traits::{Layout, LinkContext, OpenApiSummaryLabel, StoredEntity};

use crate::companion::CompanionNav;
use crate::summary::chapters::{entity_summary_items, package_target};
use crate::summary::render_md::{link_path_for_summary, SUMMARY_MAX_DEPTH};

pub struct NavInput<'a> {
    pub companions: &'a [CompanionNav],
    pub packages: Vec<PackageAtDir<'a>>,
    pub summary_from: &'a Path,
    pub links: &'a LinkContext,
    pub openapi_summary_label: OpenApiSummaryLabel,
}

#[derive(Clone)]
pub struct PackageAtDir<'a> {
    pub rel_dir: PathBuf,
    pub package: &'a str,
    pub title: &'a str,
    pub group_dir: &'a str,
    pub family: &'a str,
    pub entities: &'a [StoredEntity],
}

#[derive(Default)]
struct DirNode {
    companions: Vec<LinkEntry>,
    packages: Vec<LinkEntry>,
    children: BTreeMap<String, DirNode>,
}

#[derive(Clone)]
struct LinkEntry {
    title: String,
    path: PathBuf,
    stem: String,
    module_path: Option<String>,
    is_package: bool,
    entity_items: Vec<SummaryItem>,
}

/// Build companion + package nav tree; entity sub-links attach at package link creation when requested.
pub fn build_summary(
    h1_title: &str,
    input: NavInput<'_>,
    layout: Layout,
    package_only: bool,
) -> Summary {
    let attach_entities = !package_only && !matches!(layout, Layout::Package);
    let mut root = DirNode::default();

    for doc in input.companions {
        let target = PathBuf::from(&input.links.markdown_root).join(&doc.output_rel);
        let path = link_path_for_summary(input.summary_from, &target);
        insert_at(
            &mut root,
            path_segments(&doc.source_dir),
            LinkEntry {
                title: doc.title.clone(),
                path,
                stem: doc.stem.clone(),
                module_path: doc.module_path.clone(),
                is_package: false,
                entity_items: Vec::new(),
            },
            InsertKind::Companion,
        );
    }

    for info in &input.packages {
        let target = package_target(input.links, layout, info.package);
        let path = link_path_for_summary(input.summary_from, &target);
        let entity_items = if attach_entities {
            entity_summary_items(
                info.package,
                info.group_dir,
                info.entities,
                info.family,
                input.links,
                input.summary_from,
                input.openapi_summary_label,
            )
        } else {
            Vec::new()
        };
        insert_at(
            &mut root,
            path_segments(&info.rel_dir),
            LinkEntry {
                title: info.title.to_string(),
                path,
                stem: String::new(),
                module_path: Some(info.package.to_string()),
                is_package: true,
                entity_items,
            },
            InsertKind::Package,
        );
    }

    let collapsed = collapse_linear_chains(root);
    let chapters = emit_dir(&collapsed, 0);

    let mut summary = Summary::default();
    summary.title = Some(h1_title.to_string());
    summary.numbered_chapters = chapters;
    summary
}

enum InsertKind {
    Companion,
    Package,
}

fn path_segments(rel_dir: &Path) -> Vec<String> {
    rel_dir
        .components()
        .filter_map(|c| match c {
            std::path::Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect()
}

fn insert_at(node: &mut DirNode, segments: Vec<String>, entry: LinkEntry, kind: InsertKind) {
    if segments.is_empty() {
        match kind {
            InsertKind::Companion => node.companions.push(entry),
            InsertKind::Package => node.packages.push(entry),
        }
        return;
    }
    let head = segments[0].clone();
    let tail = segments[1..].to_vec();
    insert_at(node.children.entry(head).or_default(), tail, entry, kind);
}

fn collapse_linear_chains(mut node: DirNode) -> DirNode {
    loop {
        if !node.companions.is_empty() || !node.packages.is_empty() || node.children.len() != 1 {
            break;
        }
        let (_key, child) = node.children.into_iter().next().expect("one child");
        node = child;
    }
    let children = node
        .children
        .into_iter()
        .map(|(k, c)| (k, collapse_linear_chains(c)))
        .collect();
    DirNode {
        companions: node.companions,
        packages: node.packages,
        children,
    }
}

fn emit_dir(node: &DirNode, depth: usize) -> Vec<SummaryItem> {
    if depth >= SUMMARY_MAX_DEPTH {
        return emit_flat(node);
    }

    let mut out = Vec::new();

    if node.companions.is_empty() && node.packages.is_empty() {
        for child in node.children.values() {
            out.extend(emit_dir(child, depth));
        }
        return out;
    }

    let mut group: Vec<LinkEntry> = node.companions.clone();
    sort_companions(&mut group);
    group.extend(node.packages.clone());

    if group.len() == 1 {
        out.push(link_item(&summary_link_entry(&group[0], false)));
    } else if !group.is_empty() {
        let first = &group[0];
        let mut link = link_item(&summary_link_entry(first, false));
        if let SummaryItem::Link(ref mut parent) = link {
            for entry in &group[1..] {
                parent
                    .nested_items
                    .push(link_item(&summary_link_entry(entry, true)));
            }
        }
        out.push(link);
    }

    let child_depth = depth.saturating_add(1);
    for child in node.children.values() {
        out.extend(emit_dir(child, child_depth));
    }

    out
}

fn emit_flat(node: &DirNode) -> Vec<SummaryItem> {
    let mut items = Vec::new();
    for c in &node.companions {
        items.push(link_item(&summary_link_entry(c, false)));
    }
    for child in node.children.values() {
        items.extend(emit_flat(child));
    }
    for p in &node.packages {
        items.push(link_item(&summary_link_entry(p, false)));
    }
    items
}

fn summary_link_entry(entry: &LinkEntry, is_nested: bool) -> LinkEntry {
    LinkEntry {
        title: format_summary_title(entry, is_nested),
        path: entry.path.clone(),
        stem: entry.stem.clone(),
        module_path: entry.module_path.clone(),
        is_package: entry.is_package,
        entity_items: entry.entity_items.clone(),
    }
}

fn format_summary_title(entry: &LinkEntry, is_nested: bool) -> String {
    if is_nested || entry.is_package {
        return entry.title.clone();
    }
    let Some(ref module_path) = entry.module_path else {
        return entry.title.clone();
    };
    if !module_path.contains('.') {
        return entry.title.clone();
    }
    if entry.title == *module_path {
        return entry.title.clone();
    }
    format!("{module_path} - {}", entry.title)
}

fn sort_companions(companions: &mut [LinkEntry]) {
    companions.sort_by(|a, b| {
        companion_stem_rank(&a.stem)
            .cmp(&companion_stem_rank(&b.stem))
            .then_with(|| a.stem.cmp(&b.stem))
    });
}

fn companion_stem_rank(stem: &str) -> u8 {
    if stem.eq_ignore_ascii_case("readme") {
        0
    } else {
        1
    }
}

fn link_item(entry: &LinkEntry) -> SummaryItem {
    let mut link = Link::new(entry.title.clone(), &entry.path);
    link.nested_items = entry.entity_items.clone();
    SummaryItem::Link(link)
}

/// Parent directory of a proto file, used for nav-tree placement.
pub fn package_rel_dir(proto_name: &str) -> PathBuf {
    Path::new(proto_name)
        .parent()
        .unwrap_or(Path::new(""))
        .to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    use switchback_traits::{Layout, LinkContext, Options, ReferenceManual};

    fn section(title: &str, module_path: &str) -> LinkEntry {
        LinkEntry {
            title: title.into(),
            path: PathBuf::from("packages/x.md"),
            stem: "README".into(),
            module_path: Some(module_path.into()),
            is_package: false,
            entity_items: Vec::new(),
        }
    }

    fn package(pkg: &str) -> LinkEntry {
        LinkEntry {
            title: pkg.into(),
            path: PathBuf::from("packages/x.md"),
            stem: String::new(),
            module_path: Some(pkg.into()),
            is_package: true,
            entity_items: Vec::new(),
        }
    }

    #[test]
    fn format_summary_title_rules() {
        assert_eq!(
            format_summary_title(&section("Acme APIs", "acme"), false),
            "Acme APIs"
        );
        assert_eq!(
            format_summary_title(&section("Example services", "acme.example"), false),
            "acme.example - Example services"
        );
        assert_eq!(
            format_summary_title(&section("acme.example.v1 README", "acme.example.v1"), false),
            "acme.example.v1 - acme.example.v1 README"
        );
        assert_eq!(
            format_summary_title(&section("Moving to v2", "acme.example.v1"), true),
            "Moving to v2"
        );
        assert_eq!(
            format_summary_title(&package("acme.example.v1"), true),
            "acme.example.v1"
        );
        assert_eq!(
            format_summary_title(&package("acme.example.v2"), false),
            "acme.example.v2"
        );
    }

    #[test]
    fn summary_parses_with_companion_tree() {
        let companions = vec![
            CompanionNav {
                output_rel: "acme.README.md".into(),
                title: "Acme APIs".into(),
                source_dir: PathBuf::from("acme"),
                stem: "README".into(),
                module_path: Some("acme".into()),
            },
            CompanionNav {
                output_rel: "acme.example.README.md".into(),
                title: "Example services".into(),
                source_dir: PathBuf::from("acme/example"),
                stem: "README".into(),
                module_path: Some("acme.example".into()),
            },
            CompanionNav {
                output_rel: "acme.example.v1.README.md".into(),
                title: "acme.example.v1 README".into(),
                source_dir: PathBuf::from("acme/example/v1"),
                stem: "README".into(),
                module_path: Some("acme.example.v1".into()),
            },
            CompanionNav {
                output_rel: "acme.example.v1.MOVING-TO-V2.md".into(),
                title: "Moving to v2".into(),
                source_dir: PathBuf::from("acme/example/v1"),
                stem: "MOVING-TO-V2".into(),
                module_path: Some("acme.example.v1".into()),
            },
        ];
        let packages = vec![PackageAtDir {
            rel_dir: package_rel_dir("acme/example/v1/a.proto"),
            package: "acme.example.v1",
            title: "acme.example.v1",
            group_dir: "acme.example.v1",
            family: "protobuf",
            entities: &[],
        }];
        let manual = ReferenceManual {
            switchback_version: "v1alpha1".into(),
            title: "Protobuf documentation".into(),
            sources: vec![],
            modules: vec![],
        };
        let opts = Options {
            summary: true,
            init: true,
            ..Options::default()
        };
        let links = LinkContext::from_manual(&manual, &opts);
        let summary = build_summary(
            "Protobuf documentation",
            NavInput {
                companions: &companions,
                packages,
                summary_from: Path::new("src/SUMMARY.md"),
                links: &links,
                openapi_summary_label: OpenApiSummaryLabel::default(),
            },
            Layout::Package,
            true,
        );
        let md = crate::summary::render_md::render_summary_markdown(&summary);
        mdbook_summary::parse_summary(&md).expect("valid SUMMARY");
        assert!(md.contains("[Acme APIs]"));
        assert!(md.contains("[acme.example - Example services]"));
        assert!(md.contains("[acme.example.v1 - acme.example.v1 README]"));
        assert!(md.contains("[Moving to v2]"));
        assert!(!md.contains("example/v1"));
        assert!(md.contains("[acme.example.v1](packages/acme.example.v1.md)"));
    }

    #[test]
    fn asymmetric_sparse_tree_summary_shape() {
        let companions = vec![CompanionNav {
            output_rel: "a.b.NOTES.md".into(),
            title: "Notes".into(),
            source_dir: PathBuf::from("a/b"),
            stem: "NOTES".into(),
            module_path: Some("a.b".into()),
        }];
        let packages = vec![
            PackageAtDir {
                rel_dir: package_rel_dir("a/b/c/d/v2/e.proto"),
                package: "shallow.v2",
                title: "shallow.v2",
                group_dir: "shallow.v2",
                family: "protobuf",
                entities: &[],
            },
            PackageAtDir {
                rel_dir: package_rel_dir("a/b/c/d/e/f/g/h/v1/stuff.proto"),
                package: "deep.v1",
                title: "deep.v1",
                group_dir: "deep.v1",
                family: "protobuf",
                entities: &[],
            },
        ];
        let manual = ReferenceManual {
            switchback_version: "v1alpha1".into(),
            title: "Protobuf documentation".into(),
            sources: vec![],
            modules: vec![],
        };
        let opts = Options::default();
        let links = LinkContext::from_manual(&manual, &opts);
        let summary = build_summary(
            "Protobuf documentation",
            NavInput {
                companions: &companions,
                packages,
                summary_from: Path::new("src/SUMMARY.md"),
                links: &links,
                openapi_summary_label: OpenApiSummaryLabel::default(),
            },
            Layout::Package,
            true,
        );
        let md = crate::summary::render_md::render_summary_markdown(&summary);
        mdbook_summary::parse_summary(&md).expect("valid SUMMARY");
        assert!(md.contains("Notes"));
        assert!(md.contains("shallow.v2"));
        assert!(md.contains("deep.v1"));
    }
}
