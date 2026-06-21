//! Avro schema IR and populate helpers.

mod body;
mod ir;
mod named;

pub use body::populate_avro_schema_body;
pub use ir::{
    AvroArray, AvroEnum, AvroField, AvroFixed, AvroMap, AvroPrimitive, AvroRecord, AvroSchema,
    AvroUnion,
};
pub use named::collect_named_avro_schemas;
