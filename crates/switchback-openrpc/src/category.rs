//! OpenRPC entity categories.

use switchback_traits::{EntityCategory, GenericCategory};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum OpenRpcCategory {
    Operation,
    Schema,
    Parameter,
}

impl EntityCategory for OpenRpcCategory {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Operation => "operation",
            Self::Schema => "schema",
            Self::Parameter => "parameter",
        }
    }

    fn dir(&self) -> &str {
        match self {
            Self::Operation => "operations",
            Self::Schema => "schemas",
            Self::Parameter => "parameters",
        }
    }

    fn summary_prefix(&self) -> &str {
        match self {
            Self::Operation => "Operation",
            Self::Schema => "Schema",
            Self::Parameter => "Parameter",
        }
    }

    fn to_generic(&self) -> Option<GenericCategory> {
        match self {
            Self::Schema => Some(GenericCategory::Schema),
            Self::Operation => Some(GenericCategory::Operation),
            Self::Parameter => Some(GenericCategory::Generic),
        }
    }
}
