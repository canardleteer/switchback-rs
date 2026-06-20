//! In-memory switchback model types.

mod contract;
mod entity;
mod link;
mod manual;
mod protocol;
mod resolved;

pub use contract::{Companion, Group, ManualContract};
pub use entity::{
    ChannelBody, EntityBody, ExtensionBody, MessageBody, OperationBody, OperationRequestBodyRef,
    ParameterBody, ParameterRef, Property, RequestBodyBody, ResponseBody, ResponseRef, SchemaBody,
    SecuritySchemeBody, ServiceBody, StoredEntity,
};
pub use link::{
    Anchor, ContractRef, EntityRef, ExternalUrl, GroupRef, IntraLink, LinkTarget, ManualRef,
    ManualRefInner, ModuleRef, RefKind, Reference,
};
pub use manual::{CompanionFile, Document, Module, ReferenceManual, Source, SourceRef, Span};
pub use protocol::ProtocolAttachment;
pub use resolved::{IndexedEntity, ResolvedManual};
