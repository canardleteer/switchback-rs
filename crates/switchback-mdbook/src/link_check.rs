//! Resolve relative Markdown links in a generated documentation tree.

use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use switchback_traits::decode_markdown_link_path;
use switchback_traits::unique_heading_ids;
use walkdir::WalkDir;

/// A broken relative link or anchor in generated markdown.
#[derive(Debug)]
pub struct LinkError {
    /// Source markdown file.
    pub file: PathBuf,
    /// Raw link target from `](...)`.
    pub target: String,
    /// Human-readable failure reason.
    pub message: String,
}

/// Scan `root` for broken relative markdown links and heading anchors.
pub fn check_tree(root: &Path) -> Result<Vec<LinkError>> {
    let mut errors = Vec::new();
    let mut headings_by_file: HashMap<PathBuf, Vec<String>> = HashMap::new();

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        let content =
            std::fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
        headings_by_file.insert(path.to_path_buf(), extract_heading_ids(&content));
    }

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        let content =
            std::fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
        for (target, line) in extract_links(&content) {
            if target.starts_with("http://")
                || target.starts_with("https://")
                || target.starts_with("mailto:")
            {
                continue;
            }
            let (file_part, anchor) = split_anchor(&target);
            if file_part.contains(' ') {
                errors.push(LinkError {
                    file: path.to_path_buf(),
                    target: target.clone(),
                    message: format!(
                        "broken link at line {line}: unencoded space in URL (use %20)"
                    ),
                });
                continue;
            }
            let decoded_file = decode_markdown_link_path(file_part);
            let resolved = if decoded_file.is_empty() {
                path.to_path_buf()
            } else {
                path.parent().unwrap_or(root).join(&decoded_file)
            };
            if !resolved.exists() {
                errors.push(LinkError {
                    file: path.to_path_buf(),
                    target: target.clone(),
                    message: format!("broken link at line {line}: file not found"),
                });
                continue;
            }
            if let Some(anchor) = anchor {
                let headings = headings_by_file.get(&resolved).cloned().unwrap_or_default();
                if !headings.iter().any(|h| h == anchor) {
                    errors.push(LinkError {
                        file: path.to_path_buf(),
                        target: target.clone(),
                        message: format!("broken anchor #{anchor} at line {line}"),
                    });
                }
            }
        }
    }

    Ok(errors)
}

/// Fail with a formatted report when [`check_tree`] finds errors.
pub fn assert_tree(root: &Path) -> Result<()> {
    let errors = check_tree(root)?;
    if errors.is_empty() {
        return Ok(());
    }
    let mut msg = String::from("markdown link check failed:\n");
    for e in &errors {
        msg.push_str(&format!(
            "  {}: {} — {}\n",
            e.file.display(),
            e.target,
            e.message
        ));
    }
    bail!("{msg}");
}

fn split_anchor(target: &str) -> (&str, Option<&str>) {
    match target.split_once('#') {
        Some((f, a)) => (f, Some(a)),
        None => (target, None),
    }
}

fn extract_heading_ids(content: &str) -> Vec<String> {
    let titles: Vec<&str> = content
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            let hashes = line.chars().take_while(|c| *c == '#').count();
            if hashes == 0 {
                return None;
            }
            Some(line[hashes..].trim())
        })
        .collect();
    unique_heading_ids(titles)
}

fn extract_links(content: &str) -> Vec<(String, usize)> {
    let mut out = Vec::new();
    for (i, line) in content.lines().enumerate() {
        let mut rest = line;
        while let Some(start) = rest.find("](") {
            let before = &rest[..start];
            if before.rfind('[').is_some() {
                let target_start = start + 2;
                if let Some(end) = rest[target_start..].find(')') {
                    let target = &rest[target_start..target_start + end];
                    out.push((target.to_string(), i + 1));
                    rest = &rest[target_start + end + 1..];
                    continue;
                }
            }
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_tree_missing_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(dir.path().join("page.md"), "[broken](missing.md)\n").expect("write");
        let errors = check_tree(dir.path()).expect("check");
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("file not found"));
    }

    #[test]
    fn check_tree_bad_anchor() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("page.md"),
            "# Real heading\n\n[link](page.md#no-such-heading)\n",
        )
        .expect("write");
        let errors = check_tree(dir.path()).expect("check");
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("broken anchor"));
    }

    #[test]
    fn check_tree_duplicate_headings_use_unique_anchors() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("page.md"),
            "### EchoRequest\n\n### EchoResponse\n\n### EchoRequest\n\n[dup](page.md#echorequest-1)\n",
        )
        .expect("write");
        let errors = check_tree(dir.path()).expect("check");
        assert!(errors.is_empty(), "{errors:?}");
    }

    #[test]
    fn check_tree_unencoded_space_in_link() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(dir.path().join("target file.md"), "# x\n").expect("write");
        std::fs::write(dir.path().join("page.md"), "[bad](target file.md)\n").expect("write");
        let errors = check_tree(dir.path()).expect("check");
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("unencoded space"));
    }

    #[test]
    fn check_tree_percent_encoded_space_resolves() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(dir.path().join("target file.md"), "# x\n").expect("write");
        std::fs::write(dir.path().join("page.md"), "[ok](target%20file.md)\n").expect("write");
        let errors = check_tree(dir.path()).expect("check");
        assert!(errors.is_empty(), "{errors:?}");
    }

    #[test]
    fn assert_tree_surfaces_errors() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(dir.path().join("page.md"), "[broken](missing.md)\n").expect("write");
        let err = assert_tree(dir.path()).expect_err("should fail");
        let msg = err.to_string();
        assert!(msg.contains("markdown link check failed"));
        assert!(msg.contains("missing.md"));
    }
}
