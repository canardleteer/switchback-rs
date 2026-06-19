//! Stored contract, group, and companion shapes.

use std::path::PathBuf;

use crate::ids::{GroupId, SpecVersion};
use crate::model::entity::StoredEntity;
use crate::model::manual::Source;

/// Serialized contract within a [`ReferenceManual`](super::manual::ReferenceManual).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManualContract {
    /// [`ContractFamily`](crate::traits::ContractFamily) name (e.g. `"openapi"`, `"protobuf"`).
    pub family: String,
    /// Parsed spec version for this contract instance.
    pub version: SpecVersion,
    /// Intra-contract groups (packages, tags, applications, etc.).
    pub groups: Vec<Group>,
    /// Companion documents embedded in the switchback artifact.
    pub companions: Vec<Companion>,
}

/// Intra-contract grouping unit (package, tag group, application, etc.).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Group {
    /// Stable group key within the contract ([`GroupId`]).
    pub id: GroupId,
    /// Output directory segment for this group under the markdown root.
    pub dir: String,
    /// Human-readable group title for package/overview pages.
    pub title: String,
    /// Optional overview prose for the group page.
    pub overview: Option<String>,
    /// Provenance pointer into the switchback source layer, when available.
    pub source: Option<Source>,
    /// Entities belonging to this group.
    ///
    /// Populated in the serialized switchback; empty on parser-side
    /// [`Contract`](crate::traits::Contract) views (entities are queried separately).
    pub entities: Vec<StoredEntity>,
    /// Filesystem path to the contract input used for the package-page *path* line.
    ///
    /// Parser-local provenance only; not serialized on the wire.
    pub source_path: PathBuf,
}

/// Companion document embedded in the switchback.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Companion {
    /// Output filename relative to the companion output directory.
    pub output_name: String,
    /// Raw companion file bytes.
    pub bytes: Vec<u8>,
    /// MIME type for the companion (e.g. `"text/markdown"`).
    pub media_type: String,
}
