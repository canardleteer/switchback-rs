//! Stable identifiers and version newtypes.

use std::fmt;

/// Intra-contract grouping identifier (protobuf package, OpenAPI tag, etc.).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GroupId(pub String);

impl GroupId {
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityId {
    pub group: GroupId,
    pub category: String,
    pub name: String,
}

impl EntityId {
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModuleId(pub String);

impl ModuleId {
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SpecVersion(pub String);

impl SpecVersion {
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
