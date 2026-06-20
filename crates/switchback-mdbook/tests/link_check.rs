//! Generated markdown links must resolve across layout variants.

mod common;

use common::{load_examples, options_for, render_doc_rich, render_examples, render_to_tempdir};
use switchback_mdbook::assert_tree;
use switchback_traits::Layout;

#[test]
fn links_resolve_package_layout() {
    let out = render_examples(Layout::Package, "");
    assert_tree(out.path()).unwrap_or_else(|e| panic!("package links: {e}"));
    assert!(!out.path().join("book.toml").exists());
    assert!(
        !out.path().join("src/packages/buf.validate.md").exists(),
        "BSR protovalidate export must not be a protoc input"
    );
    assert!(
        out.path()
            .join("src/packages/acme.example.v1.README.md")
            .is_file(),
        "companion markdown copied beside protos"
    );
}

#[test]
fn optional_fields_preserved_in_generated_markdown() {
    let manual = load_examples();
    let opts = options_for(Layout::Package, "");
    let tree = common::render_manual(&manual, &opts);
    let pkg = tree
        .get("src/packages/acme.example.v1.md")
        .expect("v1 package md");
    assert!(
        pkg.contains("optional string locale"),
        "optional fields preserved"
    );
}

#[test]
fn links_resolve_entity_layout() {
    let out = render_examples(Layout::Entity, "");
    assert_tree(out.path()).unwrap_or_else(|e| panic!("entity links: {e}"));
}

#[test]
fn links_resolve_split_layout() {
    let out = render_examples(Layout::Split, "");
    assert_tree(out.path()).unwrap_or_else(|e| panic!("split links: {e}"));
}

#[test]
fn summary_without_init_emits_summary_only() {
    let out = render_examples(Layout::Package, "summary");
    assert_tree(out.path()).unwrap_or_else(|e| panic!("summary links: {e}"));
    assert!(out.path().join("src/SUMMARY.md").is_file());
    assert!(!out.path().join("book.toml").exists());
}

#[test]
fn summary_entity_layout_lists_entities() {
    let out = render_examples(Layout::Entity, "summary,no_proto_markdown");
    let sum = std::fs::read_to_string(out.path().join("src/SUMMARY.md")).expect("SUMMARY");
    assert!(sum.contains("Message "));
    assert!(sum.contains("Service "));
    assert_tree(out.path()).unwrap_or_else(|e| panic!("entity summary links: {e}"));
}

#[test]
fn summary_split_layout_lists_entities() {
    let out = render_examples(Layout::Split, "summary,no_proto_markdown");
    let sum = std::fs::read_to_string(out.path().join("src/SUMMARY.md")).expect("SUMMARY");
    assert!(sum.contains("Message "));
    assert_tree(out.path()).unwrap_or_else(|e| panic!("split summary links: {e}"));
}

#[test]
fn summary_no_proto_markdown_entity_on_fixture() {
    let out = render_doc_rich("summary,layout=entity,no_proto_markdown");
    let sum = std::fs::read_to_string(out.path().join("src/SUMMARY.md")).expect("SUMMARY");
    assert!(sum.contains("Message "));
    assert!(
        !out.path()
            .join("src/packages/acme.example.v1.README.md")
            .exists(),
        "no_proto_markdown skips companions"
    );
    assert_tree(out.path()).unwrap_or_else(|e| panic!("fixture entity summary: {e}"));
}

#[test]
fn custom_markdown_root_and_summary_path() {
    let out = render_examples(
        Layout::Package,
        "markdown_root=content/api,summary_path=content/SUMMARY.md,summary",
    );
    assert!(out.path().join("content/api/acme.example.v1.md").is_file());
    assert!(!out.path().join("src/packages").exists());
    assert!(out.path().join("content/SUMMARY.md").is_file());
    assert_tree(out.path()).unwrap_or_else(|e| panic!("custom path links: {e}"));
}

#[test]
fn book_root_prefixes_all_output() {
    let out = render_examples(Layout::Package, "book_root=docs");
    assert!(out
        .path()
        .join("docs/src/packages/acme.example.v1.md")
        .is_file());
    assert!(!out.path().join("src/packages").exists());
}

#[test]
fn book_option_infers_paths_from_book_toml() {
    let out = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        out.path().join("book.toml"),
        "[book]\ntitle = \"Test\"\nsrc = \"content\"\n",
    )
    .expect("book.toml");
    std::fs::create_dir_all(out.path().join("content")).expect("content dir");
    let book = out.path().to_string_lossy();
    let manual = load_examples();
    let opts = switchback_mdbook::parse_parameter(&Some(format!(
        "layout=package,book={book},mdbook_out={book},summary"
    )))
    .expect("parse");
    let rendered = render_to_tempdir(&manual, &opts);
    assert!(rendered
        .path()
        .join("content/packages/acme.example.v1.md")
        .is_file());
    assert!(!rendered.path().join("src/packages").exists());
    assert!(rendered.path().join("content/SUMMARY.md").is_file());
    assert_tree(rendered.path()).unwrap_or_else(|e| panic!("book= links: {e}"));
}

