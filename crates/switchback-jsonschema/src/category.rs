//! JSON Schema catalog entity categories.

use switchback_traits::{EntityCategory, GenericCategory};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum JsonSchemaCategory {
    Schema,
}

impl EntityCategory for JsonSchemaCategory {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Schema => "schema",
        }
    }

    fn dir(&self) -> &str {
        match self {
            Self::Schema => "schemas",
        }
    }

    fn summary_prefix(&self) -> &str {
        match self {
            Self::Schema => "Schema",
        }
    }

    fn to_generic(&self) -> Option<GenericCategory> {
        match self {
            Self::Schema => Some(GenericCategory::Schema),
        }
    }
}
