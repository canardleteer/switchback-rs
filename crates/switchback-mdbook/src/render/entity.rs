//! Per-entity pages (`layout=entity` and `layout=split`).

use switchback_traits::{
    EntityBody, Group, LinkContext, LinkFormatter, Options, ProtobufEntityKind, StoredEntity,
    apply_intra_links,
};

use crate::render::asyncapi::{
    asyncapi_entity_rel_path, is_asyncapi_family, render_asyncapi_entity_page,
    render_asyncapi_index_sections, renderable_asyncapi_entities,
};
use crate::render::fence::{
    entity_module_group, link_structural_refs_in_prose, operation_signature_markdown,
    proto_file_name, push_proto_fence_body, render_proto_fence,
};
use crate::render::markdown_doc::format_markdown_doc;
use crate::render::openapi::{
    is_openapi_family, openapi_entity_rel_path, render_openapi_entity_page,
    render_openapi_index_sections, renderable_openapi_entities,
};
use crate::render::{md_heading, push_paragraph_break};

pub fn render_entity_pages(
    group: &Group,
    entities: &[StoredEntity],
    family: &str,
    links: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> Vec<(String, String)> {
    if is_openapi_family(family) {
        return render_openapi_entity_pages(group, entities, links, opts, formatter);
    }

    if is_asyncapi_family(family) {
        return render_asyncapi_entity_pages(group, entities, links, opts, formatter);
    }

    let package = group.id.as_str();
    let mut pages = Vec::new();

    {
        let index_rel = links.package_index_rel(package);
        let index_path = opts.output_path(index_rel.to_string_lossy().as_ref());
        let mut index = md_heading(1, package);
        if group.overview.as_ref().is_some_and(|o| !o.is_empty()) {
            index.push_str(&format_markdown_doc(
                group.overview.as_deref().unwrap(),
                opts.escape_tags,
            ));
            push_paragraph_break(&mut index);
        }
        index.push_str(&md_heading(2, "Contents"));
        let index_from = index_rel.as_path();
        for entity in layout_summary_entities(entities) {
            if let Some(kind) = entity_kind(entity) {
                let p = links
                    .layout_entity_path(package, kind, &entity.name)
                    .expect("entity path");
                index.push_str("- ");
                index.push_str(&links.summary_link(index_from, p, &entity.name));
                index.push('\n');
            }
        }
        pages.push((index_path, index));
    }

    for entity in entities {
        let Some(kind) = entity_kind(entity) else {
            continue;
        };
        if matches!(entity.body, EntityBody::Operation(_)) {
            continue;
        }
        let rel = links
            .layout_entity_path(package, kind, &entity.name)
            .expect("entity path");
        let path = opts.output_path(rel.to_string_lossy().as_ref());
        let mut ctx = links.clone();
        ctx.render_from = Some(rel.clone());
        let page = render_entity_page(entity, entities, &ctx, opts, formatter);
        pages.push((path, page));
    }

    pages
}

fn render_openapi_entity_pages(
    group: &Group,
    entities: &[StoredEntity],
    links: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> Vec<(String, String)> {
    let package = group.id.as_str();
    let mut pages = Vec::new();

    {
        let index_rel = links.package_index_rel(package);
        let index_path = opts.output_path(index_rel.to_string_lossy().as_ref());
        let mut index = md_heading(1, &group.title);
        if group.overview.as_ref().is_some_and(|o| !o.is_empty()) {
            index.push_str(&format_markdown_doc(
                group.overview.as_deref().unwrap(),
                opts.escape_tags,
            ));
            push_paragraph_break(&mut index);
        }
        render_openapi_index_sections(
            &mut index,
            entities,
            &group.dir,
            &opts.markdown_root,
            index_rel.as_path(),
            links,
            opts.openapi_summary_label,
        );
        pages.push((index_path, index));
    }

    for entity in renderable_openapi_entities(entities) {
        let rel = openapi_entity_rel_path(&opts.markdown_root, &group.dir, entity);
        let path = opts.output_path(&rel);
        let mut ctx = links.clone();
        ctx.render_from = Some(std::path::PathBuf::from(rel));
        let page = render_openapi_entity_page(entity, &ctx, opts, formatter);
        pages.push((path, page));
    }

    pages
}

fn render_asyncapi_entity_pages(
    group: &Group,
    entities: &[StoredEntity],
    links: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> Vec<(String, String)> {
    let mut pages = Vec::new();

    {
        let index_rel = links.package_index_rel(group.id.as_str());
        let index_path = opts.output_path(index_rel.to_string_lossy().as_ref());
        let mut index = md_heading(1, &group.title);
        if group.overview.as_ref().is_some_and(|o| !o.is_empty()) {
            index.push_str(&format_markdown_doc(
                group.overview.as_deref().unwrap(),
                opts.escape_tags,
            ));
            push_paragraph_break(&mut index);
        }
        render_asyncapi_index_sections(
            &mut index,
            entities,
            &group.dir,
            &opts.markdown_root,
            index_rel.as_path(),
            links,
        );
        pages.push((index_path, index));
    }

    for entity in renderable_asyncapi_entities(entities) {
        let rel = asyncapi_entity_rel_path(&opts.markdown_root, &group.dir, entity);
        let path = opts.output_path(&rel);
        let mut ctx = links.clone();
        ctx.render_from = Some(std::path::PathBuf::from(rel));
        let page = render_asyncapi_entity_page(entity, &ctx, opts, formatter);
        pages.push((path, page));
    }

    pages
}

fn render_entity_page(
    entity: &StoredEntity,
    entities: &[StoredEntity],
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> String {
    match &entity.body {
        EntityBody::Schema(body) => {
            let mut page = md_heading(1, &entity.title);
            page.push_str(&render_proto_fence(
                &proto_file_name(entity),
                entity.doc.as_deref(),
                &body.fence_body,
                opts.escape_tags,
                &entity.intra_links,
                formatter,
                ctx,
            ));
            page
        }
        EntityBody::Service(_) => {
            let mut page = md_heading(1, &entity.title);
            let file_name = proto_file_name(entity);
            if !file_name.is_empty() {
                page.push_str(&format!("*`{file_name}`*\n\n"));
            }
            if let Some(doc) = entity.doc.as_deref() {
                let doc = apply_intra_links("doc", doc, &entity.intra_links, formatter, ctx);
                page.push_str(&format_markdown_doc(&doc, opts.escape_tags));
                push_paragraph_break(&mut page);
            }
            let prefix = format!("{}.", entity.title);
            for op in entities.iter().filter(|e| {
                matches!(e.body, EntityBody::Operation(_)) && e.name.starts_with(&prefix)
            }) {
                page.push_str(&render_operation_block(op, ctx, opts, formatter));
            }
            page
        }
        _ => md_heading(1, &entity.title),
    }
}

fn render_operation_block(
    op: &StoredEntity,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> String {
    let EntityBody::Operation(body) = &op.body else {
        return String::new();
    };
    let mut out = operation_signature_markdown(&op.title, &body.signature, &op.refs, ctx);
    out.push_str("\n\n");
    let metadata: Vec<_> = body
        .parameters
        .iter()
        .filter(|param| param.location == "metadata")
        .collect();
    if !metadata.is_empty() {
        out.push_str(&md_heading(4, "Metadata"));
        out.push_str("| Key | Required |\n");
        out.push_str("| --- | --- |\n");
        for param in metadata {
            out.push('|');
            out.push(' ');
            out.push('`');
            out.push_str(&param.name);
            out.push('`');
            out.push_str(" | ");
            out.push_str(if param.required {
                "required"
            } else {
                "optional"
            });
            out.push_str(" |\n");
        }
        out.push('\n');
    }
    if !body.fence_body.trim().is_empty() {
        push_proto_fence_body(&mut out, &body.fence_body);
    }
    if let Some(doc) = op.doc.as_deref() {
        let doc = apply_intra_links("doc", doc, &op.intra_links, formatter, ctx);
        let from = ctx
            .render_from
            .as_deref()
            .unwrap_or_else(|| std::path::Path::new(&ctx.markdown_root));
        let (module, group) = entity_module_group(op);
        let doc = link_structural_refs_in_prose(&doc, &op.refs, module, group, ctx, from);
        out.push_str(&format_markdown_doc(&doc, opts.escape_tags));
        push_paragraph_break(&mut out);
    }
    out
}

fn layout_summary_entities(entities: &[StoredEntity]) -> Vec<&StoredEntity> {
    entities
        .iter()
        .filter(|e| entity_kind(e).is_some())
        .collect()
}

fn entity_kind(entity: &StoredEntity) -> Option<ProtobufEntityKind> {
    match &entity.body {
        EntityBody::Service(_) => Some(ProtobufEntityKind::Service),
        EntityBody::Schema(body) => {
            if body.fence_body.starts_with("enum ") {
                Some(ProtobufEntityKind::Enum)
            } else {
                Some(ProtobufEntityKind::Message)
            }
        }
        EntityBody::Operation(_) => None,
        _ => None,
    }
}
