//! OpenAPI render output must resolve internal links.

mod common;

use common::{collect_tree, render_openapi, render_openapi_acme};
use switchback_mdbook::assert_tree;
use switchback_traits::Layout;

#[test]
fn openapi_acme_links_resolve_package_layout() {
    let out = render_openapi_acme(Layout::Package, "");
    assert_tree(out.path()).unwrap_or_else(|e| panic!("openapi acme package links: {e}"));
}

#[test]
fn openapi_acme_split_summary_lists_version_groups() {
    let out = render_openapi_acme(Layout::Split, "init");
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

#[test]
fn openapi_links_resolve_package_layout() {
    let out = render_openapi(Layout::Package, "");
    assert_tree(out.path()).unwrap_or_else(|e| panic!("openapi package links: {e}"));
}

#[test]
fn openapi_links_resolve_entity_layout() {
    let out = render_openapi(Layout::Entity, "");
    assert_tree(out.path()).unwrap_or_else(|e| panic!("openapi entity links: {e}"));
}

#[test]
fn openapi_links_resolve_split_layout() {
    let out = render_openapi(Layout::Split, "");
    assert_tree(out.path()).unwrap_or_else(|e| panic!("openapi split links: {e}"));
}

#[test]
fn openapi_split_summary_top_level_tag_groups() {
    let out = render_openapi(Layout::Split, "init");
    let sum = std::fs::read_to_string(out.path().join("src/SUMMARY.md")).expect("SUMMARY");
    let gameplay = sum.find("[gameplay]").or_else(|| sum.find("[Gameplay]"));
    let components = sum
        .find("- [Components]")
        .or_else(|| sum.find("- [components]"));
    assert!(gameplay.is_some(), "expected gameplay in SUMMARY: {sum}");
    assert!(
        components.is_some(),
        "expected Components in SUMMARY: {sum}"
    );
    assert!(
        gameplay.expect("gameplay") < components.expect("components"),
        "gameplay should appear before Components in SUMMARY"
    );
    assert!(
        !sum.contains("  - [gameplay]"),
        "gameplay should not nest under components: {sum}"
    );
}

#[test]
fn openapi_split_init_summary_lists_entities() {
    let out = render_openapi(Layout::Split, "init");
    let sum = std::fs::read_to_string(out.path().join("src/SUMMARY.md")).expect("SUMMARY");
    assert!(
        sum.contains("/board"),
        "init+split should list operation paths in SUMMARY: {sum}"
    );
    assert_tree(out.path()).unwrap_or_else(|e| panic!("openapi init split links: {e}"));
}

#[test]
fn openapi_package_layout_uses_yaml_fences() {
    let out = render_openapi(Layout::Package, "");
    let md: String = collect_tree(out.path()).into_values().collect();
    assert!(
        md.contains("```yaml"),
        "expected yaml code fences in openapi render"
    );
    assert!(
        md.contains("## Operations") || md.contains("## Schemas"),
        "expected category sections"
    );
}
