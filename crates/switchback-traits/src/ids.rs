//! Stable identifiers and version newtypes.

use std::fmt;

/// Intra-contract grouping identifier (protobuf package, OpenAPI tag, etc.).
///
/// Serialized on the wire as the group key within a [`ManualContract`](crate::ManualContract).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GroupId(
    /// Wire-safe group key string (e.g. protobuf package name, OpenAPI tag).
    pub String,
);

impl GroupId {
    /// Borrows the underlying group key.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for GroupId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for GroupId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl fmt::Display for GroupId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Entity address within a manual: group + category + name.
///
/// Used in parser-side [`Entity`](crate::traits::Entity) views and the
/// [`ResolvedManual`](crate::ResolvedManual) reverse index. Not a standalone wire
/// message; wire addresses use [`EntityRef`](crate::EntityRef) with module scope.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityId {
    /// Group containing the entity ([`GroupId`] within one contract).
    pub group: GroupId,
    /// Family-specific category slug (e.g. `"schemas"`, `"operations"`).
    pub category: String,
    /// Entity name within the group and category.
    pub name: String,
}

impl EntityId {
    /// Builds an entity id from group, category, and name components.
    pub fn new(
        group: impl Into<GroupId>,
        category: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            group: group.into(),
            category: category.into(),
            name: name.into(),
        }
    }
}

/// Top-level module identifier within a reference manual.
///
/// Serialized on the wire as the module key on [`EntityRef`](crate::EntityRef) and
/// related cross-reference types.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModuleId(
    /// Wire-safe module key string.
    pub String,
);

impl ModuleId {
    /// Borrows the underlying module key.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for ModuleId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ModuleId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl fmt::Display for ModuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Contract-family spec version string (e.g. `"3.1.1"`, `"2.6.0"`).
///
/// Serialized on the wire on [`ManualContract`](crate::ManualContract) and
/// [`ContractRef`](crate::ContractRef).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SpecVersion(
    /// Family-specific version label (not the switchback container version).
    pub String,
);

impl SpecVersion {
    /// Borrows the underlying version string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for SpecVersion {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for SpecVersion {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl fmt::Display for SpecVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
