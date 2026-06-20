//! Package-rollup markdown (`layout=package`).

use switchback_traits::{
    apply_intra_links, EntityBody, Group, LinkContext, LinkFormatter, Options, StoredEntity,
};

use crate::render::fence::{
    operation_signature_markdown, push_markdown_doc, push_proto_fence_body, render_proto_fence,
};
use crate::render::{md_heading, push_paragraph_break};

const SECTION_LEVEL: usize = 2;
const ENTITY_LEVEL: usize = 3;

pub fn render_package_page(
    group: &Group,
    entities: &[StoredEntity],
    links: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> (String, String) {
    let package = group.id.as_str();
    let rel = links.package_page_rel(package);
    let path = opts.output_path(rel.to_string_lossy().as_ref());
    let mut ctx = links.clone();
    ctx.render_from = Some(rel.clone());

    let mut out = String::new();
    out.push_str(&md_heading(1, package));

    if group.overview.as_ref().is_some_and(|o| !o.is_empty()) {
        let overview = format_markdown_overview(group.overview.as_deref().unwrap(), opts);
        push_markdown_doc(&mut out, &overview, opts.escape_tags);
        push_paragraph_break(&mut out);
    }

    let services: Vec<_> = entities
        .iter()
        .filter(|e| matches!(e.body, EntityBody::Service(_)))
        .collect();
    if !services.is_empty() {
        out.push_str(&md_heading(SECTION_LEVEL, "Services"));
        for svc in services {
            render_service_section(&mut out, svc, entities, &ctx, opts, formatter);
        }
    }

    let schemas: Vec<_> = entities
        .iter()
        .filter(|e| matches!(e.body, EntityBody::Schema(_)))
        .collect();
    if !schemas.is_empty() {
        out.push_str(&md_heading(SECTION_LEVEL, "Messages and enums"));
        for schema in schemas {
            render_schema_section(&mut out, schema, &ctx, opts, formatter);
        }
    }

    (path, out)
}

fn render_service_section(
    out: &mut String,
    svc: &StoredEntity,
    entities: &[StoredEntity],
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) {
    out.push_str(&md_heading(ENTITY_LEVEL, &svc.title));
    let file_name = proto_file_name(svc);
    if let Some(doc) = svc.doc.as_deref() {
        let doc = apply_intra_links("doc", doc, &svc.intra_links, formatter, ctx);
        push_markdown_doc(out, &doc, opts.escape_tags);
    }
    if !file_name.is_empty() {
        out.push_str(&format!("*`{file_name}`*\n\n"));
    }

    let prefix = format!("{}.", svc.title);
    for op in entities
        .iter()
        .filter(|e| matches!(e.body, EntityBody::Operation(_)) && e.name.starts_with(&prefix))
    {
        render_operation_section(out, op, ctx, opts, formatter);
    }
}

fn render_operation_section(
    out: &mut String,
    op: &StoredEntity,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) {
    let EntityBody::Operation(body) = &op.body else {
        return;
    };
    out.push_str(&operation_signature_markdown(
        &op.title,
        &body.signature,
        &op.refs,
        ctx,
    ));
    out.push_str("\n\n");
    if !body.fence_body.trim().is_empty() {
        let linked = crate::render::fence::link_proto_body(&body.fence_body, ctx);
        push_proto_fence_body(out, &linked);
    }
    if let Some(doc) = op.doc.as_deref() {
        let doc = apply_intra_links("doc", doc, &op.intra_links, formatter, ctx);
        push_markdown_doc(out, &doc, opts.escape_tags);
    }
}

fn render_schema_section(
    out: &mut String,
    schema: &StoredEntity,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) {
    out.push_str(&md_heading(ENTITY_LEVEL, &schema.title));
    let EntityBody::Schema(body) = &schema.body else {
        return;
    };
    let file_name = proto_file_name(schema);
    out.push_str(&render_proto_fence(
        &file_name,
        schema.doc.as_deref(),
        &body.fence_body,
        opts.escape_tags,
        &schema.intra_links,
        formatter,
        ctx,
    ));
}

fn proto_file_name(entity: &StoredEntity) -> String {
    entity
        .source
        .as_ref()
        .map(|s| s.file.clone())
        .unwrap_or_default()
}

fn format_markdown_overview(overview: &str, opts: &Options) -> String {
    crate::render::markdown_doc::format_markdown_doc(overview, opts.escape_tags)
}
