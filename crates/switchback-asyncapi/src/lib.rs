#![forbid(unsafe_code)]

//! AsyncAPI -> switchback parser.
//!
//! `switchback-asyncapi` implements `ContractFamily` and `Contract` for
//! AsyncAPI and turns an AsyncAPI document (2.x and 3.x) into a switchback. It
//! preserves the contract's version faithfully: 2.x channel-embedded
//! `subscribe`/`publish` operations stay embedded; 3.x top-level `operations`
//! stay top-level. The parser does not synthesize 3.x from 2.x.
//!
//! Entity categories: `channel`, `operation`, `message`, `schema`. Grouping
//! uses the application `id` (or `info.title` slug) with `tags` for
//! sub-grouping. It reuses the loader, `$ref` resolver, and envelope from
//! `switchback-jsonschema`, JSON Schema payloads from `switchback-jsonschema`,
//! and Avro payloads from `switchback-avro`. Mermaid sequence diagrams are
//! generated from operations and replies (an AsyncAPI-specific feature with no
//! OpenAPI analog).

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

pub use category::AsyncApiCategory;
pub use contract::AsyncApiContract;
pub use examples::fixtures_dir;
pub use family::AsyncApiFamily;
pub use link::AsyncApiLinkExtractor;
pub use load::{LoadArgs, load};
pub use manual::restore_sources;
pub use populate::parse_asyncapi_version;

#[cfg(test)]
mod tests {
    use switchback_traits::{ContractFamily, SpecVersion};

    use crate::AsyncApiFamily;

    #[test]
    fn meta_schema_returns_bytes_for_latest_version() {
        let family = AsyncApiFamily;
        let bytes = family
            .meta_schema(&SpecVersion::from("3.0.0"))
            .expect("3.0 meta-schema");
        assert!(!bytes.is_empty());
        assert!(bytes.starts_with(b"{"));
    }

    #[test]
    fn minimal_asyncapi_loads() {
        let module_root = crate::fixtures_dir().join("micro/minimal");
        let manual = crate::load(&crate::LoadArgs {
            module_root: module_root.clone(),
            inputs: vec![module_root.join("asyncapi.yaml")],
            search_roots: vec![module_root],
            title: None,
        })
        .expect("load minimal fixture");
        assert_eq!(manual.modules[0].contracts[0].family, "asyncapi");
        assert_eq!(
            manual.modules[0].contracts[0].version.as_str(),
            "2.6.0"
        );
        assert!(!manual.modules[0].contracts[0].groups.is_empty());
    }
}
