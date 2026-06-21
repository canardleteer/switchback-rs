//! AsyncAPI render output must resolve internal links.

mod common;

use common::{collect_tree, render_asyncapi};
use switchback_mdbook::assert_tree;
use switchback_traits::Layout;

#[test]
fn asyncapi_links_resolve_package_layout() {
    let out = render_asyncapi(Layout::Package, "");
    assert_tree(out.path()).unwrap_or_else(|e| panic!("asyncapi package links: {e}"));
}

#[test]
fn asyncapi_links_resolve_entity_layout() {
    let out = render_asyncapi(Layout::Entity, "");
    assert_tree(out.path()).unwrap_or_else(|e| panic!("asyncapi entity links: {e}"));
}

#[test]
fn asyncapi_links_resolve_split_layout() {
    let out = render_asyncapi(Layout::Split, "");
    assert_tree(out.path()).unwrap_or_else(|e| panic!("asyncapi split links: {e}"));
}

#[test]
fn asyncapi_package_layout_uses_yaml_fences() {
    let out = render_asyncapi(Layout::Package, "");
    let md: String = collect_tree(out.path()).into_values().collect();
    assert!(
        md.contains("```yaml") || md.contains("```json"),
        "expected fenced source in asyncapi render"
    );
    assert!(
        md.contains("## Channels")
            || md.contains("## Operations")
            || md.contains("## Messages")
            || md.contains("## Schemas"),
        "expected category sections"
    );
}
