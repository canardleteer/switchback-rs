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
//! sub-grouping. It reuses the loader, `$ref` resolver, envelope, and schema
//! renderer from `switchback-jsonschema`. Mermaid sequence diagrams are
//! generated from operations and replies (an AsyncAPI-specific feature with no
//! OpenAPI analog).

pub mod category;
pub mod family;
pub mod link;
pub mod meta_schemas;

pub use category::AsyncApiCategory;
pub use family::AsyncApiFamily;
pub use link::AsyncApiLinkExtractor;

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
}
