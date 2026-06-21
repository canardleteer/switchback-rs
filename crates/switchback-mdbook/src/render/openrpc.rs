//! OpenRPC-specific mdBook rendering helpers.

use switchback_traits::{
    EntityBody, LinkContext, LinkFormatter, OperationBody, Options, ParameterBody, SchemaBody,
    StoredEntity, apply_intra_links, entity_category_dir,
};

use crate::render::fence::{
    openapi_operation_markdown, proto_file_name, push_fence_body, render_schema_fence,
};
use crate::render::markdown_doc::format_markdown_doc;
use crate::render::operation_fence::render_openapi_operation_fence;
use crate::render::{md_heading, push_paragraph_break};

const SECTION_LEVEL: usize = 2;
const ENTITY_LEVEL: usize = 3;

pub fn is_openrpc_family(family: &str) -> bool {
    family == "openrpc"
}

pub fn category_section_title(category: &str) -> &'static str {
    match category {
        "operation" => "Operations",
        "schema" => "Schemas",
        "parameter" => "Parameters",
        _ => "Entities",
    }
}

pub fn openrpc_summary_link_text(entity: &StoredEntity) -> String {
    match entity.category.as_str() {
        "operation" => entity.title.clone(),
        category => format!("{} {}", summary_prefix(category), entity.name),
    }
}

pub fn openrpc_summary_sort_key(entity: &StoredEntity) -> String {
    match entity.category.as_str() {
        "operation" => entity.title.clone(),
        _ => entity.name.clone(),
    }
}

pub fn summary_prefix(category: &str) -> &'static str {
    match category {
        "operation" => "Operation",
        "schema" => "Schema",
        "parameter" => "Parameter",
        _ => "Entity",
    }
}

pub fn renderable_openrpc_entities(entities: &[StoredEntity]) -> Vec<&StoredEntity> {
    entities
        .iter()
        .filter(|e| {
            matches!(
                e.body,
                EntityBody::Operation(_) | EntityBody::Schema(_) | EntityBody::Parameter(_)
            )
        })
        .collect()
}

pub fn openrpc_index_categories() -> &'static [&'static str] {
    &["operation", "schema", "parameter"]
}

pub(crate) fn openrpc_category_rank(category: &str) -> usize {
    openrpc_index_categories()
        .iter()
        .position(|c| *c == category)
        .unwrap_or(openrpc_index_categories().len())
}

pub fn render_openrpc_index_sections(
    out: &mut String,
    entities: &[StoredEntity],
    group_dir: &str,
    markdown_root: &str,
    index_from: &std::path::Path,
    links: &LinkContext,
) {
    for category in openrpc_index_categories() {
        let mut section: Vec<_> = renderable_openrpc_entities(entities)
            .into_iter()
            .filter(|e| e.category == *category)
            .collect();
        if section.is_empty() {
            continue;
        }
        section.sort_by_key(|a| openrpc_summary_sort_key(a));
        out.push_str(&md_heading(SECTION_LEVEL, category_section_title(category)));
        for entity in section {
            let rel = openrpc_entity_rel_path(markdown_root, group_dir, entity);
            let p = std::path::PathBuf::from(rel);
            let title = openrpc_summary_link_text(entity);
            out.push_str("- ");
            out.push_str(&links.summary_link(index_from, &p, &title));
            out.push('\n');
        }
        out.push('\n');
    }
}

pub fn render_openrpc_package_sections(
    out: &mut String,
    entities: &[StoredEntity],
    group: &str,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) {
    for category in openrpc_index_categories() {
        let mut section: Vec<_> = entities
            .iter()
            .filter(|e| e.category.as_str() == *category)
            .collect();
        if section.is_empty() {
            continue;
        }
        section.sort_by_key(|a| openrpc_summary_sort_key(a));
        out.push_str(&md_heading(SECTION_LEVEL, category_section_title(category)));
        for entity in section {
            render_openrpc_entity_section(out, entity, group, ctx, opts, formatter);
        }
    }
}

fn entity_link_group(entity: &StoredEntity) -> &str {
    entity
        .refs
        .iter()
        .find(|r| !r.target.group.is_empty())
        .map(|r| r.target.group.as_str())
        .unwrap_or("")
}

fn render_openrpc_entity_section(
    out: &mut String,
    entity: &StoredEntity,
    group: &str,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) {
    out.push_str(&md_heading(ENTITY_LEVEL, &openrpc_entity_heading(entity)));
    out.push_str(&render_openrpc_entity_body(
        entity, group, ctx, opts, formatter,
    ));
}

pub fn render_openrpc_entity_page(
    entity: &StoredEntity,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> String {
    let mut page = md_heading(1, &openrpc_entity_heading(entity));
    page.push_str(&render_openrpc_entity_body(
        entity,
        entity_link_group(entity),
        ctx,
        opts,
        formatter,
    ));
    page
}

fn openrpc_entity_heading(entity: &StoredEntity) -> String {
    entity.title.clone()
}

fn render_openrpc_entity_body(
    entity: &StoredEntity,
    group: &str,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> String {
    match &entity.body {
        EntityBody::Operation(body) => {
            render_openrpc_operation_markdown(entity, body, group, ctx, opts, formatter)
        }
        EntityBody::Schema(body) => render_openrpc_schema_fence(entity, body, ctx, opts, formatter),
        EntityBody::Parameter(body) => {
            render_openrpc_parameter_fence(entity, body, ctx, opts, formatter)
        }
        _ => String::new(),
    }
}

fn render_openrpc_operation_markdown(
    entity: &StoredEntity,
    body: &OperationBody,
    group: &str,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> String {
    let mut out = openapi_operation_markdown(entity, body, group, opts.escape_tags, formatter, ctx);
    render_openapi_operation_fence(
        &mut out,
        &body.fence_language,
        &body.fence_body,
        opts.openapi_operation_source,
    );
    out
}

fn render_openrpc_schema_fence(
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

fn render_openrpc_parameter_fence(
    entity: &StoredEntity,
    body: &ParameterBody,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> String {
    let mut out = String::new();
    if let Some(doc) = entity.doc.as_deref() {
        let doc = apply_intra_links("doc", doc, &entity.intra_links, formatter, ctx);
        out.push_str(&format_markdown_doc(&doc, opts.escape_tags));
        push_paragraph_break(&mut out);
    }
    if !body.fence_body.trim().is_empty() {
        push_fence_body(&mut out, &body.fence_language, &body.fence_body);
    }
    out
}

pub fn openrpc_entity_rel_path(
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
