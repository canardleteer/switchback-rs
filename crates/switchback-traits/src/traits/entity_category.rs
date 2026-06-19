//! `EntityCategory` marker trait and renderer-known generic categories.

use std::fmt::Debug;
use std::hash::Hash;

/// Typed entity category owned by each contract family.
pub trait EntityCategory:
    Copy + Clone + Eq + PartialEq + Hash + Debug + Send + Sync + 'static
{
    fn as_str(&self) -> &'static str;
    fn dir(&self) -> &str;
    fn summary_prefix(&self) -> &str;
    fn to_generic(&self) -> Option<GenericCategory>;
}

/// Categories renderers know how to format specially.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum GenericCategory {
    Schema,
    Operation,
    Service,
    Generic,
}
