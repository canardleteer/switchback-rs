//! `SUMMARY.md` generation from a reference manual.

mod chapters;
mod nav_tree;
mod render_md;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use switchback_traits::{Group, LinkContext, Options, ReferenceManual, StoredEntity};

use crate::companion::CompanionNav;
use crate::init::DEFAULT_BOOK_TITLE;
use crate::render::output_file;
use crate::summary::chapters::{
    build_flat_summary, build_mixed_family_summary, build_openapi_summary,
};
use crate::summary::nav_tree::{NavInput, PackageAtDir, build_summary, package_rel_dir};

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
    let openapi_only = !packages.is_empty() && packages.iter().all(|p| p.family == "openapi");
    let mixed_family = contract_families(manual).len() > 1;

    let summary = if mixed_family {
        build_mixed_family_summary(
            &h1,
            manual,
            &packages,
            companions,
            opts.layout,
            package_only,
            links,
            summary_from,
            opts.openapi_summary_label,
        )
    } else if openapi_only {
        build_openapi_summary(
            &h1,
            &packages,
            opts.layout,
            package_only,
            links,
            summary_from,
            opts.openapi_summary_label,
        )
    } else if opts.no_proto_markdown {
        build_flat_summary(
            &h1,
            &flat_packages_from_manual(manual),
            opts.layout,
            package_only,
            links,
            summary_from,
            opts.openapi_summary_label,
        )
    } else {
        build_summary(
            &h1,
            NavInput {
                companions,
                packages,
                summary_from,
                links,
                openapi_summary_label: opts.openapi_summary_label,
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

fn packages_nav_input(manual: &ReferenceManual) -> Vec<PackageAtDir<'_>> {
    let mut out = Vec::new();
    for module in &manual.modules {
        for contract in &module.contracts {
            for group in &contract.groups {
                let package = group.id.as_str();
                if package.is_empty() {
                    continue;
                }
                let rel_dir = if contract.family == "openapi" {
                    PathBuf::from(&group.dir)
                } else {
                    group
                        .source
                        .as_ref()
                        .map(|s| package_rel_dir(&s.file))
                        .unwrap_or_default()
                };
                out.push(PackageAtDir {
                    rel_dir,
                    package,
                    title: &group.title,
                    group_dir: &group.dir,
                    family: &contract.family,
                    entities: &group.entities,
                });
            }
        }
    }
    out
}

fn contract_families(manual: &ReferenceManual) -> std::collections::BTreeSet<&str> {
    let mut families = std::collections::BTreeSet::new();
    for module in &manual.modules {
        for contract in &module.contracts {
            families.insert(contract.family.as_str());
        }
    }
    families
}

fn flat_packages_from_manual(
    manual: &ReferenceManual,
) -> BTreeMap<String, (String, Group, Vec<StoredEntity>)> {
    let mut packages = BTreeMap::new();
    for module in &manual.modules {
        for contract in &module.contracts {
            for group in &contract.groups {
                if group.id.as_str().is_empty() {
                    continue;
                }
                packages.insert(
                    group.id.as_str().to_string(),
                    (
                        contract.family.clone(),
                        group.clone(),
                        group.entities.clone(),
                    ),
                );
            }
        }
    }
    packages
}
