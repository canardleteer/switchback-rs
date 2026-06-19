//! Intra-links, structural references, and link targets.

/// Structural cross-reference in an entity body (schema `$ref`, protobuf FQN, etc.).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reference {
    pub target: EntityRef,
    pub kind: RefKind,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RefKind {
    #[default]
    Unspecified,
    Internal,
    External,
    Component,
    Inline,
}

/// Address of an entity within a manual.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityRef {
    pub module: String,
    pub group: String,
    pub category: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GroupRef {
    pub module: String,
    pub group: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ContractRef {
    pub module: String,
    pub family: String,
    pub version: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ModuleRef {
    pub module: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManualRef {
    pub uri: String,
    pub version: String,
    pub inner: Option<ManualRefInner>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ManualRefInner {
    Entity(EntityRef),
    Group(GroupRef),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExternalUrl {
    pub url: String,
}

/// Prose-level link with anchor, resolved target, and raw author text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntraLink {
    pub anchor: Anchor,
    pub target: LinkTarget,
    pub raw: String,
}

/// Byte span within a prose field (`doc`, `fence_body`, etc.).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Anchor {
    pub field: String,
    pub byte_start: u32,
    pub byte_end: u32,
}

/// Resolved intra-link destination.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinkTarget {
    Entity(EntityRef),
    Group(GroupRef),
    Contract(ContractRef),
    Module(ModuleRef),
    Manual(ManualRef),
    External(ExternalUrl),
    Unresolved,
}
