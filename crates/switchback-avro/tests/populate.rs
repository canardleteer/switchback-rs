use serde_json::json;
use switchback_avro::{AvroSchema, populate_avro_schema_body};

#[test]
fn avro_record_ir_and_populate() {
    let value = json!({
        "type": "record",
        "name": "User",
        "namespace": "acme.events",
        "fields": [
            { "name": "id", "type": "long" },
            { "name": "profile", "type": "acme.events.Profile" }
        ]
    });
    let schema = AvroSchema::from_value(&value);
    assert!(matches!(schema, AvroSchema::Record(_)));

    let body = populate_avro_schema_body(
        &value,
        "acme.events",
        "components",
        Some("application/vnd.apache.avro+json"),
    );
    assert_eq!(body.payload_format, "application/vnd.apache.avro+json");
    assert_eq!(body.fence_language, "json");
    assert_eq!(body.properties.len(), 1);
    assert_eq!(body.properties[0].name, "profile");
    assert_eq!(
        body.properties[0].schema_ref.target.name,
        "acme.events.Profile"
    );
}

#[test]
fn avro_inline_enum_property_links_named_type() {
    let value = json!({
        "type": "record",
        "name": "PipelineStepCompleted",
        "fields": [
            { "name": "status", "type": {
                "type": "enum",
                "name": "PipelineStatus",
                "symbols": ["queued", "running"]
            }}
        ]
    });
    let body = populate_avro_schema_body(
        &value,
        "acme.events",
        "acme.example.v3alpha1",
        Some("application/vnd.apache.avro+json"),
    );
    assert_eq!(body.properties.len(), 1);
    assert_eq!(body.properties[0].name, "status");
    assert_eq!(body.properties[0].schema_ref.target.name, "PipelineStatus");
}

#[test]
fn collect_named_avro_schemas_nested_enum() {
    let value = json!({
        "type": "record",
        "name": "PipelineStepCompleted",
        "fields": [
            { "name": "status", "type": {
                "type": "enum",
                "name": "PipelineStatus",
                "symbols": ["queued"]
            }}
        ]
    });
    let named = switchback_avro::collect_named_avro_schemas(&value);
    let names: Vec<_> = named.iter().map(|(n, _)| n.as_str()).collect();
    assert!(names.contains(&"PipelineStepCompleted"));
    assert!(names.contains(&"PipelineStatus"));
}