#[test]
fn book_option_explicit_markdown_root_overrides_inference() {
    let out = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        out.path().join("book.toml"),
        "[book]\ntitle = \"Test\"\nsrc = \"content\"\n",
    )
    .expect("book.toml");
    std::fs::create_dir_all(out.path().join("content")).expect("content dir");
    let book = out.path().to_string_lossy();
    let manual = load_examples();
    let opts = switchback_mdbook::parse_parameter(&Some(format!(
        "layout=package,book={book},mdbook_out={book},markdown_root=content/api"
    )))
    .expect("parse");
    let rendered = render_to_tempdir(&manual, &opts);
    assert!(rendered
        .path()
        .join("content/api/acme.example.v1.md")
        .is_file());
    assert!(!rendered.path().join("content/packages").exists());
}

#[test]
fn init_writes_mdbook_tree_and_readme() {
    let out = render_examples(Layout::Package, "init");
    assert!(out.path().join("book.toml").is_file());
    assert!(out.path().join("README.md").is_file());
    assert!(out.path().join("src/SUMMARY.md").is_file());
    let sum = std::fs::read_to_string(out.path().join("src/SUMMARY.md")).expect("SUMMARY");
    assert!(sum.starts_with("# Protobuf documentation"));
    assert!(sum.contains("acme.example.v1"));
    assert!(sum.contains("acme.README.md"));
    assert!(!sum.contains("Message "));
    assert!(!sum.contains("chapter_1"));
    let readme = std::fs::read_to_string(out.path().join("README.md")).expect("README");
    assert!(readme.contains("mdbook-mermaid"));
    assert!(readme.contains("rumdl"));
    assert!(readme.contains("switchback-mdbook"));
    assert!(readme.contains("Syntax highlighting") || readme.contains("CEL"));
    let book = std::fs::read_to_string(out.path().join("book.toml")).expect("book.toml");
    assert!(book.contains("switchback-mdbook: syntax highlighting"));
    assert!(book.contains("[preprocessor.protobuf-highlight]"));
    assert!(book.contains("mdbook-protobuf-highlight"));
    assert!(book.contains("theme/protobuf-highlight.css"));
    assert!(out.path().join("theme/protobuf-highlight.css").is_file());
    assert!(!out.path().join("theme/highlight-protobuf.js").exists());
    assert!(!out.path().join("theme/highlight-cel.js").exists());
    let index = std::fs::read_to_string(out.path().join("theme/index.hbs")).expect("index.hbs");
    assert!(!index.contains("switchback-mdbook: syntax highlight begin"));
    assert!(!index.contains("hljs.registerLanguage(\"protobuf\""));
    assert_tree(out.path()).unwrap_or_else(|e| panic!("init links: {e}"));
}

#[test]
fn init_no_proto_highlight_skips_protobuf_grammar_only() {
    let out = render_examples(Layout::Package, "init,no_proto_highlight");
    let book = std::fs::read_to_string(out.path().join("book.toml")).expect("book.toml");
    assert!(book.contains("protobuf = false"));
    assert!(book.contains("cel = true"));
}

#[test]
fn init_no_cel_highlight_skips_cel_grammar_only() {
    let out = render_examples(Layout::Package, "init,no_cel_highlight");
    let book = std::fs::read_to_string(out.path().join("book.toml")).expect("book.toml");
    assert!(book.contains("protobuf = true"));
    assert!(book.contains("cel = false"));
}

#[test]
fn package_layout_splits_message_cel_into_cel_fence() {
    let out = render_examples(Layout::Package, "");
    let v2 = std::fs::read_to_string(out.path().join("src/packages/acme.example.v2.md"))
        .expect("v2 package md");
    assert!(v2.contains("```cel"));
    assert!(v2.contains("numeric_range.min_lte_max"));
    let numeric = v2
        .split("### NumericRange")
        .nth(1)
        .expect("NumericRange section");
    let proto_fence = numeric
        .split("```protobuf")
        .nth(1)
        .and_then(|s| s.split("```").next())
        .expect("protobuf fence");
    assert!(!proto_fence.contains("buf.validate.message).cel"));
}

#[test]
fn init_entity_layout_summary_lists_entities() {
    let out = render_examples(Layout::Entity, "init");
    let sum = std::fs::read_to_string(out.path().join("src/SUMMARY.md")).expect("SUMMARY");
    assert!(sum.contains("Message "));
    assert_tree(out.path()).unwrap_or_else(|e| panic!("init entity links: {e}"));
}

#[test]
fn init_package_layout_summary_is_package_only() {
    let out = render_examples(Layout::Package, "init");
    let sum = std::fs::read_to_string(out.path().join("src/SUMMARY.md")).expect("SUMMARY");
    assert!(!sum.contains("Message "));
    assert!(!sum.contains("Enum "));
    assert_tree(out.path()).unwrap_or_else(|e| panic!("init package links: {e}"));
}

