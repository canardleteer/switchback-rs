//! Shared API-description envelope types (OpenAPI / AsyncAPI / OpenRPC).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// OpenAPI / AsyncAPI / OpenRPC metadata block.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Info {
    pub title: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "termsOfService")]
    pub terms_of_service: Option<String>,
    pub contact: Option<Value>,
    pub license: Option<Value>,
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocs>,
}

/// Server entry shared across API description formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Server {
    pub url: Option<String>,
    pub description: Option<String>,
    pub variables: Option<BTreeMap<String, Value>>,
}

/// Reusable components container (partial; extended by family parsers).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Components {
    pub schemas: Option<BTreeMap<String, Value>>,
    pub parameters: Option<BTreeMap<String, Value>>,
    pub responses: Option<BTreeMap<String, Value>>,
    #[serde(rename = "requestBodies")]
    pub request_bodies: Option<BTreeMap<String, Value>>,
    pub headers: Option<BTreeMap<String, Value>>,
    #[serde(rename = "securitySchemes")]
    pub security_schemes: Option<BTreeMap<String, Value>>,
}

/// Tag object shared across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Tag {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocs>,
}

/// External documentation link.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExternalDocs {
    pub url: Option<String>,
    pub description: Option<String>,
}

/// JSON Schema reference object (`{"$ref": "..."}`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    #[serde(rename = "$ref")]
    pub ref_key: String,
}

/// Minimal envelope view extracted from a loaded document root.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Envelope {
    pub info: Info,
    pub servers: Vec<Server>,
    pub components: Components,
    pub tags: Vec<Tag>,
    pub external_docs: Option<ExternalDocs>,
}

impl Envelope {
    /// Extract shared envelope fields when present at the document root.
    pub fn from_value(value: &Value) -> Self {
        let obj = match value.as_object() {
            Some(o) => o,
            None => return Self::default(),
        };

        let info = obj
            .get("info")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        let servers = obj
            .get("servers")
            .and_then(parse_servers)
            .unwrap_or_default();

        let components = obj
            .get("components")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        let tags = obj
            .get("tags")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        let external_docs = obj
            .get("externalDocs")
            .and_then(|v| serde_json::from_value(v.clone()).ok());

        Self {
            info,
            servers,
            components,
            tags,
            external_docs,
        }
    }

    pub fn title_or_fallback<'a>(&'a self, fallback: &'a str) -> &'a str {
        self.info
            .title
            .as_deref()
            .filter(|s| !s.is_empty())
            .unwrap_or(fallback)
    }
}

fn parse_servers(value: &Value) -> Option<Vec<Server>> {
    match value {
        Value::Array(items) => items
            .iter()
            .map(|v| serde_json::from_value(v.clone()))
            .collect::<Result<Vec<_>, _>>()
            .ok(),
        Value::Object(map) => Some(
            map.iter()
                .map(|(name, v)| {
                    let mut server: Server = serde_json::from_value(v.clone()).unwrap_or_default();
                    if server.url.is_none() {
                        server.url = Some(name.clone());
                    }
                    server
                })
                .collect(),
        ),
        _ => None,
    }
}
