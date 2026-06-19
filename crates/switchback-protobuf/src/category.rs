//! Protobuf entity categories.

use switchback_traits::{EntityCategory, GenericCategory};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ProtobufCategory {
    Schema,
    Service,
    Operation,
}

impl EntityCategory for ProtobufCategory {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Schema => "schema",
            Self::Service => "service",
            Self::Operation => "operation",
        }
    }

    fn dir(&self) -> &str {
        match self {
            Self::Schema => "schemas",
            Self::Service => "services",
            Self::Operation => "operations",
        }
    }

    fn summary_prefix(&self) -> &str {
        match self {
            Self::Schema => "Schema",
            Self::Service => "Service",
            Self::Operation => "Operation",
        }
    }

    fn to_generic(&self) -> Option<GenericCategory> {
        match self {
            Self::Schema => Some(GenericCategory::Schema),
            Self::Service => Some(GenericCategory::Service),
            Self::Operation => Some(GenericCategory::Operation),
        }
    }
}
