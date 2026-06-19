//! Intra-links, structural references, and link targets.

/// Structural cross-reference in an entity body (schema `$ref`, protobuf FQN, etc.).
///
/// Serialized on the wire as part of [`StoredEntity`](super::entity::StoredEntity).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reference {
    /// Address of the referenced entity within the manual.
    pub target: EntityRef,
    /// How the reference is expressed in source (internal, external, inline, etc.).
    pub kind: RefKind,
}

/// Classification of a structural reference in source.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RefKind {
    /// Reference kind not specified or unrecognized.
    #[default]
    Unspecified,
    /// Reference to another entity in the same manual.
    Internal,
    /// Reference to an entity outside the manual (by FQN or URL).
    External,
    /// OpenAPI-style component reference.
    Component,
    /// Inline definition with no separate entity target.
    Inline,
}

/// Address of an entity within a manual.
///
/// Wire-safe cross-reference key used in [`Reference`], [`IntraLink`] targets, and
/// the [`ResolvedManual`](super::resolved::ResolvedManual) index.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityRef {
    /// Owning [`Module`](super::manual::Module) id string.
    pub module: String,
    /// Group id string within the contract.
    pub group: String,
    /// Entity category slug.
    pub category: String,
    /// Entity name within the group and category.
    pub name: String,
}

/// Address of a group within a manual.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GroupRef {
    /// Owning module id string.
    pub module: String,
    /// Group id string within the contract.
    pub group: String,
}

/// Address of a contract within a manual.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ContractRef {
    /// Owning module id string.
    pub module: String,
    /// [`ContractFamily`](crate::traits::ContractFamily) name.
    pub family: String,
    /// Contract spec version string.
    pub version: String,
}

/// Address of a module within a manual.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ModuleRef {
    /// Module id string.
    pub module: String,
}

/// Cross-manual reference by URI with optional inner target.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManualRef {
    /// URI of the external manual (may be a file path or registry URL).
    pub uri: String,
    /// Version pin or label for the external manual, when specified.
    pub version: String,
    /// Optional entity or group target within the external manual.
    pub inner: Option<ManualRefInner>,
}

/// Inner target of a [`ManualRef`] cross-manual link.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ManualRefInner {
    /// Link to a specific entity in the external manual.
    Entity(EntityRef),
    /// Link to a group overview in the external manual.
    Group(GroupRef),
}

/// External URL link target (not resolved to an in-manual entity).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExternalUrl {
    /// Absolute or relative URL string as authored.
    pub url: String,
}

/// Prose-level link with anchor, resolved target, and raw author text.
///
/// Serialized on the wire as part of [`StoredEntity`](super::entity::StoredEntity).
/// The `target` field uses wire-safe variants only at serialize time; see
/// [`LinkTarget::Unresolved`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntraLink {
    /// Byte span locating the link within a prose field.
    pub anchor: Anchor,
    /// Resolved link destination.
    pub target: LinkTarget,
    /// Raw link text as authored in source prose.
    pub raw: String,
}

/// Byte span within a prose field (`doc`, `fence_body`, etc.).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Anchor {
    /// Name of the field containing the link (e.g. `"doc"`, `"fence_body"`).
    pub field: String,
    /// Inclusive start byte offset within `field`'s UTF-8 content.
    pub byte_start: u32,
    /// Exclusive end byte offset within `field`'s UTF-8 content.
    pub byte_end: u32,
}

/// Resolved intra-link destination.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinkTarget {
    /// Link to an entity in the current or another module.
    Entity(EntityRef),
    /// Link to a group overview page.
    Group(GroupRef),
    /// Link to a contract within a module.
    Contract(ContractRef),
    /// Link to a module overview page.
    Module(ModuleRef),
    /// Link to another manual (possibly with an inner entity/group target).
    Manual(ManualRef),
    /// Link to an external URL.
    External(ExternalUrl),
    /// Link target could not be resolved during extraction.
    ///
    /// In-memory only; not serialized on the wire. Parsers may emit this during
    /// extraction; codecs should reject or strip unresolved targets at serialize time.
    Unresolved,
}
