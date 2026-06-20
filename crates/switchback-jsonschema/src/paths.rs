//! Relative path helpers for schema inputs.

use std::path::{Component, Path, PathBuf};

/// Normalize a filesystem path by resolving `.` and `..` components.
pub fn normalize_path(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(s) => out.push(s),
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            Component::RootDir => {
                out.push(Component::RootDir.as_os_str());
            }
            Component::Prefix(p) => out.push(p.as_os_str()),
        }
    }
    out
}

/// Normalize a relative directory path to forward-slash segments (no `.` or `..`).
pub fn normalize_rel_dir(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(s) => out.push(s),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {}
        }
    }
    out
}

/// Strip a leading `./` from a ref path string.
pub fn strip_dot_slash(s: &str) -> &str {
    s.strip_prefix("./").unwrap_or(s)
}

/// Return a path relative to `base`, when possible.
pub fn relativize(base: &Path, path: &Path) -> Option<PathBuf> {
    path.strip_prefix(base).ok().map(|p| p.to_path_buf())
}
