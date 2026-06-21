//! Human-readable schema type labels for OpenAPI populate.

use serde_json::Value;

/// Derive a display type label from a JSON Schema or OpenAPI schema object.
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

/// Derive a parameter type label from the parameter object and its schema value.
pub fn parameter_type_label(param: &Value, schema: &Value) -> String {
    if let Some(obj) = param.as_object()
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn inline_string_schema() {
        assert_eq!(schema_type_label(&json!({"type": "string"})), "string");
    }

    #[test]
    fn ref_tail_name() {
        assert_eq!(
            schema_type_label(&json!({"$ref": "#/components/schemas/mark"})),
            "mark"
        );
    }

    #[test]
    fn openapi31_nullable_type_array() {
        assert_eq!(
            schema_type_label(&json!({"type": ["string", "null"]})),
            "string | null"
        );
    }
}
