//! Route message/schema payloads to JSON Schema or Avro populate.

use serde_json::Value;
use switchback_avro::populate_avro_schema_body;
use switchback_jsonschema::loader::Doc;
use switchback_jsonschema::schema::populate_schema_body;
use switchback_traits::SchemaBody;

use crate::populate::PopulateCtx;

const AVRO_JSON: &str = "application/vnd.apache.avro+json";
const AVRO_YAML: &str = "application/vnd.apache.avro+yaml";

pub fn is_avro_schema_format(format: &str) -> bool {
    format == AVRO_JSON || format == AVRO_YAML || format.contains("avro")
}

pub fn schema_format(value: &Value) -> Option<&str> {
    value
        .get("schemaFormat")
        .and_then(|v| v.as_str())
        .or_else(|| {
            value
                .get("payload")
                .and_then(|p| p.get("schemaFormat"))
                .and_then(|v| v.as_str())
        })
}

pub fn payload_value(value: &Value) -> &Value {
    if value.get("payload").is_some() {
        value.get("payload").unwrap_or(value)
    } else {
        value
    }
}

pub fn populate_schema_dispatch(
    value: &Value,
    doc: &Doc,
    ctx: &PopulateCtx<'_>,
    group_id: &str,
) -> SchemaBody {
    let payload = payload_value(value);
    if let Some(format) = schema_format(value).or_else(|| schema_format(payload))
        && is_avro_schema_format(format)
    {
        let schema_val = payload.get("schema").unwrap_or(payload);
        return populate_avro_schema_body(schema_val, ctx.module_id, group_id, Some(format));
    }

    let schema_val = if payload.get("schema").is_some() {
        payload.get("schema").unwrap_or(payload)
    } else {
        payload
    };

    populate_schema_body(
        schema_val,
        doc,
        ctx.module_id,
        group_id,
        &|base_uri, file_part| resolve_group_for_ref(ctx, base_uri, file_part, group_id),
    )
}

fn resolve_group_for_ref(
    ctx: &PopulateCtx<'_>,
    _base_uri: &str,
    file_part: &str,
    default_group: &str,
) -> Option<String> {
    if file_part.is_empty() {
        return Some(default_group.to_string());
    }
    let file_part = switchback_jsonschema::paths::strip_dot_slash(file_part);
    let candidate = std::path::Path::new(ctx.doc_uri)
        .parent()
        .unwrap_or(std::path::Path::new(""))
        .join(file_part);
    let normalized = candidate.to_string_lossy().replace('\\', "/");
    ctx.uri_to_group
        .get(&normalized)
        .cloned()
        .or_else(|| Some(default_group.to_string()))
}
