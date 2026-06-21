//! Avro schema IR and populate helpers.

mod body;
mod ir;

pub use body::populate_avro_schema_body;
pub use ir::{
    AvroArray, AvroEnum, AvroField, AvroFixed, AvroMap, AvroPrimitive, AvroRecord, AvroSchema,
    AvroUnion,
};
