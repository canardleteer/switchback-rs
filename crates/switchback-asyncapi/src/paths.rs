//! AsyncAPI-specific path and slug helpers.

use std::path::Path;

/// Slug for a tag or group id (lowercase, spaces → hyphens).
pub fn slugify(name: &str) -> String {
    name.trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Module id from application `id`, `info.title`, or file stem.
pub fn module_id_from_id_title_or_stem(root: &serde_json::Value, entry_uri: &str) -> String {
    if let Some(id) = root
        .get("id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        return slugify(id);
    }
    let title = root
        .get("info")
        .and_then(|info| info.get("title"))
        .and_then(|v| v.as_str());
    module_id_from_title_or_stem(title, entry_uri)
}

/// Module id from info title or file stem.
pub fn module_id_from_title_or_stem(title: Option<&str>, entry_uri: &str) -> String {
    title
        .filter(|s| !s.is_empty())
        .map(slugify)
        .unwrap_or_else(|| {
            Path::new(entry_uri)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("asyncapi")
                .to_string()
        })
}

pub const COMPONENTS_GROUP: &str = "components";
pub const UNTAGGED_GROUP: &str = "untagged";
