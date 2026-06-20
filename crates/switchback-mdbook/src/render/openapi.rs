//! OpenAPI-specific mdBook rendering helpers.

use switchback_traits::{
    apply_intra_links, entity_category_dir, EntityBody, LinkContext, LinkFormatter, Options,
    ParameterBody, RequestBodyBody, ResponseBody, SchemaBody, SecuritySchemeBody, StoredEntity,
};

use crate::render::fence::{
    openapi_operation_markdown, proto_file_name, push_fence_body, render_schema_fence,
};
use crate::render::markdown_doc::format_markdown_doc;
use crate::render::operation_fence::render_openapi_operation_fence;
use crate::render::{md_heading, push_paragraph_break};

const SECTION_LEVEL: usize = 2;
const ENTITY_LEVEL: usize = 3;

pub fn is_openapi_family(family: &str) -> bool {
    family == "openapi"
}

pub fn category_section_title(category: &str) -> &'static str {
    match category {
        "operation" => "Operations",
        "schema" => "Schemas",
        "parameter" => "Parameters",
        "response" => "Responses",
        "request-body" => "Request bodies",
        "security-scheme" => "Security schemes",
        _ => "Entities",
    }
}

pub fn summary_prefix(category: &str) -> &'static str {
    match category {
        "schema" => "Schema",
        "operation" => "Operation",
        "parameter" => "Parameter",
        "response" => "Response",
        "request-body" => "Request body",
        "security-scheme" => "Security scheme",
        _ => "Entity",
    }
}

pub fn renderable_openapi_entities(entities: &[StoredEntity]) -> Vec<&StoredEntity> {
    entities
        .iter()
        .filter(|e| {
            matches!(
                e.body,
                EntityBody::Schema(_)
                    | EntityBody::Operation(_)
                    | EntityBody::Parameter(_)
                    | EntityBody::Response(_)
                    | EntityBody::RequestBody(_)
                    | EntityBody::SecurityScheme(_)
            )
        })
        .collect()
}

/// Category iteration order for OpenAPI index pages and SUMMARY entity lists.
pub fn openapi_index_categories() -> &'static [&'static str] {
    &[
        "operation",
        "schema",
        "parameter",
        "response",
        "request-body",
        "security-scheme",
    ]
}

pub(crate) fn openapi_category_rank(category: &str) -> usize {
    openapi_index_categories()
        .iter()
        .position(|c| *c == category)
        .unwrap_or(openapi_index_categories().len())
}

/// Render grouped index links (`## Operations`, `## Schemas`, …) for split/entity layouts.
pub fn render_openapi_index_sections(
    out: &mut String,
    entities: &[StoredEntity],
    group_dir: &str,
    markdown_root: &str,
    index_from: &std::path::Path,
    links: &LinkContext,
) {
    for category in openapi_index_categories() {
        let mut section: Vec<_> = renderable_openapi_entities(entities)
            .into_iter()
            .filter(|e| e.category == *category)
            .collect();
        if section.is_empty() {
            continue;
        }
        section.sort_by(|a, b| a.title.cmp(&b.title));
        out.push_str(&md_heading(SECTION_LEVEL, category_section_title(category)));
        for entity in section {
            let rel = openapi_entity_rel_path(markdown_root, group_dir, entity);
            let p = std::path::PathBuf::from(rel);
            let title = format!("{} {}", summary_prefix(&entity.category), entity.name);
            out.push_str("- ");
            out.push_str(&links.summary_link(index_from, &p, &title));
            out.push('\n');
        }
        out.push('\n');
    }
}

pub fn render_openapi_package_sections(
    out: &mut String,
    entities: &[StoredEntity],
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) {
    let categories = [
        "operation",
        "schema",
        "parameter",
        "response",
        "request-body",
        "security-scheme",
    ];
    for category in categories {
        let mut section: Vec<_> = entities.iter().filter(|e| e.category == category).collect();
        if section.is_empty() {
            continue;
        }
        section.sort_by(|a, b| a.title.cmp(&b.title));
        out.push_str(&md_heading(SECTION_LEVEL, category_section_title(category)));
        for entity in section {
            render_openapi_entity_section(out, entity, ctx, opts, formatter);
        }
    }
}

fn render_openapi_entity_section(
    out: &mut String,
    entity: &StoredEntity,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) {
    out.push_str(&md_heading(ENTITY_LEVEL, &entity.title));
    match &entity.body {
        EntityBody::Operation(body) => {
            out.push_str(&openapi_operation_markdown(
                body,
                entity.doc.as_deref(),
                &entity.intra_links,
                opts.escape_tags,
                formatter,
                ctx,
            ));
            render_openapi_operation_fence(
                out,
                &body.fence_language,
                &body.fence_body,
                opts.openapi_operation_source,
            );
        }
        EntityBody::Schema(body) => {
            out.push_str(&render_openapi_schema_fence(
                entity, body, ctx, opts, formatter,
            ));
        }
        EntityBody::Parameter(body) => {
            out.push_str(&render_openapi_parameter_fence(
                entity, body, ctx, opts, formatter,
            ));
        }
        EntityBody::Response(body) => {
            out.push_str(&render_openapi_response_fence(
                entity, body, ctx, opts, formatter,
            ));
        }
        EntityBody::RequestBody(body) => {
            out.push_str(&render_openapi_request_body_fence(
                entity, body, ctx, opts, formatter,
            ));
        }
        EntityBody::SecurityScheme(body) => {
            out.push_str(&render_openapi_security_scheme_fence(
                entity, body, ctx, opts, formatter,
            ));
        }
        _ => {}
    }
}

