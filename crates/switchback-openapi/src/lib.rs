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
pub mod companion;
pub mod contract;
pub mod examples;
pub mod family;
pub mod link;
pub mod load;
pub mod manual;
pub mod meta_schemas;
pub mod paths;
pub mod populate;

pub use category::OpenApiCategory;
pub use contract::OpenApiContract;
pub use examples::{
    default_example_fixtures, example_fixture, fixture_path, fixtures_dir, fixtures_for_tier,
    load_acme_example, load_example, load_fixture_relative, ExampleFixture, ExampleTier,
    EXAMPLE_ACME_INPUTS, EXAMPLE_FIXTURES, MICRO_ACME_ROOT, MICRO_COMPANION, MICRO_MULTIFILE,
    MICRO_NULLABLE_3_0, MICRO_STREAMING, MICRO_TAG_GROUPS, UPSTREAM_HIGH_3_0,
    UPSTREAM_HIGH_3_1_WEBHOOK, UPSTREAM_LOW_3_0, UPSTREAM_LOW_3_1,
};
pub use family::OpenApiFamily;
pub use link::OpenApiLinkExtractor;
pub use load::{load, LoadArgs};
pub use manual::restore_sources;
pub use populate::parse_openapi_version;

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
