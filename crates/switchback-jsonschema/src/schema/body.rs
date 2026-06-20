//! Produce [`SchemaBody`](switchback_traits::SchemaBody) from schema IR.

use serde_json::Value;
use switchback_traits::{Property, RefKind, Reference, SchemaBody};

use crate::loader::Doc;
use crate::schema::Schema;

/// Build a switchback schema entity body from a schema value slice.
pub fn populate_schema_body(
    value: &Value,
    doc: &Doc,
    module_id: &str,
    group_id: &str,
    group_for_file: &dyn Fn(&str, &str) -> Option<String>,
) -> SchemaBody {
    let schema = Schema::from_value(value);
    let fence_body = serialize_fence(value, doc);
    let properties = schema_properties(
        &schema,
        doc.uri.as_str(),
        module_id,
        group_id,
        group_for_file,
    );
    SchemaBody {
        fence_language: doc.fence_language().to_string(),
        fence_body,
        payload_format: String::new(),
        properties,
    }
}

fn serialize_fence(value: &Value, doc: &Doc) -> String {
    if doc.is_yaml {
        serde_saphyr::to_string(value).unwrap_or_else(|_| "{}".into())
    } else {
        serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".into())
    }
}

fn schema_properties(
    schema: &Schema,
    doc_uri: &str,
    module_id: &str,
    group_id: &str,
    group_for_file: &dyn Fn(&str, &str) -> Option<String>,
) -> Vec<Property> {
    let Schema::Object(obj) = schema else {
        return Vec::new();
    };
    let required: std::collections::BTreeSet<&str> =
        obj.required.iter().map(String::as_str).collect();
    obj.properties
        .iter()
        .filter_map(|(name, prop_schema)| {
            let Schema::Ref { ref_key } = prop_schema else {
                return None;
            };
            let (target_group, target_name) =
                resolve_ref_target(ref_key, doc_uri, group_id, group_for_file)?;
            Some(Property {
                name: name.clone(),
                schema_ref: Reference {
                    target: switchback_traits::EntityRef {
                        module: module_id.to_string(),
                        group: target_group,
                        category: "schema".to_string(),
                        name: target_name,
                    },
                    kind: if ref_key.contains("#/components/") {
                        RefKind::Component
                    } else {
                        RefKind::Internal
                    },
                },
                required: required.contains(name.as_str()),
            })
        })
        .collect()
}

fn resolve_ref_target(
    ref_key: &str,
    doc_uri: &str,
    default_group: &str,
    group_for_file: &dyn Fn(&str, &str) -> Option<String>,
) -> Option<(String, String)> {
    let (file_part, pointer) = if let Some((file, ptr)) = ref_key.split_once('#') {
        (file, ptr.trim_start_matches('/'))
    } else if ref_key.starts_with('#') {
        ("", ref_key.trim_start_matches('/'))
    } else {
        return None;
    };

    let target_group = if file_part.is_empty() {
        default_group.to_string()
    } else {
        let file_part = crate::paths::strip_dot_slash(file_part);
        group_for_file(doc_uri, file_part)?
    };

    let name = if pointer.starts_with("$defs/") || pointer.starts_with("definitions/") {
        pointer.split('/').nth(1)?.to_string()
    } else if pointer.starts_with("components/schemas/") {
        pointer.split('/').nth(2)?.to_string()
    } else if pointer.is_empty() {
        target_group.clone()
    } else {
        pointer.split('/').next_back()?.to_string()
    };

    Some((target_group, name))
}
