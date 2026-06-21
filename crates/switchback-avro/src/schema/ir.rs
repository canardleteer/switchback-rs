//! Avro schema intermediate representation.

use serde_json::Value;

/// Parsed Avro schema node.
#[derive(Clone, Debug, PartialEq)]
pub enum AvroSchema {
    /// Avro primitive type name or primitive-with-metadata object.
    Primitive(AvroPrimitive),
    /// Named type reference (FQCN-style string).
    NamedRef(String),
    /// Record definition.
    Record(AvroRecord),
    /// Enum definition.
    Enum(AvroEnum),
    /// Array type.
    Array(AvroArray),
    /// Map type.
    Map(AvroMap),
    /// Fixed type.
    Fixed(AvroFixed),
    /// Union of types.
    Union(AvroUnion),
}

/// Avro primitive type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvroPrimitive {
    Null,
    Boolean,
    Int,
    Long,
    Float,
    Double,
    Bytes,
    String,
}

/// Record field.
#[derive(Clone, Debug, PartialEq)]
pub struct AvroField {
    pub name: String,
    pub schema: AvroSchema,
    pub default: Option<Value>,
}

/// Avro record type.
#[derive(Clone, Debug, PartialEq)]
pub struct AvroRecord {
    pub name: String,
    pub namespace: Option<String>,
    pub fields: Vec<AvroField>,
}

/// Avro enum type.
#[derive(Clone, Debug, PartialEq)]
pub struct AvroEnum {
    pub name: String,
    pub namespace: Option<String>,
    pub symbols: Vec<String>,
}

/// Avro array type.
#[derive(Clone, Debug, PartialEq)]
pub struct AvroArray {
    pub items: Box<AvroSchema>,
}

/// Avro map type.
#[derive(Clone, Debug, PartialEq)]
pub struct AvroMap {
    pub values: Box<AvroSchema>,
}

/// Avro fixed type.
#[derive(Clone, Debug, PartialEq)]
pub struct AvroFixed {
    pub name: String,
    pub namespace: Option<String>,
    pub size: u32,
}

/// Avro union type.
#[derive(Clone, Debug, PartialEq)]
pub struct AvroUnion {
    pub variants: Vec<AvroSchema>,
}

impl AvroSchema {
    /// Parse an Avro schema JSON value into IR.
    pub fn from_value(value: &Value) -> Self {
        match value {
            Value::String(s) => primitive_or_named(s),
            Value::Array(items) => Self::Union(AvroUnion {
                variants: items.iter().map(Self::from_value).collect(),
            }),
            Value::Object(map) => object_schema(map),
            _ => Self::Primitive(AvroPrimitive::Null),
        }
    }

    /// True when the JSON value looks like an Avro schema document.
    pub fn is_schema_value(value: &Value) -> bool {
        match value {
            Value::String(s) => is_primitive_name(s) || is_named_ref(s),
            Value::Array(_) => true,
            Value::Object(map) => map.contains_key("type") || map.contains_key("name"),
            _ => false,
        }
    }
}

fn object_schema(map: &serde_json::Map<String, Value>) -> AvroSchema {
    if map.get("type").is_none() && map.contains_key("fields") {
        return AvroSchema::Record(parse_record(map));
    }

    let ty = map
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    match ty {
        "record" => AvroSchema::Record(parse_record(map)),
        "enum" => AvroSchema::Enum(AvroEnum {
            name: string_field(map, "name"),
            namespace: optional_string(map, "namespace"),
            symbols: map
                .get("symbols")
                .and_then(|v| v.as_array())
                .map(|syms| {
                    syms.iter()
                        .filter_map(|s| s.as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_default(),
        }),
        "array" => AvroSchema::Array(AvroArray {
            items: Box::new(
                map.get("items")
                    .map(AvroSchema::from_value)
                    .unwrap_or(AvroSchema::Primitive(AvroPrimitive::Null)),
            ),
        }),
        "map" => AvroSchema::Map(AvroMap {
            values: Box::new(
                map.get("values")
                    .map(AvroSchema::from_value)
                    .unwrap_or(AvroSchema::Primitive(AvroPrimitive::Null)),
            ),
        }),
        "fixed" => AvroSchema::Fixed(AvroFixed {
            name: string_field(map, "name"),
            namespace: optional_string(map, "namespace"),
            size: map.get("size").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
        }),
        other => {
            if let Some(p) = parse_primitive(other) {
                AvroSchema::Primitive(p)
            } else {
                AvroSchema::NamedRef(other.to_string())
            }
        }
    }
}

fn parse_record(map: &serde_json::Map<String, Value>) -> AvroRecord {
    AvroRecord {
        name: string_field(map, "name"),
        namespace: optional_string(map, "namespace"),
        fields: map
            .get("fields")
            .and_then(|v| v.as_array())
            .map(|fields| {
                fields
                    .iter()
                    .filter_map(|field| {
                        let obj = field.as_object()?;
                        let name = obj.get("name").and_then(|v| v.as_str())?;
                        let schema = obj
                            .get("type")
                            .map(AvroSchema::from_value)
                            .unwrap_or(AvroSchema::Primitive(AvroPrimitive::Null));
                        Some(AvroField {
                            name: name.to_string(),
                            schema,
                            default: obj.get("default").cloned(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default(),
    }
}

fn primitive_or_named(s: &str) -> AvroSchema {
    if let Some(p) = parse_primitive(s) {
        AvroSchema::Primitive(p)
    } else {
        AvroSchema::NamedRef(s.to_string())
    }
}

fn parse_primitive(name: &str) -> Option<AvroPrimitive> {
    match name {
        "null" => Some(AvroPrimitive::Null),
        "boolean" => Some(AvroPrimitive::Boolean),
        "int" => Some(AvroPrimitive::Int),
        "long" => Some(AvroPrimitive::Long),
        "float" => Some(AvroPrimitive::Float),
        "double" => Some(AvroPrimitive::Double),
        "bytes" => Some(AvroPrimitive::Bytes),
        "string" => Some(AvroPrimitive::String),
        _ => None,
    }
}

fn is_primitive_name(s: &str) -> bool {
    parse_primitive(s).is_some()
}

fn is_named_ref(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
}

fn string_field(map: &serde_json::Map<String, Value>, key: &str) -> String {
    map.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string()
}

fn optional_string(map: &serde_json::Map<String, Value>, key: &str) -> Option<String> {
    map.get(key)
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .filter(|s| !s.is_empty())
}
