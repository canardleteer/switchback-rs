//! Companion navigation metadata for SUMMARY generation.

use std::path::PathBuf;

use switchback_traits::{
    Companion, module_path_from_output, source_dir_from_output, title_from_markdown,
};

/// Companion navigation metadata for SUMMARY generation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompanionNav {
    /// Output path relative to markdown root (e.g. `acme.example.v1.README.md`).
    pub output_rel: String,
    /// Link title for SUMMARY.
    pub title: String,
    /// Source directory relative to corpus root (`acme/example/v1`).
    pub source_dir: PathBuf,
    /// Source filename stem.
    pub stem: String,
    /// Protobuf-style module path for title formatting (`acme.example.v1`).
    pub module_path: Option<String>,
}

impl CompanionNav {
    /// Builds nav metadata from a wire companion, with legacy fallback.
    pub fn from_companion(c: &Companion) -> Self {
        let stem = if c.stem.is_empty() {
            c.output_name
                .strip_suffix(".md")
                .and_then(|s| s.rsplit('.').next())
                .unwrap_or("doc")
                .to_string()
        } else {
            c.stem.clone()
        };

        let title = if c.title.is_empty() {
            title_from_markdown(&stem, &c.bytes)
        } else {
            c.title.clone()
        };

        let source_dir = if c.source_dir.is_empty() {
            PathBuf::from(source_dir_from_output(&c.output_name, &stem))
        } else {
            PathBuf::from(&c.source_dir)
        };

        let module_path = module_path_from_output(&c.output_name, &stem);

        Self {
            output_rel: c.output_name.clone(),
            title,
            source_dir,
            stem,
            module_path,
        }
    }
}

pub fn render_companions(
    companions: &[Companion],
    opts: &switchback_traits::Options,
) -> Vec<switchback_traits::OutputFile> {
    if opts.no_proto_markdown {
        return Vec::new();
    }
    companions
        .iter()
        .map(|c| {
            let path = opts.output_path(&format!("{}/{}", opts.markdown_root, c.output_name));
            let content = String::from_utf8_lossy(&c.bytes).into_owned();
            crate::render::output_file(path, content)
        })
        .collect()
}
