//! Collect inline named Avro record/enum/fixed types for schema outbreak.

use std::collections::BTreeSet;

use serde_json::Value;

/// Walk an Avro schema JSON value and collect inline named types (`record`, `enum`, `fixed`).
///
/// Each entry is `(short_name, schema_value)` suitable for promoting to a schema entity.
/// The root value is included when it carries a `name`.
pub fn collect_named_avro_schemas(value: &Value) -> Vec<(String, Value)> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    collect_named_avro_schemas_rec(value, &mut out, &mut seen);
    out
}

fn collect_named_avro_schemas_rec(
    value: &Value,
    out: &mut Vec<(String, Value)>,
    seen: &mut BTreeSet<String>,
) {
    match value {
        Value::Object(map) => {
            let name = map
                .get("name")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty());
            let ty = map.get("type").and_then(|v| v.as_str());

            let is_named = name.is_some()
                && (ty == Some("record")
                    || ty == Some("enum")
                    || ty == Some("fixed")
                    || map.contains_key("fields")
                    || map.get("symbols").is_some());

            if let Some(name) = name
                && is_named
                && !seen.contains(name)
            {
                seen.insert(name.to_string());
                out.push((name.to_string(), value.clone()));
                if ty == Some("record") || map.contains_key("fields") {
                    for field in map
                        .get("fields")
                        .and_then(|v| v.as_array())
                        .into_iter()
                        .flatten()
                    {
                        if let Some(ft) = field.get("type") {
                            collect_named_avro_schemas_rec(ft, out, seen);
                        }
                    }
                    return;
                }
            }

            for v in map.values() {
                collect_named_avro_schemas_rec(v, out, seen);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_named_avro_schemas_rec(item, out, seen);
            }
        }
        _ => {}
    }
}
