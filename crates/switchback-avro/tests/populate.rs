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
fn avro_union_ir() {
    let value = json!(["null", "string"]);
    let schema = AvroSchema::from_value(&value);
    assert!(matches!(schema, AvroSchema::Union(_)));
}
