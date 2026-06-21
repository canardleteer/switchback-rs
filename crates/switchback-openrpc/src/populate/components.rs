//! Populate reusable component entities.

use serde_json::Value;
use switchback_jsonschema::schema::populate_schema_body;
use switchback_traits::{Entity, EntityBody, EntityCategory, EntityId, ParameterBody};

use crate::category::OpenRpcCategory;
use crate::paths::COMPONENTS_GROUP;
use crate::populate::PopulateCtx;
use crate::populate::PopulatedEntity;
use crate::populate::refs::structural_refs;

fn entity_group<'a>(ctx: &'a PopulateCtx<'a>) -> &'a str {
    ctx.entry_group.unwrap_or(COMPONENTS_GROUP)
}

pub fn populate_components(root: &Value, ctx: &PopulateCtx<'_>, out: &mut Vec<PopulatedEntity>) {
    let Some(components) = root.get("components").and_then(|v| v.as_object()) else {
        return;
    };

    if let Some(schemas) = components.get("schemas").and_then(|v| v.as_object()) {
        for (name, schema_val) in schemas {
            push_schema_entity(schema_val, name, ctx, out);
        }
    }

    if let Some(descriptors) = components
        .get("contentDescriptors")
        .and_then(|v| v.as_object())
    {
        for (name, descriptor_val) in descriptors {
            push_content_descriptor_entity(descriptor_val, name, ctx, out);
        }
    }
}

fn resolve_group_for_ref(
    ctx: &PopulateCtx<'_>,
    _base_uri: &str,
    file_part: &str,
) -> Option<String> {
    if file_part.is_empty() {
        return Some(entity_group(ctx).to_string());
    }
    let file_part = switchback_jsonschema::paths::strip_dot_slash(file_part);
    let candidate = std::path::Path::new(ctx.doc_uri)
        .parent()
        .unwrap_or(std::path::Path::new(""))
        .join(file_part);
    let normalized = candidate.to_string_lossy().replace('\\', "/");
    ctx.uri_to_group
        .get(&normalized)
        .cloned()
        .or_else(|| Some(entity_group(ctx).to_string()))
}

fn push_schema_entity(
    value: &Value,
    name: &str,
    ctx: &PopulateCtx<'_>,
    out: &mut Vec<PopulatedEntity>,
) {
    let body = populate_schema_body(
        value,
        ctx.doc,
        ctx.module_id,
        entity_group(ctx),
        &|base_uri, file_part| resolve_group_for_ref(ctx, base_uri, file_part),
    );
    let doc_text = description(value);
    let refs = structural_refs(
        value,
        ctx.doc_uri,
        ctx.module_id,
        ctx.uri_to_group,
        ctx.index,
    );
    out.push(PopulatedEntity {
        entity: Entity {
            id: EntityId::new(entity_group(ctx), OpenRpcCategory::Schema.as_str(), name),
            category: OpenRpcCategory::Schema,
            title: name.to_string(),
            doc: doc_text,
            source_span: None,
            body: EntityBody::Schema(body),
        },
        refs,
    });
}

fn push_content_descriptor_entity(
    value: &Value,
    name: &str,
    ctx: &PopulateCtx<'_>,
    out: &mut Vec<PopulatedEntity>,
) {
    let obj = value.as_object();
    let required = obj
        .and_then(|o| o.get("required"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let schema_val = obj.and_then(|o| o.get("schema")).unwrap_or(value);
    let fence = populate_schema_body(
        schema_val,
        ctx.doc,
        ctx.module_id,
        entity_group(ctx),
        &|base_uri, file_part| resolve_group_for_ref(ctx, base_uri, file_part),
    );
    let refs = structural_refs(
        value,
        ctx.doc_uri,
        ctx.module_id,
        ctx.uri_to_group,
        ctx.index,
    );
    out.push(PopulatedEntity {
        entity: Entity {
            id: EntityId::new(entity_group(ctx), OpenRpcCategory::Parameter.as_str(), name),
            category: OpenRpcCategory::Parameter,
            title: name.to_string(),
            doc: description(value),
            source_span: None,
            body: EntityBody::Parameter(ParameterBody {
                name: name.to_string(),
                location: "param".to_string(),
                required,
                fence_language: ctx.doc.fence_language().to_string(),
                fence_body: fence.fence_body,
                protocols: Vec::new(),
            }),
        },
        refs,
    });
}

fn description(value: &Value) -> Option<String> {
    value
        .get("description")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .filter(|s| !s.is_empty())
}
