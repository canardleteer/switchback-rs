//! Emit companion markdown files from a reference manual.

use switchback_traits::{Companion, Options, OutputFile};

use crate::render::output_file;

/// Companion navigation metadata for SUMMARY generation.
pub struct CompanionNav {
    pub output_rel: String,
    pub title: String,
}

impl CompanionNav {
    pub fn from_companion(c: &Companion) -> Self {
        let stem = c
            .output_name
            .strip_suffix(".md")
            .and_then(|s| s.rsplit('.').next())
            .unwrap_or("doc")
            .to_string();
        Self {
            output_rel: c.output_name.clone(),
            title: stem,
        }
    }
}

pub fn render_companions(companions: &[Companion], opts: &Options) -> Vec<OutputFile> {
    if opts.no_proto_markdown {
        return Vec::new();
    }
    companions
        .iter()
        .map(|c| {
            let path = opts.output_path(&format!("{}/{}", opts.markdown_root, c.output_name));
            let content = String::from_utf8_lossy(&c.bytes).into_owned();
            output_file(path, content)
        })
        .collect()
}
