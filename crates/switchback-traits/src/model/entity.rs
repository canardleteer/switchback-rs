//! Entity bodies and stored entities.

use crate::model::link::{IntraLink, Reference};
use crate::model::manual::Source;
use crate::response_severity::ResponseSeverity;

/// One entity as stored in a serialized switchback [`Group`](super::contract::Group).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredEntity {
    /// Entity name within its group and category.
    pub name: String,
    /// Family-specific category slug (mirrors [`EntityCategory::as_str`](crate::traits::EntityCategory::as_str)).
    pub category: String,
    /// Human-readable entity title for page headings.
    pub title: String,
    /// Leading documentation prose, when present.
    pub doc: Option<String>,
    /// Provenance pointer into the switchback source layer, when available.
    pub source: Option<Source>,
    /// Structural cross-references in the entity body (schema `$ref`, FQN, etc.).
    pub refs: Vec<Reference>,
    /// Prose-level intra-links extracted from `doc` and fence bodies.
    pub intra_links: Vec<IntraLink>,
    /// Category-specific payload (mirrors the proto `oneof body`).
    pub body: EntityBody,
}

/// Discriminated entity payload mirroring the proto `oneof body`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntityBody {
    /// RPC or HTTP operation entity.
    Operation(OperationBody),
    /// Schema or message type entity.
    Schema(SchemaBody),
    /// Async messaging channel entity.
    Channel(ChannelBody),
    /// Message payload entity (distinct from schema in some families).
    Message(MessageBody),
    /// Standalone parameter entity.
    Parameter(ParameterBody),
    /// Standalone response entity.
    Response(ResponseBody),
    /// Request body entity.
    RequestBody(RequestBodyBody),
    /// Security scheme entity.
    SecurityScheme(SecuritySchemeBody),
    /// Service definition entity.
    Service(ServiceBody),
    /// Opaque family-specific extension payload.
    Extension(ExtensionBody),
}

/// Operation entity body (RPC signature, parameters, responses).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OperationBody {
    /// Human-readable operation signature line.
    pub signature: String,
    /// Language tag for the fenced code block (e.g. `"protobuf"`, `"yaml"`).
    pub fence_language: String,
    /// Fenced source excerpt for the operation definition.
    pub fence_body: String,
    /// Parameter references attached to the operation.
    pub parameters: Vec<ParameterRef>,
    /// Response references attached to the operation.
    pub responses: Vec<ResponseRef>,
    /// Request body attachment when the operation defines one.
    pub request_body: Option<OperationRequestBodyRef>,
}

/// Request body attachment on an [`OperationBody`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperationRequestBodyRef {
    /// Whether the request body is required.
    pub required: bool,
    /// Primary content media type (e.g. `"application/json"`).
    pub media_type: String,
    /// Structural reference to the request body schema entity.
    pub schema_ref: Reference,
    /// Human-readable schema type label for rendering.
    pub type_label: String,
}

/// Schema entity body (type definition with optional properties).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SchemaBody {
    /// Language tag for the fenced code block.
    pub fence_language: String,
    /// Fenced source excerpt for the schema definition.
    pub fence_body: String,
    /// Serialization format hint (e.g. `"application/json"`).
    pub payload_format: String,
    /// Object properties when the schema represents a structured type.
    pub properties: Vec<Property>,
}

/// Channel entity body (async messaging endpoint).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChannelBody {
    /// Human-readable channel signature line.
    pub signature: String,
    /// Language tag for the fenced code block.
    pub fence_language: String,
    /// Fenced source excerpt for the channel definition.
    pub fence_body: String,
}

/// Message entity body (payload type excerpt).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MessageBody {
    /// Language tag for the fenced code block.
    pub fence_language: String,
    /// Fenced source excerpt for the message definition.
    pub fence_body: String,
}

/// Parameter entity body (location, requirement, schema excerpt).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ParameterBody {
    /// Parameter name.
    pub name: String,
    /// Parameter location (e.g. `"query"`, `"path"`, `"header"`).
    pub location: String,
    /// Whether the parameter is required.
    pub required: bool,
    /// Language tag for the fenced code block.
    pub fence_language: String,
    /// Fenced source excerpt for the parameter schema.
    pub fence_body: String,
}

/// Response entity body (status, media type, schema excerpt).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ResponseBody {
    /// HTTP or RPC status label.
    pub status: String,
    /// Outcome severity class derived from the status at populate time.
    pub severity: ResponseSeverity,
    /// Response media type when applicable.
    pub media_type: String,
    /// Language tag for the fenced code block.
    pub fence_language: String,
    /// Fenced source excerpt for the response schema.
    pub fence_body: String,
}

/// Request body entity (requirement flag and schema excerpt).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RequestBodyBody {
    /// Whether the request body is required.
    pub required: bool,
    /// Language tag for the fenced code block.
    pub fence_language: String,
    /// Fenced source excerpt for the request body schema.
    pub fence_body: String,
}

/// Security scheme entity body.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SecuritySchemeBody {
    /// Scheme type label (e.g. `"apiKey"`, `"oauth2"`).
    pub scheme_type: String,
    /// Language tag for the fenced code block.
    pub fence_language: String,
    /// Fenced source excerpt for the security scheme definition.
    pub fence_body: String,
}

/// Service definition entity body.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ServiceBody {
    /// Human-readable service signature line.
    pub signature: String,
    /// Language tag for the fenced code block.
    pub fence_language: String,
    /// Fenced source excerpt for the service definition.
    pub fence_body: String,
}

/// Opaque family-specific extension entity body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtensionBody {
    /// Family-defined extension type identifier.
    pub extension_type: String,
    /// Opaque extension payload bytes.
    pub payload: Vec<u8>,
    /// Optional language tag when a fenced excerpt is also provided.
    pub fence_language: Option<String>,
    /// Optional fenced source excerpt for human-readable rendering.
    pub fence_body: Option<String>,
}

/// One property on a schema-like entity, pointing at another entity via [`Reference`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Property {
    /// Property field name.
    pub name: String,
    /// Structural reference to the property's schema entity.
    pub schema_ref: Reference,
    /// Whether the property is required on the parent type.
    pub required: bool,
}

/// Parameter attachment on an [`OperationBody`], referencing a schema entity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParameterRef {
    /// Parameter name.
    pub name: String,
    /// Parameter location (e.g. `"query"`, `"path"`).
    pub location: String,
    /// Whether the parameter is required.
    pub required: bool,
    /// Structural reference to the parameter schema entity.
    pub schema_ref: Reference,
    /// Human-readable schema type label for rendering (e.g. `"string"`, `"coordinate"`).
    pub type_label: String,
    /// OpenAPI parameter description prose, when present.
    pub description: String,
}

/// Response attachment on an [`OperationBody`], referencing a schema entity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResponseRef {
    /// HTTP or RPC status label.
    pub status: String,
    /// Outcome severity class derived from the status at populate time.
    pub severity: ResponseSeverity,
    /// Structural reference to the response schema entity.
    pub schema_ref: Reference,
    /// Response media type when applicable.
    pub media_type: String,
    /// OpenAPI response description prose, when present.
    pub description: String,
}
