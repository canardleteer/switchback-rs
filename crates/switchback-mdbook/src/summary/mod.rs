//! `SUMMARY.md` generation from a reference manual.

mod chapters;
mod render_md;

use std::collections::BTreeMap;
use std::path::Path;

use mdbook_summary::Summary;
use switchback_traits::{Group, Layout, LinkContext, Options, ReferenceManual, StoredEntity};

use crate::companion::CompanionNav;
use crate::render::output_file;

pub fn render_summary(
    manual: &ReferenceManual,
    opts: &Options,
    links: &LinkContext,
    companions: &[CompanionNav],
) -> Option<switchback_traits::OutputFile> {
    if !opts.render_summary() {
        return None;
    }
    let h1 = opts.title.clone().unwrap_or_else(|| manual.title.clone());
    let summary_from = Path::new(&opts.summary_path);
    let package_only = opts.package_only_summary();

    let mut packages = BTreeMap::new();
    for module in &manual.modules {
        for contract in &module.contracts {
            for group in &contract.groups {
                if group.id.as_str().is_empty() {
                    continue;
                }
                packages.insert(
                    group.id.as_str().to_string(),
                    (group.clone(), group.entities.clone()),
                );
            }
        }
    }

    let summary = if opts.no_proto_markdown {
        chapters::build_flat_summary(
            &h1,
            &packages,
            opts.layout,
            package_only,
            links,
            summary_from,
        )
    } else {
        nav_summary(
            &h1,
            companions,
            &packages,
            opts.layout,
            package_only,
            links,
            summary_from,
        )
    };

    let content = render_md::render_summary_markdown(&summary);
    render_md::validate_summary_warn(&content);
    let path = opts.output_path(&opts.summary_path);
    Some(output_file(path, content))
}

fn nav_summary(
    h1: &str,
    companions: &[CompanionNav],
    packages: &BTreeMap<String, (Group, Vec<StoredEntity>)>,
    layout: Layout,
    package_only: bool,
    links: &LinkContext,
    summary_from: &Path,
) -> Summary {
    use mdbook_summary::{Link, SummaryItem};

    let mut items = Vec::new();
    for doc in companions {
        let target = Path::new(&links.markdown_root).join(&doc.output_rel);
        let path = render_md::link_path_for_summary(summary_from, &target);
        items.push(SummaryItem::Link(Link::new(&doc.title, path)));
    }
    for (package, (_group, entities)) in packages {
        items.push(chapters::flat_package_chapter(
            package,
            entities.clone(),
            layout,
            package_only,
            links,
            summary_from,
        ));
    }
    let mut summary = Summary::default();
    summary.title = Some(h1.to_string());
    summary.numbered_chapters = items;
    summary
}
