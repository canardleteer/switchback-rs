//! Intra-link splice unit tests.

use crate::intra_links::{anchor, apply_intra_links};
use crate::link_context::LinkContext;
use crate::options::Layout;
use crate::{EntityRef, IntraLink, LinkFormatter, LinkTarget};

struct StubFormatter;

impl LinkFormatter for StubFormatter {
    fn name(&self) -> &'static str {
        "stub"
    }

    fn format(&self, target: &LinkTarget, _ctx: &LinkContext) -> String {
        match target {
            LinkTarget::Entity(e) => format!("[{}]", e.name),
            _ => String::new(),
        }
    }
}

#[test]
fn apply_intra_links_splices_field_spans() {
    let ctx = LinkContext::empty(Layout::Package, ".", "src/packages");
    let links = vec![IntraLink {
        anchor: anchor("doc", 4, 24),
        target: LinkTarget::Entity(EntityRef {
            module: "m".into(),
            group: "acme.example.v1".into(),
            category: "schema".into(),
            name: "User".into(),
        }),
        raw: "acme.example.v1.User".into(),
    }];
    let out = apply_intra_links(
        "doc",
        "See acme.example.v1.User for details.",
        &links,
        &StubFormatter,
        &ctx,
    );
    assert_eq!(out, "See [User] for details.");
}

#[test]
fn apply_intra_links_skips_unresolved_targets() {
    let ctx = LinkContext::empty(Layout::Package, ".", "src/packages");
    let links = vec![IntraLink {
        anchor: anchor("doc", 0, 3),
        target: LinkTarget::Unresolved,
        raw: "foo".into(),
    }];
    let out = apply_intra_links("doc", "foo bar", &links, &StubFormatter, &ctx);
    assert_eq!(out, "foo bar");
}
