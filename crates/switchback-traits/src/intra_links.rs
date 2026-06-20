//! Intra-link splice helpers.

use crate::link_context::LinkContext;
use crate::LinkFormatter;
use crate::{Anchor, IntraLink, LinkTarget};

/// Applies resolved intra-links to a prose field, replacing anchored spans with
/// formatted link strings.
pub fn apply_intra_links(
    field: &str,
    content: &str,
    links: &[IntraLink],
    formatter: &dyn LinkFormatter,
    ctx: &LinkContext,
) -> String {
    let mut field_links: Vec<_> = links
        .iter()
        .filter(|l| l.anchor.field == field && !matches!(l.target, LinkTarget::Unresolved))
        .collect();
    if field_links.is_empty() {
        return content.to_string();
    }
    field_links.sort_by_key(|l| l.anchor.byte_start);
    let mut out = String::new();
    let mut cursor = 0usize;
    for link in field_links {
        let start = link.anchor.byte_start as usize;
        let end = link.anchor.byte_end as usize;
        if start < cursor || end > content.len() || start > end {
            continue;
        }
        out.push_str(&content[cursor..start]);
        out.push_str(&formatter.format(&link.target, ctx));
        cursor = end;
    }
    out.push_str(&content[cursor..]);
    out
}

/// Returns intra-links whose anchor targets `field`.
pub fn links_for_field<'a>(links: &'a [IntraLink], field: &str) -> Vec<&'a IntraLink> {
    links.iter().filter(|l| l.anchor.field == field).collect()
}

/// Builds an anchor for a byte span within a named field.
pub fn anchor(field: impl Into<String>, byte_start: u32, byte_end: u32) -> Anchor {
    Anchor {
        field: field.into(),
        byte_start,
        byte_end,
    }
}

#[cfg(test)]
mod tests;
