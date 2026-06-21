//! Populate reusable component entities.

use serde_json::Value;
use switchback_traits::{
    Entity, EntityBody, EntityCategory, EntityId, ParameterBody, SecuritySchemeBody,
};

use crate::category::AsyncApiCategory;
use crate::paths::COMPONENTS_GROUP;
use crate::populate::PopulateCtx;
use crate::populate::PopulatedEntity;
use crate::populate::refs::structural_refs;
use crate::populate::schema_dispatch::populate_schema_dispatch;

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

    if let Some(params) = components.get("parameters").and_then(|v| v.as_object()) {
        for (name, param_val) in params {
            push_parameter_entity(param_val, name, ctx, out);
        }
    }

    if let Some(schemes) = components
        .get("securitySchemes")
        .and_then(|v| v.as_object())
    {
        for (name, scheme_val) in schemes {
            push_security_scheme_entity(scheme_val, name, ctx, out);
        }
    }

    crate::populate::messages::populate_component_messages(root, ctx, out);
}

fn push_schema_entity(
    value: &Value,
    name: &str,
    ctx: &PopulateCtx<'_>,
    out: &mut Vec<PopulatedEntity>,
) {
    let group = entity_group(ctx);
    let body = populate_schema_dispatch(value, ctx.doc, ctx, group);
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
            id: EntityId::new(group, AsyncApiCategory::Schema.as_str(), name),
            category: AsyncApiCategory::Schema,
            title: name.to_string(),
            doc: doc_text,
            source_span: None,
            body: EntityBody::Schema(body),
        },
        refs,
    });
}

fn push_parameter_entity(
    value: &Value,
    name: &str,
    ctx: &PopulateCtx<'_>,
    out: &mut Vec<PopulatedEntity>,
) {
    let group = entity_group(ctx);
    let obj = value.as_object();
    let location = obj
        .and_then(|o| o.get("location"))
        .or_else(|| obj.and_then(|o| o.get("in")))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let required = obj
        .and_then(|o| o.get("required"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let schema_val = obj.and_then(|o| o.get("schema")).unwrap_or(value);
    let fence = populate_schema_dispatch(schema_val, ctx.doc, ctx, group);
    let refs = structural_refs(
        value,
        ctx.doc_uri,
        ctx.module_id,
        ctx.uri_to_group,
        ctx.index,
    );
    out.push(PopulatedEntity {
        entity: Entity {
            id: EntityId::new(group, AsyncApiCategory::Parameter.as_str(), name),
            category: AsyncApiCategory::Parameter,
            title: name.to_string(),
            doc: description(value),
            source_span: None,
            body: EntityBody::Parameter(ParameterBody {
                name: name.to_string(),
                location,
                required,
                fence_language: fence.fence_language.clone(),
                fence_body: fence.fence_body,
                protocols: Vec::new(),
            }),
        },
        refs,
    });
}

fn push_security_scheme_entity(
    value: &Value,
    name: &str,
    ctx: &PopulateCtx<'_>,
    out: &mut Vec<PopulatedEntity>,
) {
    let group = entity_group(ctx);
    let scheme_type = value
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let fence_body = if ctx.doc.fence_language() == "yaml" {
        serde_saphyr::to_string(value).unwrap_or_else(|_| "{}".into())
    } else {
        serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".into())
    };
    let refs = structural_refs(
        value,
        ctx.doc_uri,
        ctx.module_id,
        ctx.uri_to_group,
        ctx.index,
    );
    out.push(PopulatedEntity {
        entity: Entity {
            id: EntityId::new(group, AsyncApiCategory::SecurityScheme.as_str(), name),
            category: AsyncApiCategory::SecurityScheme,
            title: name.to_string(),
            doc: description(value),
            source_span: None,
            body: EntityBody::SecurityScheme(SecuritySchemeBody {
                scheme_type,
                fence_language: ctx.doc.fence_language().to_string(),
                fence_body,
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
