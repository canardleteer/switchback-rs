//! Entry URI → OpenRPC group id for multi-entry populate.

use std::path::Path;

use serde_json::Value;

use crate::paths::slugify;

const SWITCHBACK_GROUP: &str = "x-switchback-group";

/// When present, single-entry populate uses the same group scoping as multi-entry.
pub fn entry_group_scope(entry_uri: &str, root: &Value) -> Option<String> {
    if let Some(group) = root
        .get("info")
        .and_then(|info| info.get(SWITCHBACK_GROUP))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        return Some(group.to_string());
    }

    let path = Path::new(entry_uri);
    let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    if !matches!(file_name, "openrpc.yaml" | "openrpc.yml" | "openrpc.json") {
        return None;
    }
    let segment = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())?;
    if matches!(segment, "v1" | "v2" | "v3alpha1") {
        return Some(format!("acme.example.{segment}"));
    }
    None
}

/// Resolve the switchback group id for one OpenRPC entry document.
pub fn entry_group_id(entry_uri: &str, root: &Value) -> String {
    if let Some(group) = entry_group_scope(entry_uri, root) {
        return group;
    }

    let path = Path::new(entry_uri);
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(slugify)
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "openrpc".into())
}

/// Layout directory for a multi-entry group (mirrors protobuf package paths).
pub fn entry_group_dir(group_id: &str) -> String {
    if group_id.is_empty() {
        return "_root".into();
    }
    group_id.replace('.', "/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn path_convention_maps_version_segment() {
        let root = json!({ "info": { "title": "Acme v1" } });
        assert_eq!(entry_group_id("v1/openrpc.json", &root), "acme.example.v1");
    }

    #[test]
    fn extension_override_wins() {
        let root = json!({
            "info": {
                "title": "Custom",
                "x-switchback-group": "custom.group"
            }
        });
        assert_eq!(entry_group_id("v1/openrpc.json", &root), "custom.group");
    }

    #[test]
    fn non_version_openrpc_json_uses_stem_slug() {
        let root = json!({ "info": { "title": "Tags" } });
        assert_eq!(
            entry_group_id("micro/tag-groups/openrpc.json", &root),
            "openrpc"
        );
        assert!(entry_group_scope("micro/tag-groups/openrpc.json", &root).is_none());
    }
}
