#![forbid(unsafe_code)]

//! OpenRPC -> switchback parser.
//!
//! `switchback-openrpc` implements `ContractFamily` and `Contract` for OpenRPC
//! and turns an OpenRPC document (1.3.x and 1.4.x) into a switchback. An OpenRPC
//! document has `info`, `servers`, `methods`, and `components`; a method has
//! `params` (`ContentDescriptor[]`) and `result` (a `ContentDescriptor`). The
//! parser maps `methods` -> `Operation` entities and content descriptors /
//! component schemas -> `Schema` entities, reusing the loader, `$ref` resolver,
//! envelope, and schema renderer from `switchback-jsonschema`.
//!
//! Entity categories: `operation`, `schema`, `parameter`. Grouping uses
//! `x-tagGroup`; the module is the service the OpenRPC document describes.

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

pub use category::OpenRpcCategory;
pub use contract::OpenRpcContract;
pub use examples::{
    EXAMPLE_ACME_INPUTS, EXAMPLE_FIXTURES, ExampleFixture, ExampleTier, MICRO_ACME_ROOT,
    MICRO_COMPANION, MICRO_MULTIFILE, MICRO_TAG_GROUPS, UPSTREAM_LINK_1_4, UPSTREAM_METRICS_1_3,
    UPSTREAM_PETSTORE_1_4, default_example_fixtures, example_fixture, fixture_path, fixtures_dir,
    fixtures_for_tier, load_acme_example, load_example, load_fixture_relative,
};
pub use family::OpenRpcFamily;
pub use link::OpenRpcLinkExtractor;
pub use load::{LoadArgs, load};
pub use manual::restore_sources;
pub use populate::parse_openrpc_version;

#[cfg(test)]
mod tests {
    use switchback_traits::{ContractFamily, SpecVersion};

    use crate::OpenRpcFamily;

    #[test]
    fn meta_schema_returns_bytes_for_latest_version() {
        let family = OpenRpcFamily;
        let bytes = family
            .meta_schema(&SpecVersion::from("1.4"))
            .expect("1.4 meta-schema");
        assert!(!bytes.is_empty());
        assert!(bytes.starts_with(b"{"));
    }
}