pub fn render_openapi_entity_page(
    entity: &StoredEntity,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> String {
    let mut page = md_heading(1, &entity.title);
    match &entity.body {
        EntityBody::Operation(body) => {
            page.push_str(&openapi_operation_markdown(
                body,
                entity.doc.as_deref(),
                &entity.intra_links,
                opts.escape_tags,
                formatter,
                ctx,
            ));
            render_openapi_operation_fence(
                &mut page,
                &body.fence_language,
                &body.fence_body,
                opts.openapi_operation_source,
            );
        }
        EntityBody::Schema(body) => {
            page.push_str(&render_openapi_schema_fence(
                entity, body, ctx, opts, formatter,
            ));
        }
        EntityBody::Parameter(body) => {
            page.push_str(&render_openapi_parameter_fence(
                entity, body, ctx, opts, formatter,
            ));
        }
        EntityBody::Response(body) => {
            page.push_str(&render_openapi_response_fence(
                entity, body, ctx, opts, formatter,
            ));
        }
        EntityBody::RequestBody(body) => {
            page.push_str(&render_openapi_request_body_fence(
                entity, body, ctx, opts, formatter,
            ));
        }
        EntityBody::SecurityScheme(body) => {
            page.push_str(&render_openapi_security_scheme_fence(
                entity, body, ctx, opts, formatter,
            ));
        }
        _ => {}
    }
    page
}

pub fn openapi_entity_rel_path(
    markdown_root: &str,
    group_dir: &str,
    entity: &StoredEntity,
) -> String {
    format!(
        "{markdown_root}/{}",
        switchback_traits::entity_rel_path(
            group_dir,
            entity_category_dir(&entity.category),
            &entity.name,
        )
    )
}

fn render_openapi_schema_fence(
    entity: &StoredEntity,
    body: &SchemaBody,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> String {
    render_schema_fence(
        &body.fence_language,
        &proto_file_name(entity),
        entity.doc.as_deref(),
        &body.fence_body,
        opts.escape_tags,
        &entity.intra_links,
        formatter,
        ctx,
    )
}

fn render_openapi_parameter_fence(
    entity: &StoredEntity,
    body: &ParameterBody,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> String {
    let mut out = String::new();
    let required = if body.required {
        "required"
    } else {
        "optional"
    };
    out.push_str(&format!("Location: `{}` ({required})\n\n", body.location));
    push_entity_doc(&mut out, entity, ctx, opts, formatter);
    push_fence_body(&mut out, &body.fence_language, &body.fence_body);
    out
}

fn render_openapi_response_fence(
    entity: &StoredEntity,
    body: &ResponseBody,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> String {
    let mut out = String::new();
    if !body.status.is_empty() {
        out.push_str(&format!("Status: `{}`\n\n", body.status));
    }
    if !body.media_type.is_empty() {
        out.push_str(&format!("Media type: `{}`\n\n", body.media_type));
    }
    push_entity_doc(&mut out, entity, ctx, opts, formatter);
    push_fence_body(&mut out, &body.fence_language, &body.fence_body);
    out
}

fn render_openapi_request_body_fence(
    entity: &StoredEntity,
    body: &RequestBodyBody,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> String {
    let mut out = String::new();
    let required = if body.required {
        "required"
    } else {
        "optional"
    };
    out.push_str(&format!("Required: {required}\n\n"));
    push_entity_doc(&mut out, entity, ctx, opts, formatter);
    push_fence_body(&mut out, &body.fence_language, &body.fence_body);
    out
}

fn render_openapi_security_scheme_fence(
    entity: &StoredEntity,
    body: &SecuritySchemeBody,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> String {
    let mut out = String::new();
    if !body.scheme_type.is_empty() {
        out.push_str(&format!("Type: `{}`\n\n", body.scheme_type));
    }
    push_entity_doc(&mut out, entity, ctx, opts, formatter);
    push_fence_body(&mut out, &body.fence_language, &body.fence_body);
    out
}

fn push_entity_doc(
    out: &mut String,
    entity: &StoredEntity,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) {
    if let Some(doc) = entity.doc.as_deref() {
        let doc = apply_intra_links("doc", doc, &entity.intra_links, formatter, ctx);
        out.push_str(&format_markdown_doc(&doc, opts.escape_tags));
        push_paragraph_break(out);
    }
}
