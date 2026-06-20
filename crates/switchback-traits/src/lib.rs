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
//! # Traits
//!
//! - [`ContractFamily`] and [`Contract`] — parser-side identity and loaded views
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
//! Helper implementations (slug, link check, paths, prose escaping) are deferred
//! to follow-up work; this crate ships trait definitions, model types, and
//! [`entity_rel_path`] / [`ResolvedManual::from_reference_manual`].

mod error;
mod ids;
mod intra_links;
mod layout_paths;
mod link_context;
mod model;
mod options;
mod paths;
mod traits;

pub use error::{Result, SwitchbackError};
pub use ids::{EntityId, GroupId, ModuleId, SpecVersion};
pub use intra_links::{anchor, apply_intra_links, links_for_field};
pub use layout_paths::{
    heading_slug, layout_entity_rel_path, package_index_rel, package_page_rel,
    relative_path_from_dir, LayoutEntityKey, ProtobufEntityKind,
};
pub use link_context::LinkContext;
pub use model::{
    Anchor, ChannelBody, Companion, ContractRef, Document, EntityBody, EntityRef, ExtensionBody,
    ExternalUrl, Group, GroupRef, IndexedEntity, IntraLink, LinkTarget, ManualContract, ManualRef,
    ManualRefInner, MessageBody, Module, ModuleRef, OperationBody, ParameterBody, ParameterRef,
    Property, RefKind, Reference, ReferenceManual, RequestBodyBody, ResolvedManual, ResponseBody,
    ResponseRef, SchemaBody, SecuritySchemeBody, ServiceBody, Source, SourceRef, Span,
    StoredEntity,
};
pub use options::{EscapeTags, Layout, Options};
pub use paths::entity_rel_path;
pub use traits::{
    companion_files_to_stored, AsyncCompanionStrategy, AsyncContractLoader, AsyncLinkExtractor,
    CompanionDiscovery, CompanionStrategy, Contract, ContractFamily, Entity, EntityCategory,
    GenericCategory, LinkExtractor, LinkFormatter, OutputFile, RawDoc, Renderer, SupportedVersion,
    SwitchbackCodec, SyncRenderer, SyncSwitchbackCodec, VersionStatus,
};

pub use model::CompanionFile;

#[cfg(test)]
mod tests;
