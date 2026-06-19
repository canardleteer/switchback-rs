//! Stored contract, group, and companion shapes.

use std::path::PathBuf;

use crate::ids::{GroupId, SpecVersion};
use crate::model::entity::StoredEntity;
use crate::model::manual::Source;

/// Serialized contract within a [`ReferenceManual`](super::manual::ReferenceManual).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManualContract {
    pub family: String,
    pub version: SpecVersion,
    pub groups: Vec<Group>,
    pub companions: Vec<Companion>,
}

/// Intra-contract grouping unit (package, tag group, application, etc.).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Group {
    pub id: GroupId,
    pub dir: String,
    pub title: String,
    pub overview: Option<String>,
    pub source: Option<Source>,
    /// Populated in the serialized switchback; empty on parser-side [`Contract`](crate::traits::Contract) views.
    pub entities: Vec<StoredEntity>,
    /// Provenance path for the *`path`* line on package pages.
    pub source_path: PathBuf,
}

/// Companion document embedded in the switchback.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Companion {
    pub output_name: String,
    pub bytes: Vec<u8>,
    pub media_type: String,
}
