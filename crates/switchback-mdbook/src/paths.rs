//! Shared path helpers.

use anyhow::{Result, bail};

pub fn normalize_rel_path(path: &str, default: &str) -> Result<String> {
    let path = path.trim().trim_matches('/');
    if path.is_empty() {
        return Ok(default.to_string());
    }
    if path.contains("..") {
        bail!("path must not contain `..`: {path:?}");
    }
    Ok(path.replace('\\', "/"))
}

pub fn join_book_root(book_root: &str, rel: &str) -> String {
    let rel = rel.trim_start_matches('/');
    if book_root == "." || book_root.is_empty() {
        rel.to_string()
    } else {
        format!("{book_root}/{rel}")
    }
}
