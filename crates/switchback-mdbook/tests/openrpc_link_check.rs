//! Acme OpenRPC render output must resolve internal links.

mod common;

use common::render_openrpc_acme;
use switchback_mdbook::assert_tree;
use switchback_traits::Layout;

#[test]
fn openrpc_acme_links_resolve_package_layout() {
    let out = render_openrpc_acme(Layout::Package, "");
    assert_tree(out.path()).unwrap_or_else(|e| panic!("openrpc acme package links: {e}"));
}

#[test]
fn openrpc_acme_split_summary_lists_version_groups() {
    let out = render_openrpc_acme(Layout::Split, "init");
    let sum = std::fs::read_to_string(out.path().join("src/SUMMARY.md")).expect("SUMMARY");
    assert!(
        sum.contains("acme.example.v1") || sum.contains("v1"),
        "expected v1 group in SUMMARY: {sum}"
    );
    assert!(
        sum.contains("acme.example.v2") || sum.contains("v2"),
        "expected v2 group in SUMMARY: {sum}"
    );
}
