//! Protobuf fence wrapping and CEL splitting.

use switchback_protocols::ProtocolRegistry;
use switchback_traits::{
    EntityRef, EscapeTags, IntraLink, LinkContext, LinkFormatter, OperationBody,
    OperationRequestBodyRef, ParameterRef, ProtocolAttachment, RefKind, Reference, ResponseRef,
    StoredEntity, apply_intra_links,
};

use crate::highlight::split_message_cel_blocks;
use crate::render::markdown_doc::format_markdown_doc;
use crate::render::md_heading;

const OPERATION_SUBSECTION_LEVEL: usize = 4;

pub fn proto_file_name(entity: &StoredEntity) -> String {
    entity
        .source
        .as_ref()
        .map(|s| s.file.clone())
        .unwrap_or_default()
}

pub fn render_proto_fence(
    file_name: &str,
    entity_doc: Option<&str>,
    body: &str,
    escape_tags: EscapeTags,
    intra_links: &[IntraLink],
    formatter: &dyn LinkFormatter,
    ctx: &LinkContext,
) -> String {
    render_schema_fence(
        "protobuf",
        file_name,
        entity_doc,
        body,
        escape_tags,
        intra_links,
        formatter,
        ctx,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn render_schema_fence(
    fence_language: &str,
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
    if fence_language == "protobuf" {
        let (body, cel_blocks) = split_message_cel_blocks(body);
        push_fence_body(&mut out, fence_language, &body);
        for block in cel_blocks {
            out.push_str("**Protovalidate (CEL)**\n\n");
            push_fence_body(&mut out, "cel", &block);
        }
    } else {
        push_fence_body(&mut out, fence_language, body);
    }
    out
}

pub fn push_proto_fence_body(out: &mut String, body: &str) {
    push_fence_body(out, "protobuf", body);
}

pub fn push_fence_body(out: &mut String, language: &str, body: &str) {
    out.push_str(&format!("```{language}\n"));
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
    refs: &[Reference],
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

pub fn openapi_operation_markdown(
    entity: &StoredEntity,
    body: &OperationBody,
    group: &str,
    escape_tags: EscapeTags,
    formatter: &dyn LinkFormatter,
    ctx: &LinkContext,
) -> String {
    let from = ctx
        .render_from
        .as_deref()
        .unwrap_or_else(|| std::path::Path::new(&ctx.markdown_root));
    let module = entity
        .refs
        .iter()
        .find(|r| !r.target.module.is_empty())
        .map(|r| r.target.module.as_str())
        .unwrap_or("");
    let mut out = if body.signature.contains(" -> ") {
        format_openrpc_method_line(
            &body.signature,
            &body.parameters,
            &body.responses,
            ctx,
            from,
        )
    } else {
        format_method_path_line(&body.signature, &body.protocols)
    };
    if let Some(doc) = entity.doc.as_deref() {
        let doc = apply_intra_links("doc", doc, &entity.intra_links, formatter, ctx);
        let doc = link_structural_refs_in_prose(&doc, &entity.refs, module, group, ctx, from);
        push_markdown_doc(&mut out, &doc, escape_tags);
    }
    if !body.parameters.is_empty() {
        out.push_str(&md_heading(OPERATION_SUBSECTION_LEVEL, "Parameters"));
        out.push_str("| Name | In | Type | Required | Description |\n");
        out.push_str("| --- | --- | --- | --- | --- |\n");
        for param in &body.parameters {
            out.push('|');
            out.push(' ');
            out.push_str(&format_openapi_parameter_name(param, ctx, from));
            out.push_str(" | ");
            out.push_str(&param.location);
            out.push_str(" | ");
            out.push_str(&format_openapi_type(
                param.type_label.as_str(),
                &param.schema_ref,
                ctx,
                from,
            ));
            out.push_str(" | ");
            out.push_str(if param.required {
                "required"
            } else {
                "optional"
            });
            out.push_str(" | ");
            let description = link_structural_refs_in_prose(
                &param.description,
                &entity.refs,
                module,
                group,
                ctx,
                from,
            );
            out.push_str(&escape_table_cell(&description));
            out.push_str(" |\n");
        }
        out.push('\n');
    }
    if let Some(request_body) = &body.request_body {
        out.push_str(&md_heading(OPERATION_SUBSECTION_LEVEL, "Request body"));
        out.push_str(&format_openapi_request_body(request_body, ctx, from));
        out.push('\n');
    }
    if !body.responses.is_empty() {
        out.push_str(&md_heading(OPERATION_SUBSECTION_LEVEL, "Responses"));
        out.push_str("| Status | Description | Media type | Schema |\n");
        out.push_str("| --- | --- | --- | --- |\n");
        for response in &body.responses {
            out.push('|');
            out.push(' ');
            out.push_str(&response.status);
            out.push_str(" | ");
            let description = link_structural_refs_in_prose(
                &response.description,
                &entity.refs,
                module,
                group,
                ctx,
                from,
            );
            out.push_str(&escape_table_cell(&description));
            out.push_str(" | ");
            if response.media_type.is_empty() {
                out.push('—');
            } else {
                out.push_str(&escape_table_cell(&response.media_type));
            }
            out.push_str(" | ");
            let schema = if body.signature.contains(" -> ") {
                let result_type = body
                    .signature
                    .split_once(" -> ")
                    .map(|(_, label)| label)
                    .unwrap_or("");
                format_schema_type_display(result_type, &response.schema_ref, ctx, from)
            } else {
                format_openapi_response_schema(response, ctx, from)
            };
            out.push_str(&schema);
            out.push_str(" |\n");
        }
        out.push('\n');
    }
    out
}

fn format_method_path_line(signature: &str, protocols: &[ProtocolAttachment]) -> String {
    let registry = ProtocolRegistry::with_builtins();
    if let Some(meta) = registry.http_operation_from_attachments(protocols) {
        let mut line = format!("**{}** `{}`", meta.method, meta.path_template);
        let mut hints = Vec::new();
        if meta.request_streaming {
            hints.push("request stream");
        }
        if meta.response_streaming {
            hints.push("response stream");
        }
        if !hints.is_empty() {
            line.push_str(" — ");
            line.push_str(&hints.join(", "));
        }
        return format!("{line}\n\n");
    }
    let Some((method, path)) = signature.split_once(' ') else {
        return format!("**{signature}**\n\n");
    };
    format!("**{method}** `{path}`\n\n")
}

fn format_openrpc_method_line(
    signature: &str,
    parameters: &[ParameterRef],
    responses: &[ResponseRef],
    ctx: &LinkContext,
    from: &std::path::Path,
) -> String {
    let Some((_, result_plain)) = signature.split_once(" -> ") else {
        return format!("{signature}\n\n");
    };
    let method_head = signature
        .split_once('(')
        .map(|(head, _)| head)
        .unwrap_or(signature);
    let param_parts: Vec<String> = parameters
        .iter()
        .map(|param| {
            let name = format!("`{}`", param.name);
            let ty = format_openapi_type(param.type_label.as_str(), &param.schema_ref, ctx, from);
            format!("{name}: {ty}")
        })
        .collect();
    let params = if param_parts.is_empty() {
        String::new()
    } else {
        param_parts.join(", ")
    };
    let result = responses
        .first()
        .map(|response| format_schema_type_display(result_plain, &response.schema_ref, ctx, from))
        .unwrap_or_else(|| {
            if result_plain.is_empty() || result_plain == "void" {
                "`void`".into()
            } else {
                format!("`{result_plain}`")
            }
        });
    format!("{method_head}({params}) -> {result}\n\n")
}

fn format_schema_type_display(
    type_label: &str,
    schema_ref: &Reference,
    ctx: &LinkContext,
    from: &std::path::Path,
) -> String {
    if let Some(item_label) = type_label.strip_suffix("[]") {
        if !item_label.is_empty() {
            let linked = format_openapi_type(item_label, schema_ref, ctx, from);
            if linked != "—" {
                return format!("{linked}[]");
            }
        }
    }
    let linked = format_openapi_type(type_label, schema_ref, ctx, from);
    if linked != "—" {
        return linked;
    }
    if type_label.is_empty() || type_label == "void" {
        "`void`".into()
    } else {
        format!("`{type_label}`")
    }
}

fn format_openapi_parameter_name(
    param: &ParameterRef,
    ctx: &LinkContext,
    from: &std::path::Path,
) -> String {
    if param.schema_ref.kind == RefKind::Inline {
        return format!("`{}`", param.name);
    }
    let target = switchback_traits::EntityRef {
        module: param.schema_ref.target.module.clone(),
        group: param.schema_ref.target.group.clone(),
        category: "parameter".into(),
        name: param.name.clone(),
    };
    ctx.link_entity(from, &target)
}

fn format_openapi_type(
    type_label: &str,
    schema_ref: &Reference,
    ctx: &LinkContext,
    from: &std::path::Path,
) -> String {
    if schema_ref.kind != RefKind::Inline && !schema_ref.target.category.is_empty() {
        link_ref(schema_ref, ctx, from)
    } else if type_label.is_empty() {
        "—".into()
    } else {
        format!("`{type_label}`")
    }
}

fn format_openapi_request_body(
    body: &OperationRequestBodyRef,
    ctx: &LinkContext,
    from: &std::path::Path,
) -> String {
    let schema = format_openapi_type(body.type_label.as_str(), &body.schema_ref, ctx, from);
    let required = if body.required {
        "required"
    } else {
        "optional"
    };
    if body.media_type.is_empty() {
        format!("{schema} ({required})\n\n")
    } else {
        format!("`{}`: {schema} ({required})\n\n", body.media_type)
    }
}

fn format_openapi_response_schema(
    response: &ResponseRef,
    ctx: &LinkContext,
    from: &std::path::Path,
) -> String {
    if response.schema_ref.kind != RefKind::Inline
        && !response.schema_ref.target.category.is_empty()
    {
        format_openapi_type("", &response.schema_ref, ctx, from)
    } else if response.status.chars().all(|c| c.is_ascii_digit()) {
        format!("`{}`", response.status)
    } else {
        "—".into()
    }
}

fn escape_table_cell(value: &str) -> String {
    value.replace('|', "\\|")
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

pub fn link_ref(reference: &Reference, ctx: &LinkContext, from: &std::path::Path) -> String {
    if !reference.target.category.is_empty() {
        return ctx.link_entity(from, &reference.target);
    }
    let fqn = format!(
        ".{}.{name}",
        reference.target.group,
        name = reference.target.name
    );
    ctx.link_type(from, &fqn)
}

pub fn entity_module_group(entity: &StoredEntity) -> (&str, &str) {
    entity
        .refs
        .first()
        .map(|r| (r.target.module.as_str(), r.target.group.as_str()))
        .unwrap_or(("", ""))
}

pub fn link_structural_refs_in_prose(
    doc: &str,
    refs: &[Reference],
    module: &str,
    group: &str,
    ctx: &LinkContext,
    from: &std::path::Path,
) -> String {
    let doc = link_qualified_backtick_schema_refs(doc, refs, module, group, ctx, from);
    let doc = link_markdown_backticks_by_reference(&doc, refs, ctx, from);
    let doc = link_markdown_backticks_for_group_schemas(&doc, module, group, ctx, from);
    link_bare_structural_refs_in_tables(&doc, refs, ctx, from)
}

fn link_qualified_backtick_schema_refs(
    doc: &str,
    refs: &[Reference],
    module: &str,
    group: &str,
    ctx: &LinkContext,
    from: &std::path::Path,
) -> String {
    let mut targets: Vec<(String, EntityRef)> = refs
        .iter()
        .filter(|r| !r.target.name.is_empty() && !r.target.category.is_empty())
        .map(|r| (r.target.name.clone(), r.target.clone()))
        .collect();
    if !module.is_empty() && !group.is_empty() {
        targets.extend(
            ctx.entity_paths
                .keys()
                .filter(|key| key.module == module && key.group == group)
                .map(|key| (key.name.clone(), key.clone())),
        );
    }
    targets.sort_by_key(|(name, _)| std::cmp::Reverse(name.len()));
    let mut seen = std::collections::HashSet::new();
    targets.retain(|(name, _)| seen.insert(name.clone()));

    let mut out = doc.to_string();
    for (name, entity_ref) in targets {
        let link = ctx.link_entity(from, &entity_ref);
        if !link.starts_with('[') {
            continue;
        }
        let Some(url) = markdown_link_target(&link) else {
            continue;
        };
        let prefix = format!("`{name}.");
        let mut rebuilt = String::new();
        let mut rest = out.as_str();
        while let Some(idx) = rest.find(&prefix) {
            rebuilt.push_str(&rest[..idx]);
            let after_prefix = &rest[idx + prefix.len()..];
            let Some(end) = after_prefix.find('`') else {
                rebuilt.push_str(&rest[idx..]);
                return rebuilt;
            };
            let suffix = &after_prefix[..end];
            rebuilt.push('[');
            rebuilt.push_str(&name);
            rebuilt.push_str("](");
            rebuilt.push_str(url);
            rebuilt.push_str(").");
            rebuilt.push_str(suffix);
            rest = &after_prefix[end + 1..];
        }
        rebuilt.push_str(rest);
        out = rebuilt;
    }
    out
}

fn markdown_link_target(link: &str) -> Option<&str> {
    let open = link.find("](")?;
    let close = link[open + 2..].find(')')?;
    Some(&link[open + 2..open + 2 + close])
}

fn link_bare_structural_refs_in_tables(
    doc: &str,
    refs: &[Reference],
    ctx: &LinkContext,
    from: &std::path::Path,
) -> String {
    let mut sorted: Vec<_> = refs
        .iter()
        .filter(|r| !r.target.name.is_empty() && !r.target.category.is_empty())
        .collect();
    sorted.sort_by_key(|r| std::cmp::Reverse(r.target.name.len()));
    let mut out = doc.to_string();
    for reference in sorted {
        let link = link_ref(reference, ctx, from);
        if !link.starts_with('[') {
            continue;
        }
        let cell = format!("| {} |", reference.target.name);
        let linked = format!("| {link} |");
        out = out.replace(&cell, &linked);
    }
    out
}

fn link_markdown_backticks_by_reference(
    doc: &str,
    refs: &[Reference],
    ctx: &LinkContext,
    from: &std::path::Path,
) -> String {
    let mut out = doc.to_string();
    let mut sorted: Vec<_> = refs
        .iter()
        .filter(|r| !r.target.name.is_empty() && !r.target.category.is_empty())
        .collect();
    sorted.sort_by_key(|r| std::cmp::Reverse(r.target.name.len()));
    for reference in sorted {
        let link = link_ref(reference, ctx, from);
        if !link.starts_with('[') {
            continue;
        }
        let needle = format!("`{}`", reference.target.name);
        out = out.replace(&needle, &link);
    }
    out
}

fn link_markdown_backticks_for_group_schemas(
    doc: &str,
    module: &str,
    group: &str,
    ctx: &LinkContext,
    from: &std::path::Path,
) -> String {
    if module.is_empty() || group.is_empty() {
        return doc.to_string();
    }
    let mut names: Vec<String> = ctx
        .entity_paths
        .keys()
        .filter(|key| key.module == module && key.group == group && key.category == "schema")
        .map(|key| key.name.clone())
        .collect();
    names.sort_by_key(|name| std::cmp::Reverse(name.len()));
    let mut out = doc.to_string();
    for name in names {
        let entity_ref = EntityRef {
            module: module.to_string(),
            group: group.to_string(),
            category: "schema".into(),
            name: name.clone(),
        };
        let link = ctx.link_entity(from, &entity_ref);
        if !link.starts_with('[') {
            continue;
        }
        let needle = format!("`{name}`");
        out = out.replace(&needle, &link);
    }
    out
}
