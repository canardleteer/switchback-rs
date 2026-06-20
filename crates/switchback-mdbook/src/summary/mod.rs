//! `SUMMARY.md` generation from a reference manual.

mod chapters;
mod nav_tree;
mod render_md;

use std::collections::BTreeMap;
use std::path::Path;

use switchback_traits::{Group, LinkContext, Options, ReferenceManual, StoredEntity};

use crate::companion::CompanionNav;
use crate::init::DEFAULT_BOOK_TITLE;
use crate::render::output_file;
use crate::summary::nav_tree::{build_summary, package_rel_dir, NavInput, PackageAtDir};

pub fn render_summary(
    manual: &ReferenceManual,
    opts: &Options,
    links: &LinkContext,
    companions: &[CompanionNav],
) -> Option<switchback_traits::OutputFile> {
    if !opts.render_summary() {
        return None;
    }
    let h1 = summary_h1_title(manual, opts);
    let summary_from = Path::new(&opts.summary_path);
    let package_only = opts.package_only_summary();
    let packages = packages_nav_input(manual);

    let summary = if opts.no_proto_markdown {
        chapters::build_flat_summary(
            &h1,
            &flat_packages_from_manual(manual),
            opts.layout,
            package_only,
            links,
            summary_from,
        )
    } else {
        build_summary(
            &h1,
            NavInput {
                companions,
                packages,
                summary_from,
                links,
            },
            opts.layout,
            package_only,
        )
    };

    let content = render_md::render_summary_markdown(&summary);
    render_md::validate_summary_warn(&content);
    let path = opts.output_path(&opts.summary_path);
    Some(output_file(path, content))
}

fn summary_h1_title(manual: &ReferenceManual, opts: &Options) -> String {
    if opts.init {
        opts.title
            .clone()
            .unwrap_or_else(|| DEFAULT_BOOK_TITLE.to_string())
    } else {
        manual.title.clone()
    }
}

fn packages_nav_input(manual: &ReferenceManual) -> BTreeMap<String, PackageAtDir<'_>> {
    let mut out = BTreeMap::new();
    for module in &manual.modules {
        for contract in &module.contracts {
            for group in &contract.groups {
                let package = group.id.as_str();
                if package.is_empty() {
                    continue;
                }
                let rel_dir = group
                    .source
                    .as_ref()
                    .map(|s| package_rel_dir(&s.file))
                    .unwrap_or_default();
                out.insert(
                    package.to_string(),
                    PackageAtDir {
                        rel_dir,
                        package,
                        entities: &group.entities,
                    },
                );
            }
        }
    }
    out
}

fn flat_packages_from_manual(
    manual: &ReferenceManual,
) -> BTreeMap<String, (Group, Vec<StoredEntity>)> {
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
    packages
}
