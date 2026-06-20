//! Protobuf fence wrapping and CEL splitting.

use switchback_traits::{apply_intra_links, EscapeTags, IntraLink, LinkContext, LinkFormatter};

use crate::highlight::split_message_cel_blocks;
use crate::render::markdown_doc::format_markdown_doc;

pub fn render_proto_fence(
    file_name: &str,
    entity_doc: Option<&str>,
    body: &str,
    escape_tags: EscapeTags,
    intra_links: &[IntraLink],
    formatter: &dyn LinkFormatter,
    ctx: &LinkContext,
) -> String {
    let mut out = String::new();
    if let Some(c) = entity_doc {
        let doc = apply_intra_links("doc", c, intra_links, formatter, ctx);
        push_markdown_doc(&mut out, &doc, escape_tags);
    }
    if !file_name.is_empty() {
        out.push_str(&format!("*`{file_name}`*\n\n"));
    }
    let linked_body = link_proto_body(body, ctx);
    let (body, cel_blocks) = split_message_cel_blocks(&linked_body);
    push_proto_fence_body(&mut out, &body);
    for block in cel_blocks {
        out.push_str("**Protovalidate (CEL)**\n\n");
        push_cel_fence_body(&mut out, &block);
    }
    out
}

pub fn push_proto_fence_body(out: &mut String, body: &str) {
    out.push_str("```protobuf\n");
    out.push_str(body);
    if !body.ends_with('\n') {
        out.push('\n');
    }
    out.push_str("```\n\n");
}

fn push_cel_fence_body(out: &mut String, body: &str) {
    out.push_str("```cel\n");
    out.push_str(body);
    if !body.ends_with('\n') {
        out.push('\n');
    }
    out.push_str("```\n\n");
}

pub fn push_markdown_doc(out: &mut String, comment: &str, escape_tags: EscapeTags) {
    let formatted = format_markdown_doc(comment, escape_tags);
    out.push_str(&formatted);
    out.push_str("\n\n");
}

pub fn operation_signature_markdown(
    title: &str,
    signature: &str,
    refs: &[switchback_traits::Reference],
    ctx: &LinkContext,
) -> String {
    let from = ctx
        .render_from
        .as_deref()
        .unwrap_or_else(|| std::path::Path::new(&ctx.markdown_root));
    let (in_part, out_part) = split_signature_parts(signature);
    let input = refs
        .first()
        .map(|r| link_ref(r, ctx, from))
        .unwrap_or_else(|| in_part.clone());
    let output = refs
        .get(1)
        .map(|r| link_ref(r, ctx, from))
        .unwrap_or_else(|| out_part.clone());
    format!("**{title}** ( {input} ) returns ( {output} )")
}

fn split_signature_parts(signature: &str) -> (String, String) {
    let Some((left, right)) = signature.split_once(" returns ") else {
        return (signature.to_string(), String::new());
    };
    let input = left
        .split_once('(')
        .and_then(|(_, rest)| rest.strip_suffix(')'))
        .map(str::trim)
        .unwrap_or(left)
        .to_string();
    let output = right
        .trim()
        .strip_prefix('(')
        .and_then(|s| s.strip_suffix(')'))
        .map(str::trim)
        .unwrap_or(right)
        .to_string();
    (input, output)
}

fn link_ref(
    reference: &switchback_traits::Reference,
    ctx: &LinkContext,
    from: &std::path::Path,
) -> String {
    let fqn = format!(
        ".{}.{name}",
        reference.target.group,
        name = reference.target.name
    );
    ctx.link_type(from, &fqn)
}

pub fn link_proto_body(body: &str, ctx: &LinkContext) -> String {
    let from = ctx
        .render_from
        .as_deref()
        .unwrap_or_else(|| std::path::Path::new(&ctx.markdown_root));
    let mut names: Vec<(String, String)> = Vec::new();
    for key in ctx.layout_entity_keys() {
        if matches!(
            key.kind,
            switchback_traits::ProtobufEntityKind::Message
                | switchback_traits::ProtobufEntityKind::Enum
        ) {
            let fqn = format!(".{}.{name}", key.package, name = key.name);
            let link = ctx.link_type(from, &fqn);
            if link.starts_with('[') {
                names.push((key.name.clone(), link));
            }
        }
    }
    names.sort_by_key(|(n, _)| std::cmp::Reverse(n.len()));
    let mut out = body.to_string();
    for (name, link) in names {
        out = replace_type_token(&out, &name, &link);
    }
    out
}

fn replace_type_token(haystack: &str, name: &str, link: &str) -> String {
    let mut out = String::new();
    let mut rest = haystack;
    while let Some(idx) = rest.find(name) {
        let before = &rest[..idx];
        let after_char = rest.get(idx + name.len()..);
        let valid_start = idx == 0 || !rest.as_bytes()[idx - 1].is_ascii_alphanumeric();
        let valid_end = after_char
            .and_then(|s| s.chars().next())
            .is_none_or(|c| !c.is_ascii_alphanumeric());
        out.push_str(before);
        if valid_start && valid_end {
            out.push_str(link);
        } else {
            out.push_str(name);
        }
        rest = &rest[idx + name.len()..];
    }
    out.push_str(rest);
    out
}
