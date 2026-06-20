//! OpenAPI entity categories.

use switchback_traits::{EntityCategory, GenericCategory};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum OpenApiCategory {
    Schema,
    Operation,
    Parameter,
    Response,
    RequestBody,
    SecurityScheme,
}

impl EntityCategory for OpenApiCategory {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Schema => "schema",
            Self::Operation => "operation",
            Self::Parameter => "parameter",
            Self::Response => "response",
            Self::RequestBody => "request-body",
            Self::SecurityScheme => "security-scheme",
        }
    }

    fn dir(&self) -> &str {
        match self {
            Self::Schema => "schemas",
            Self::Operation => "operations",
            Self::Parameter => "parameters",
            Self::Response => "responses",
            Self::RequestBody => "request-bodies",
            Self::SecurityScheme => "security-schemes",
        }
    }

    fn summary_prefix(&self) -> &str {
        match self {
            Self::Schema => "Schema",
            Self::Operation => "Operation",
            Self::Parameter => "Parameter",
            Self::Response => "Response",
            Self::RequestBody => "Request body",
            Self::SecurityScheme => "Security scheme",
        }
    }

    fn to_generic(&self) -> Option<GenericCategory> {
        match self {
            Self::Schema => Some(GenericCategory::Schema),
            Self::Operation => Some(GenericCategory::Operation),
            _ => Some(GenericCategory::Generic),
        }
    }
}
