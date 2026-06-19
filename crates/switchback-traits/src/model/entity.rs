//! Entity bodies and stored entities.

use crate::model::link::{IntraLink, Reference};
use crate::model::manual::Source;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredEntity {
    pub name: String,
    pub category: String,
    pub title: String,
    pub doc: Option<String>,
    pub source: Option<Source>,
    pub refs: Vec<Reference>,
    pub intra_links: Vec<IntraLink>,
    pub body: EntityBody,
}

/// Discriminated entity payload mirroring the proto `oneof body`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntityBody {
    Operation(OperationBody),
    Schema(SchemaBody),
    Channel(ChannelBody),
    Message(MessageBody),
    Parameter(ParameterBody),
    Response(ResponseBody),
    RequestBody(RequestBodyBody),
    SecurityScheme(SecuritySchemeBody),
    Service(ServiceBody),
    Extension(ExtensionBody),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OperationBody {
    pub signature: String,
    pub fence_language: String,
    pub fence_body: String,
    pub parameters: Vec<ParameterRef>,
    pub responses: Vec<ResponseRef>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SchemaBody {
    pub fence_language: String,
    pub fence_body: String,
    pub payload_format: String,
    pub properties: Vec<Property>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChannelBody {
    pub signature: String,
    pub fence_language: String,
    pub fence_body: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MessageBody {
    pub fence_language: String,
    pub fence_body: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ParameterBody {
    pub name: String,
    pub location: String,
    pub required: bool,
    pub fence_language: String,
    pub fence_body: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ResponseBody {
    pub status: String,
    pub media_type: String,
    pub fence_language: String,
    pub fence_body: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RequestBodyBody {
    pub required: bool,
    pub fence_language: String,
    pub fence_body: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SecuritySchemeBody {
    pub scheme_type: String,
    pub fence_language: String,
    pub fence_body: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ServiceBody {
    pub signature: String,
    pub fence_language: String,
    pub fence_body: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtensionBody {
    pub extension_type: String,
    pub payload: Vec<u8>,
    pub fence_language: Option<String>,
    pub fence_body: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Property {
    pub name: String,
    pub schema_ref: Reference,
    pub required: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParameterRef {
    pub name: String,
    pub location: String,
    pub required: bool,
    pub schema_ref: Reference,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResponseRef {
    pub status: String,
    pub schema_ref: Reference,
    pub media_type: String,
}
