//! `$ref` → [`Reference`] resolution for OpenRPC populate.

use std::collections::BTreeMap;
use std::path::Path;

use serde_json::Value;
use switchback_jsonschema::paths::{normalize_path, strip_dot_slash};
use switchback_jsonschema::resolver::{NodeRef, RefIndex};
use switchback_traits::{EntityRef, RefKind, Reference};

use crate::paths::COMPONENTS_GROUP;

pub fn structural_refs(
    value: &Value,
    doc_uri: &str,
    module_id: &str,
    uri_to_group: &BTreeMap<String, String>,
    index: &RefIndex,
) -> Vec<Reference> {
    let mut refs = Vec::new();
    collect_ref_strings(value, &mut refs, doc_uri, module_id, uri_to_group, index);
    refs
}

fn collect_ref_strings(
    value: &Value,
    out: &mut Vec<Reference>,
    doc_uri: &str,
    module_id: &str,
    uri_to_group: &BTreeMap<String, String>,
    index: &RefIndex,
) {
    match value {
        Value::Object(map) => {
            if let Some(Value::String(ref_key)) = map.get("$ref")
                && let Some(reference) =
                    ref_to_reference(ref_key, doc_uri, module_id, uri_to_group, index)
            {
                out.push(reference);
            }
            for v in map.values() {
                collect_ref_strings(v, out, doc_uri, module_id, uri_to_group, index);
            }
        }
        Value::Array(items) => {
            for v in items {
                collect_ref_strings(v, out, doc_uri, module_id, uri_to_group, index);
            }
        }
        _ => {}
    }
}

pub fn ref_to_reference(
    ref_key: &str,
    doc_uri: &str,
    module_id: &str,
    uri_to_group: &BTreeMap<String, String>,
    index: &RefIndex,
) -> Option<Reference> {
    let (target_group, category, target_name) =
        resolve_ref_target(ref_key, doc_uri, uri_to_group, index)?;
    Some(Reference {
        target: EntityRef {
            module: module_id.to_string(),
            group: target_group,
            category: category.to_string(),
            name: target_name,
        },
        kind: if ref_key.contains("#/components/") || ref_key.contains("components/") {
            RefKind::Component
        } else {
            RefKind::Internal
        },
    })
}

pub fn resolve_ref_target(
    ref_key: &str,
    doc_uri: &str,
    uri_to_group: &BTreeMap<String, String>,
    index: &RefIndex,
) -> Option<(String, &'static str, String)> {
    let (file_part, pointer) = if let Some((file, ptr)) = ref_key.split_once('#') {
        (file, ptr.trim_start_matches('/'))
    } else if ref_key.starts_with('#') {
        ("", ref_key.trim_start_matches('/'))
    } else if ref_key.contains('/')
        || ref_key.ends_with(".yaml")
        || ref_key.ends_with(".yml")
        || ref_key.ends_with(".json")
    {
        (ref_key, "")
    } else {
        return None;
    };

    let target_doc_uri = if file_part.is_empty() {
        doc_uri.to_string()
    } else {
        let file_part = strip_dot_slash(file_part);
        let candidate = Path::new(doc_uri)
            .parent()
            .unwrap_or(Path::new(""))
            .join(file_part);
        normalize_path(&candidate)
            .to_string_lossy()
            .replace('\\', "/")
    };

    let target_group = uri_to_group
        .get(&target_doc_uri)
        .cloned()
        .unwrap_or_else(|| COMPONENTS_GROUP.to_string());

    let node = NodeRef::with_pointer(&target_doc_uri, pointer);
    if let Some(resolved) = index.ref_targets.get(&node)
        && resolved.pointer.ends_with("#recursive")
    {
        return Some((target_group, "schema", "recursive".into()));
    }

    let (category, name) = name_from_pointer(pointer, &target_doc_uri)?;
    Some((target_group, category, name))
}

fn name_from_pointer(pointer: &str, target_doc_uri: &str) -> Option<(&'static str, String)> {
    if pointer.starts_with("components/schemas/") {
        return pointer.split('/').nth(2).map(|n| ("schema", n.to_string()));
    }
    if pointer.starts_with("components/contentDescriptors/") {
        return pointer
            .split('/')
            .nth(2)
            .map(|n| ("parameter", n.to_string()));
    }
    if pointer.starts_with("$defs/") || pointer.starts_with("definitions/") {
        return pointer.split('/').nth(1).map(|n| ("schema", n.to_string()));
    }
    if pointer.is_empty() {
        let stem = Path::new(target_doc_uri)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("schema");
        return Some(("schema", stem.to_string()));
    }
    pointer
        .split('/')
        .next_back()
        .map(|n| ("schema", n.to_string()))
}

pub fn schema_ref_from_value(
    value: &Value,
    doc_uri: &str,
    module_id: &str,
    uri_to_group: &BTreeMap<String, String>,
    index: &RefIndex,
) -> Option<Reference> {
    if let Some(obj) = value.as_object() {
        if let Some(Value::String(ref_key)) = obj.get("$ref") {
            return ref_to_reference(ref_key, doc_uri, module_id, uri_to_group, index);
        }
        if obj.get("type").and_then(|v| v.as_str()) == Some("array")
            && let Some(items) = obj.get("items")
        {
            return schema_ref_from_value(items, doc_uri, module_id, uri_to_group, index);
        }
    }
    None
}
