//! Core render driver.

use switchback_traits::{
    Layout, LinkContext, Options, OutputFile, ReferenceManual, Result, SyncRenderer,
};

use crate::book_config::apply_book_config;
use crate::companion::{render_companions, CompanionNav};
use crate::formatter::MdBookRelativeFormatter;
use crate::init::{merge_init_files, scaffold_init_tree};
use crate::render::entity::render_entity_pages;
use crate::render::output_file;
use crate::render::package::render_package_page;
use crate::summary::render_summary;

/// mdBook renderer for switchback reference manuals.
#[derive(Clone, Copy, Debug, Default)]
pub struct MdBookRenderer;

impl MdBookRenderer {
    /// Renders a manual to mdBook markdown files.
    pub fn render_manual(manual: &ReferenceManual, opts: &Options) -> Result<Vec<OutputFile>> {
        let mut opts = opts.clone();
        apply_book_config(&mut opts)
            .map_err(|e| switchback_traits::SwitchbackError::render(e.to_string()))?;
        let links = LinkContext::from_manual(manual, &opts);
        let formatter = MdBookRelativeFormatter;

        let companion_nav: Vec<CompanionNav> = manual
            .modules
            .iter()
            .flat_map(|m| &m.contracts)
            .flat_map(|c| &c.companions)
            .map(CompanionNav::from_companion)
            .collect();

        let mut files = Vec::new();
        for module in &manual.modules {
            for contract in &module.contracts {
                files.extend(render_companions(&contract.companions, &opts));

                for group in &contract.groups {
                    if group.id.as_str().is_empty() {
                        continue;
                    }
                    match opts.layout {
                        Layout::Package => {
                            let (path, content) = render_package_page(
                                group,
                                &group.entities,
                                &links,
                                &opts,
                                &formatter,
                            );
                            files.push(output_file(path, content));
                        }
                        Layout::Entity | Layout::Split => {
                            for (path, content) in render_entity_pages(
                                group,
                                &group.entities,
                                &links,
                                &opts,
                                &formatter,
                            ) {
                                files.push(output_file(path, content));
                            }
                        }
                    }
                }
            }
        }

        if let Some(summary) = render_summary(manual, &opts, &links, &companion_nav) {
            files.push(summary);
        }

        if opts.init {
            let init_files = scaffold_init_tree(&opts)
                .map_err(|e| switchback_traits::SwitchbackError::render(e.to_string()))?;
            let pairs: Vec<(String, String)> = files
                .iter()
                .map(|f| {
                    (
                        f.path.clone(),
                        String::from_utf8_lossy(&f.content).into_owned(),
                    )
                })
                .collect();
            let merged = merge_init_files(&opts, &opts.book_root, init_files, &pairs);
            files = merged
                .into_iter()
                .map(|(path, content)| output_file(path, content))
                .collect();
        }

        files.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(files)
    }
}

impl SyncRenderer for MdBookRenderer {
    type Opts = Options;

    fn render(&self, manual: &ReferenceManual, opts: &Self::Opts) -> Result<Vec<OutputFile>> {
        Self::render_manual(manual, opts)
    }
}

impl switchback_traits::Renderer for MdBookRenderer {
    type Opts = Options;

    async fn render(&self, manual: &ReferenceManual, opts: &Self::Opts) -> Result<Vec<OutputFile>> {
        Self::render_manual(manual, opts)
    }
}

/// Writes rendered files to `out_root`.
pub fn write_output_files(out_root: &std::path::Path, files: &[OutputFile]) -> Result<()> {
    for file in files {
        let path = out_root.join(&file.path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                switchback_traits::SwitchbackError::render(format!(
                    "create {}: {e}",
                    parent.display()
                ))
            })?;
        }
        std::fs::write(&path, &file.content).map_err(|e| {
            switchback_traits::SwitchbackError::render(format!("write {}: {e}", path.display()))
        })?;
    }
    Ok(())
}
