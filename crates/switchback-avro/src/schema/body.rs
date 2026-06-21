//! Produce [`SchemaBody`](switchback_traits::SchemaBody) from Avro schema IR.

use serde_json::Value;
use switchback_traits::{Property, RefKind, Reference, SchemaBody};

use super::ir::{AvroField, AvroSchema};

const DEFAULT_AVRO_JSON_FORMAT: &str = "application/vnd.apache.avro+json";

/// Build a switchback schema entity body from an Avro schema JSON value.
pub fn populate_avro_schema_body(
    value: &Value,
    module_id: &str,
    group_id: &str,
    schema_format: Option<&str>,
) -> SchemaBody {
    let schema = AvroSchema::from_value(value);
    let fence_body = serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".into());
    let properties = avro_properties(&schema, module_id, group_id);
    SchemaBody {
        fence_language: "json".to_string(),
        fence_body,
        payload_format: schema_format
            .filter(|s| !s.is_empty())
            .unwrap_or(DEFAULT_AVRO_JSON_FORMAT)
            .to_string(),
        properties,
    }
}

fn avro_properties(
    schema: &AvroSchema,
    module_id: &str,
    group_id: &str,
) -> Vec<Property> {
    match schema {
        AvroSchema::Record(record) => record
            .fields
            .iter()
            .filter_map(|field| field_property(field, module_id, group_id))
            .collect(),
        _ => Vec::new(),
    }
}

fn field_property(field: &AvroField, module_id: &str, group_id: &str) -> Option<Property> {
    let AvroSchema::NamedRef(target_name) = &field.schema else {
        return None;
    };
    Some(Property {
        name: field.name.clone(),
        schema_ref: Reference {
            target: switchback_traits::EntityRef {
                module: module_id.to_string(),
                group: group_id.to_string(),
                category: "schema".to_string(),
                name: target_name.clone(),
            },
            kind: RefKind::Internal,
        },
        required: field.default.is_none(),
    })
}
