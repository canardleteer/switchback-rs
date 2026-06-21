#![forbid(unsafe_code)]
#![allow(missing_docs)]

//! Avro schema document parser layer for switchback family parsers.
//!
//! See the crate README for the public API surface.

pub mod meta_schemas;
pub mod schema;

pub use schema::{
    AvroArray, AvroEnum, AvroFixed, AvroMap, AvroPrimitive, AvroRecord, AvroSchema, AvroUnion,
    collect_named_avro_schemas, populate_avro_schema_body,
};
