//! JSON Schema intermediate representation and seam body production.

mod body;
mod ir;

pub use body::populate_schema_body;
pub use ir::{Composite, JsonType, Schema, SchemaObject};
