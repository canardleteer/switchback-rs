//! Generic YAML binding fences for non-built-in protocols.

use serde_json::Value;

const BUILTIN: &[&str] = &["kafka", "amqp", "mqtt"];

/// Serialize non-built-in protocol bindings as a YAML fence body.
pub fn generic_binding_fence(bindings: &Value, fence_language: &str) -> Option<String> {
    let obj = bindings.as_object()?;
    let mut filtered = serde_json::Map::new();
    for (key, val) in obj {
        if BUILTIN.contains(&key.as_str()) {
            continue;
        }
        filtered.insert(key.clone(), val.clone());
    }
    if filtered.is_empty() {
        return None;
    }
    let yaml = if fence_language == "yaml" {
        serde_saphyr::to_string(&Value::Object(filtered)).ok()?
    } else {
        serde_json::to_string_pretty(&Value::Object(filtered)).ok()?
    };
    Some(yaml)
}

/// Append binding YAML to a fence body when generic bindings exist.
pub fn append_generic_bindings(existing: &str, bindings: Option<&Value>, fence_language: &str) -> String {
    let Some(bindings) = bindings else {
        return existing.to_string();
    };
    let Some(extra) = generic_binding_fence(bindings, fence_language) else {
        return existing.to_string();
    };
    if existing.trim().is_empty() {
        extra
    } else {
        format!("{existing}\n\n# other bindings\n{extra}")
    }
}
