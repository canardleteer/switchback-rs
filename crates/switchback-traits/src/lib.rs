#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![allow(async_fn_in_trait)] // ADR 0002: native async fn in trait at the seam

//! The seam of the switchback-rs toolchain.
//!
//! `switchback-traits` owns the trait spine and the in-memory model that every
//! parser and every renderer depends on. See the workspace
//! [Glossary](https://github.com/canardleteer/switchback-rs/blob/main/docs/GLOSSARY.md)
//! for terminology.
//!
//! **Protocol vs contract family:** contract family describes spec grammar
//! (OpenAPI, Protobuf, AsyncAPI); [protocol](https://github.com/canardleteer/switchback-rs/blob/main/docs/GLOSSARY.md#protocol)
//! describes invocation and transport semantics (`http`, `grpc`, custom). Entity
//! and contract nodes carry [`ProtocolAttachment`] lists populated by family
//! parsers and decoded by [`switchback-protocols`](https://docs.rs/switchback-protocols).
//! Structured HTTP method/path and gRPC streaming facts live in attachments, not
//! in [`OperationBody::signature`] alone.
//!
//! # Traits
//!
//! - [`ContractFamily`] and [`Contract`] — parser-side identity and loaded views
//! - [`ProtocolAttachment`] — transport envelope on contract and entity nodes
//! - [`Renderer`] / [`SyncRenderer`] — target-format rendering (async primary)
//! - [`SwitchbackCodec`] / [`SyncSwitchbackCodec`] — binary switchback I/O
//! - [`LinkExtractor`] / [`AsyncLinkExtractor`] — intra-link extraction
//! - [`LinkFormatter`] — resolved link string formatting
//! - [`CompanionStrategy`] / [`AsyncCompanionStrategy`] — companion discovery
//!
//! I/O traits follow [ADR 0002](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0002-async-first-traits-with-synchronous-secondary-apis-in-switchback-traits.md):
//! async-primary with sync-secondary APIs for external compatibility. All seam
//! types must traverse async task boundaries (`Send` / `Sync` as appropriate).
//!
//! Helper implementations (slug, link check, paths, companion discovery, prose
//! escaping) are partially centralized; companion nav metadata and ancestor
//! discovery live in [`companion`](crate::companion). Remaining helpers deferred.

mod companion;
mod error;
mod ids;
mod intra_links;
mod layout_paths;
mod link_context;
mod model;
mod options;
mod paths;
mod response_severity;
mod traits;

pub use companion::{
    companion_output_name_from_path, companion_output_name_from_segments,
    discover_ancestors_companions, module_path_from_output, normalize_rel_dir,
    source_dir_from_output, source_dir_string, title_from_markdown,
};
pub use error::{Result, SwitchbackError};
pub use ids::{EntityId, GroupId, ModuleId, SpecVersion};
pub use intra_links::{anchor, apply_intra_links, links_for_field};
pub use layout_paths::{
    decode_markdown_link_path, encode_markdown_link_path, heading_slug, layout_entity_rel_path,
    package_index_rel, package_page_rel, relative_path_from_dir, unique_heading_ids,
    LayoutEntityKey, ProtobufEntityKind,
};
pub use link_context::LinkContext;
pub use model::{
    Anchor, ChannelBody, Companion, ContractRef, Document, EntityBody, EntityRef, ExtensionBody,
    ExternalUrl, Group, GroupRef, IndexedEntity, IntraLink, LinkTarget, ManualContract, ManualRef,
    ManualRefInner, MessageBody, Module, ModuleRef, OperationBody, OperationRequestBodyRef,
    ParameterBody, ParameterRef, Property, ProtocolAttachment, RefKind, Reference, ReferenceManual,
    RequestBodyBody, ResolvedManual, ResponseBody, ResponseRef, SchemaBody, SecuritySchemeBody,
    ServiceBody, Source, SourceRef, Span, StoredEntity,
};
pub use options::{EscapeTags, Layout, OpenApiOperationSource, OpenApiSummaryLabel, Options};
pub use paths::{entity_category_dir, entity_rel_path};
pub use response_severity::ResponseSeverity;
pub use traits::{
    companion_files_to_stored, AsyncCompanionStrategy, AsyncContractLoader, AsyncLinkExtractor,
    CompanionDiscovery, CompanionStrategy, Contract, ContractFamily, Entity, EntityCategory,
    GenericCategory, LinkExtractor, LinkFormatter, OutputFile, RawDoc, Renderer, SupportedVersion,
    SwitchbackCodec, SyncRenderer, SyncSwitchbackCodec, VersionStatus,
};

pub use model::CompanionFile;

#[cfg(test)]
mod tests;
