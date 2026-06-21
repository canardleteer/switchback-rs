//! Integration tests for mdbook-protobuf-highlight preprocessor behavior.

use switchback_mdbook::highlight::{HighlightConfig, transform_chapter};

#[test]
fn preprocessor_rewrites_protobuf_fence_to_html() {
    let md = "```protobuf\nsyntax = \"proto3\";\n```\n";
    let out = transform_chapter(md, HighlightConfig::all()).expect("transform");
    assert!(out.contains("<pre class=\"protobuf-mdbook language-protobuf\">"));
    assert!(out.contains("hljs-keyword"));
    assert!(!out.contains("```protobuf"));
}

#[test]
fn preprocessor_splits_message_level_cel_in_fence() {
    let md = r#"```protobuf
message M {
  option (buf.validate.message).cel = {
    id: "rule.one"
    expression: "true"
  };
}
```"#;
    let out = transform_chapter(md, HighlightConfig::all()).expect("transform");
    assert!(out.contains("language-protobuf"));
    assert!(out.contains("language-cel"));
    assert!(out.contains("rule.one"));
}

#[test]
fn preprocessor_respects_disabled_protobuf() {
    let md = "```protobuf\nsyntax = \"proto3\";\n```\n";
    let out = transform_chapter(
        md,
        HighlightConfig {
            protobuf: false,
            cel: true,
        },
    )
    .expect("transform");
    assert!(out.contains("```protobuf"));
}

#[test]
fn preprocessor_idempotent_on_companion_markdown() {
    let md = r#"# Companion

Some prose with a fence:

```protobuf
syntax = "proto3";
package acme.v1;
```
"#;
    let once = transform_chapter(md, HighlightConfig::all()).expect("first pass");
    let twice = transform_chapter(&once, HighlightConfig::all()).expect("second pass");
    assert_eq!(once, twice, "second pass must not re-wrap highlighted HTML");
    assert!(
        !twice.contains("```protobuf"),
        "companion markdown should stay fence-free after highlight"
    );
}