#[test]
fn package_layout_preserves_comments_and_mermaid_fences() {
    let out = render_examples(Layout::Package, "init");
    let book = std::fs::read_to_string(out.path().join("book.toml")).expect("book.toml");
    assert!(
        !book.contains("[preprocessor.mermaid]"),
        "renderer does not wire mdbook-mermaid; users configure diagrams themselves"
    );
    let pkg = std::fs::read_to_string(out.path().join("src/packages/acme.example.v1.md"))
        .expect("package md");
    let services = pkg.split("## Services").nth(1).expect("Services section");
    assert!(
        services.find("## Messages and enums").unwrap_or(usize::MAX)
            > services.find("### EchoService").unwrap_or(0),
        "Services section should precede Messages and enums"
    );
    let svc = services
        .split("### EchoService")
        .nth(1)
        .expect("EchoService section");
    let mermaid = svc.find("```mermaid").expect("mermaid in service docs");
    let unary_sig = svc.find("**EchoUnary** (").expect("EchoUnary signature");
    assert!(!svc.contains("### EchoUnary ("));
    assert!(mermaid < unary_sig);
    assert!(
        pkg.contains("hello-world RPC"),
        "EchoUnary method leading comments should appear as Markdown"
    );
    let unary_doc = svc.find("hello-world RPC").expect("EchoUnary doc");
    assert!(
        unary_sig < unary_doc,
        "signature and options before RPC prose"
    );
    assert!(
        !svc.contains("service EchoService"),
        "no monolithic service protobuf block"
    );
    assert!(svc.contains("option idempotency_level = NO_SIDE_EFFECTS;"));
}

#[test]
fn package_rpc_type_links_use_mdbook_heading_anchors() {
    let out = render_examples(Layout::Package, "");
    let pkg = std::fs::read_to_string(out.path().join("src/packages/acme.example.v1.md"))
        .expect("v1 package md");
    assert!(
        pkg.contains("[BatchEchoResponse](#batchechoresponse)"),
        "in-page links must use mdBook heading ids for RPC response types"
    );
    assert!(
        pkg.contains("**EchoBidiStream**"),
        "bidi RPC should appear in service docs"
    );
}

#[test]
fn alphabetize_services_sorts_service_headings_only() {
    let default_out = render_examples(Layout::Package, "");
    let sorted_out = render_examples(Layout::Package, "alphabetize_services");
    let default_pkg =
        std::fs::read_to_string(default_out.path().join("src/packages/acme.example.v1.md"))
            .expect("default v1");
    let sorted_pkg =
        std::fs::read_to_string(sorted_out.path().join("src/packages/acme.example.v1.md"))
            .expect("sorted v1");
    assert_ne!(default_pkg, sorted_pkg);

    let services = sorted_pkg
        .split("## Services")
        .nth(1)
        .expect("Services")
        .split("## Messages and enums")
        .next()
        .expect("messages boundary");
    let admin = services.find("### AdminService").expect("AdminService");
    let echo = services.find("### EchoService").expect("EchoService");
    assert!(admin < echo);

    let messages = sorted_pkg
        .split("## Messages and enums")
        .nth(1)
        .expect("Messages and enums");
    let default_messages = default_pkg
        .split("## Messages and enums")
        .nth(1)
        .expect("Messages and enums");
    assert_eq!(
        first_entity_heading(messages),
        first_entity_heading(default_messages),
        "alphabetize_services must not reorder Messages and enums"
    );
}

#[test]
fn alphabetize_messages_sorts_schema_headings_only() {
    let default_out = render_examples(Layout::Package, "");
    let sorted_out = render_examples(Layout::Package, "alphabetize_messages");
    let default_pkg =
        std::fs::read_to_string(default_out.path().join("src/packages/acme.example.v1.md"))
            .expect("default v1");
    let sorted_pkg =
        std::fs::read_to_string(sorted_out.path().join("src/packages/acme.example.v1.md"))
            .expect("sorted v1");
    assert_ne!(default_pkg, sorted_pkg);

    let services = sorted_pkg
        .split("## Services")
        .nth(1)
        .expect("Services")
        .split("## Messages and enums")
        .next()
        .expect("messages boundary");
    let default_services = default_pkg
        .split("## Services")
        .nth(1)
        .expect("Services")
        .split("## Messages and enums")
        .next()
        .expect("messages boundary");
    assert_eq!(
        first_entity_heading(services),
        first_entity_heading(default_services),
        "alphabetize_messages must not reorder Services"
    );

    let messages = sorted_pkg
        .split("## Messages and enums")
        .nth(1)
        .expect("Messages and enums");
    let batch = messages
        .find("### BatchEchoRequest")
        .expect("BatchEchoRequest");
    let unary = messages
        .find("### EchoUnaryRequest")
        .expect("EchoUnaryRequest");
    assert!(batch < unary);
}

fn first_entity_heading(section: &str) -> &str {
    section
        .lines()
        .find(|line| line.starts_with("### "))
        .expect("entity heading")
}
