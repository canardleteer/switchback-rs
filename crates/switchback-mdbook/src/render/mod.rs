//! Markdown page rendering from a [`ReferenceManual`](switchback_traits::ReferenceManual).

pub mod entity;
pub mod fence;
pub mod markdown_doc;
pub mod package;

use switchback_traits::OutputFile;

pub(crate) fn md_heading(level: usize, text: &str) -> String {
    let mut s = "#".repeat(level);
    s.push(' ');
    s.push_str(text);
    s.push_str("\n\n");
    s
}

pub(crate) fn push_paragraph_break(out: &mut String) {
    out.push_str("\n\n");
}

pub fn output_file(path: String, content: String) -> OutputFile {
    OutputFile {
        path,
        content: content.into_bytes(),
    }
}
