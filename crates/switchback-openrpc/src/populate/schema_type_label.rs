//! Human-readable schema type labels for OpenRPC populate.

use serde_json::Value;

/// Derive a display type label from a JSON Schema object.
pub fn schema_type_label(value: &Value) -> String {
    let Some(obj) = value.as_object() else {
        return String::new();
    };
    if let Some(Value::String(ref_key)) = obj.get("$ref") {
        return ref_key.rsplit('/').next().unwrap_or("ref").to_string();
    }
    if let Some(t) = obj.get("type") {
        return type_field_label(t);
    }
    String::new()
}

/// Derive a parameter type label from a content descriptor and its schema value.
pub fn parameter_type_label(descriptor: &Value, schema: &Value) -> String {
    if let Some(obj) = descriptor.as_object()
        && obj.get("schema").is_none()
        && let Some(t) = obj.get("type")
    {
        return type_field_label(t);
    }
    schema_type_label(schema)
}

fn type_field_label(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Array(items) => items
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join(" | "),
        _ => String::new(),
    }
}
