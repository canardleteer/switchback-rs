//! AsyncAPI entity categories.

use switchback_traits::{EntityCategory, GenericCategory};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum AsyncApiCategory {
    Channel,
    Operation,
    Message,
    Schema,
    Parameter,
    SecurityScheme,
}

impl EntityCategory for AsyncApiCategory {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Channel => "channel",
            Self::Operation => "operation",
            Self::Message => "message",
            Self::Schema => "schema",
            Self::Parameter => "parameter",
            Self::SecurityScheme => "security-scheme",
        }
    }

    fn dir(&self) -> &str {
        match self {
            Self::Channel => "channels",
            Self::Operation => "operations",
            Self::Message => "messages",
            Self::Schema => "schemas",
            Self::Parameter => "parameters",
            Self::SecurityScheme => "security-schemes",
        }
    }

    fn summary_prefix(&self) -> &str {
        match self {
            Self::Channel => "Channel",
            Self::Operation => "Operation",
            Self::Message => "Message",
            Self::Schema => "Schema",
            Self::Parameter => "Parameter",
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
