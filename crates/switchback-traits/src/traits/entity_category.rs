//! `EntityCategory` marker trait and renderer-known generic categories.

use std::fmt::Debug;
use std::hash::Hash;

/// Typed entity category owned by each contract family.
///
/// Family parsers define a concrete enum or newtype implementing this trait.
/// Renderers use [`EntityCategory::to_generic`] to apply cross-family formatting
/// rules via [`GenericCategory`].
pub trait EntityCategory:
    Copy + Clone + Eq + PartialEq + Hash + Debug + Send + Sync + 'static
{
    /// Wire-safe category slug (e.g. `"schemas"`, `"operations"`).
    fn as_str(&self) -> &'static str;

    /// Output directory segment for entities of this category.
    fn dir(&self) -> &str;

    /// Prefix used when generating SUMMARY entries for this category.
    fn summary_prefix(&self) -> &str;

    /// Maps this family-specific category to a renderer-known generic bucket, if any.
    fn to_generic(&self) -> Option<GenericCategory>;
}

/// Categories renderers know how to format specially.
///
/// Cross-family abstraction for layout and template selection in renderer crates.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum GenericCategory {
    /// Schema or message type pages.
    Schema,
    /// RPC or HTTP operation pages.
    Operation,
    /// Service definition pages.
    Service,
    /// Fallback for categories without specialized rendering.
    Generic,
}
