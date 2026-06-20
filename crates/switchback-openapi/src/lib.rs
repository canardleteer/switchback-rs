#![forbid(unsafe_code)]

//! OpenAPI -> switchback parser.
//!
//! `switchback-openapi` implements `ContractFamily` and `Contract` for OpenAPI
//! and turns an OpenAPI Description (3.0.x and 3.1.x) into a switchback. It
//! preserves the contract's version faithfully: 3.0 `nullable` stays
//! `nullable`; 3.1 `type: [T, "null"]` stays a type array. The parser does not
//! normalize or upgrade versions.
//!
//! Entity categories: `schema`, `operation`, `parameter`, `response`,
//! `request-body`, `security-scheme`. Grouping uses `tags` / `x-tagGroups`.
//! It reuses the loader, `$ref` resolver, envelope, and schema renderer from
//! `switchback-jsonschema`.

pub mod category;
pub mod family;
pub mod link;
pub mod meta_schemas;

pub use category::OpenApiCategory;
pub use family::OpenApiFamily;
pub use link::OpenApiLinkExtractor;

#[cfg(test)]
mod tests {
    use switchback_traits::{ContractFamily, SpecVersion};

    use crate::OpenApiFamily;

    #[test]
    fn meta_schema_returns_bytes_for_latest_version() {
        let family = OpenApiFamily;
        let bytes = family
            .meta_schema(&SpecVersion::from("3.1.0"))
            .expect("3.1 meta-schema");
        assert!(!bytes.is_empty());
        assert!(bytes.starts_with(b"{"));
    }
}
