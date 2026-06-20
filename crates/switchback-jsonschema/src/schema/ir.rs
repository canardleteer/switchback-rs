//! JSON Schema IR for the shared document envelope and schema bodies.

use std::collections::BTreeMap;

use serde_json::Value;

/// Parsed schema node.
#[derive(Clone, Debug, PartialEq)]
pub enum Schema {
    /// `$ref` to another entity (target resolved at populate time).
    Ref { ref_key: String },
    /// Boolean schema (`true` / `false`).
    Bool(bool),
    /// Object-typed schema with typed fields and preserved extras.
    Object(Box<SchemaObject>),
}

/// Object-shaped JSON Schema with cross-draft fields.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SchemaObject {
    pub ty: Vec<JsonType>,
    pub format: Option<String>,
    pub properties: Vec<(String, Schema)>,
    pub required: Vec<String>,
    pub items: Option<Box<Schema>>,
    pub composites: Vec<Composite>,
    pub enum_values: Vec<Value>,
    pub default: Option<Value>,
    pub example: Option<Value>,
    pub nullable: Option<bool>,
    pub discriminator: Option<Value>,
    pub content_media_type: Option<String>,
    pub content_encoding: Option<String>,
    pub description: Option<String>,
    pub extras: BTreeMap<String, Value>,
}

/// JSON Schema `type` keyword values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum JsonType {
    Null,
    Boolean,
    Object,
    Array,
    Number,
    Integer,
    String,
}

/// Composite combinator (`allOf` / `oneOf` / `anyOf`).
#[derive(Clone, Debug, PartialEq)]
pub enum Composite {
    AllOf(Vec<Schema>),
    OneOf(Vec<Schema>),
    AnyOf(Vec<Schema>),
}

impl Schema {
    pub fn from_value(value: &Value) -> Self {
        match value {
            Value::Bool(b) => Self::Bool(*b),
            Value::Object(map) => {
                if let Some(Value::String(r)) = map.get("$ref") {
                    return Self::Ref { ref_key: r.clone() };
                }
                Self::Object(Box::new(SchemaObject::from_map(map)))
            }
            _ => Self::Object(Box::default()),
        }
    }

    pub fn is_schema_value(value: &Value) -> bool {
        match value {
            Value::Bool(_) => true,
            Value::Object(map) => {
                map.contains_key("$ref")
                    || map.contains_key("type")
                    || map.contains_key("properties")
                    || map.contains_key("allOf")
                    || map.contains_key("oneOf")
                    || map.contains_key("anyOf")
                    || map.contains_key("enum")
                    || map.contains_key("$defs")
                    || map.contains_key("definitions")
            }
            _ => false,
        }
    }
}

impl SchemaObject {
    fn from_map(map: &serde_json::Map<String, Value>) -> Self {
        let mut obj = Self {
            description: map
                .get("description")
                .and_then(|v| v.as_str())
                .map(str::to_string),
            format: map
                .get("format")
                .and_then(|v| v.as_str())
                .map(str::to_string),
            required: map
                .get("required")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_default(),
            enum_values: map
                .get("enum")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default(),
            default: map.get("default").cloned(),
            example: map.get("example").cloned(),
            nullable: map.get("nullable").and_then(|v| v.as_bool()),
            discriminator: map.get("discriminator").cloned(),
            content_media_type: map
                .get("contentMediaType")
                .and_then(|v| v.as_str())
                .map(str::to_string),
            content_encoding: map
                .get("contentEncoding")
                .and_then(|v| v.as_str())
                .map(str::to_string),
            ..Default::default()
        };

        obj.ty = parse_type(map.get("type"));
        if let Some(props) = map.get("properties").and_then(|v| v.as_object()) {
            for (name, schema_val) in props {
                obj.properties
                    .push((name.clone(), Schema::from_value(schema_val)));
            }
        }
        if let Some(items) = map.get("items") {
            obj.items = Some(Box::new(Schema::from_value(items)));
        }
        for (keyword, make) in [
            ("allOf", Composite::AllOf as fn(Vec<Schema>) -> Composite),
            ("oneOf", Composite::OneOf as fn(Vec<Schema>) -> Composite),
            ("anyOf", Composite::AnyOf as fn(Vec<Schema>) -> Composite),
        ] {
            if let Some(arr) = map.get(keyword).and_then(|v| v.as_array()) {
                let schemas = arr.iter().map(Schema::from_value).collect();
                obj.composites.push(make(schemas));
            }
        }

        let reserved = reserved_keys();
        for (key, val) in map {
            if !reserved.iter().any(|k| k == &key.as_str()) {
                obj.extras.insert(key.clone(), val.clone());
            }
        }
        obj
    }
}

fn parse_type(value: Option<&Value>) -> Vec<JsonType> {
    let Some(value) = value else {
        return Vec::new();
    };
    match value {
        Value::String(s) => parse_type_str(s).into_iter().collect(),
        Value::Array(items) => items
            .iter()
            .filter_map(|v| v.as_str().and_then(parse_type_str))
            .collect(),
        _ => Vec::new(),
    }
}

fn parse_type_str(s: &str) -> Option<JsonType> {
    Some(match s {
        "null" => JsonType::Null,
        "boolean" => JsonType::Boolean,
        "object" => JsonType::Object,
        "array" => JsonType::Array,
        "number" => JsonType::Number,
        "integer" => JsonType::Integer,
        "string" => JsonType::String,
        _ => return None,
    })
}

fn reserved_keys() -> &'static [&'static str] {
    &[
        "$ref",
        "$schema",
        "$id",
        "$defs",
        "definitions",
        "type",
        "format",
        "properties",
        "required",
        "items",
        "allOf",
        "oneOf",
        "anyOf",
        "enum",
        "default",
        "example",
        "nullable",
        "discriminator",
        "contentMediaType",
        "contentEncoding",
        "description",
        "title",
    ]
}
