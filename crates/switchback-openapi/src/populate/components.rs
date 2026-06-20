//! Populate reusable component entities.

use serde_json::Value;
use switchback_jsonschema::loader::Doc;
use switchback_jsonschema::schema::populate_schema_body;
use switchback_traits::{
    Entity, EntityBody, EntityCategory, EntityId, ParameterBody, RequestBodyBody, ResponseBody,
    SecuritySchemeBody,
};

use crate::category::OpenApiCategory;
use crate::paths::COMPONENTS_GROUP;
use crate::populate::http_attach;
use crate::populate::refs::structural_refs;
use crate::populate::PopulateCtx;
use crate::populate::PopulatedEntity;

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

    if let Some(responses) = components.get("responses").and_then(|v| v.as_object()) {
        for (name, resp_val) in responses {
            push_response_entity(resp_val, name, ctx, out);
        }
    }

    if let Some(bodies) = components.get("requestBodies").and_then(|v| v.as_object()) {
        for (name, body_val) in bodies {
            push_request_body_entity(body_val, name, ctx, out);
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
}

fn resolve_group_for_ref(
    ctx: &PopulateCtx<'_>,
    _base_uri: &str,
    file_part: &str,
) -> Option<String> {
    if file_part.is_empty() {
        return Some(COMPONENTS_GROUP.to_string());
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
        .or_else(|| Some(COMPONENTS_GROUP.to_string()))
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
        COMPONENTS_GROUP,
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
            id: EntityId::new(COMPONENTS_GROUP, OpenApiCategory::Schema.as_str(), name),
            category: OpenApiCategory::Schema,
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
    let obj = value.as_object();
    let location = obj
        .and_then(|o| o.get("in"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let required = obj
        .and_then(|o| o.get("required"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let schema_val = obj.and_then(|o| o.get("schema")).unwrap_or(value);
    let fence = populate_schema_body(
        schema_val,
        ctx.doc,
        ctx.module_id,
        COMPONENTS_GROUP,
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
            id: EntityId::new(COMPONENTS_GROUP, OpenApiCategory::Parameter.as_str(), name),
            category: OpenApiCategory::Parameter,
            title: name.to_string(),
            doc: description(value),
            source_span: None,
            body: EntityBody::Parameter(ParameterBody {
                name: name.to_string(),
                location: location.clone(),
                required,
                fence_language: ctx.doc.fence_language().to_string(),
                fence_body: fence.fence_body,
                protocols: http_attach::parameter_body_protocols(name, &location, required),
            }),
        },
        refs,
    });
}

fn push_response_entity(
    value: &Value,
    name: &str,
    ctx: &PopulateCtx<'_>,
    out: &mut Vec<PopulatedEntity>,
) {
    let (status, media_type, fence_body) = response_fence(value, ctx.doc);
    let desc_text = description(value).unwrap_or_default();
    let (severity, protocols) =
        http_attach::response_body_protocols(&status, &desc_text, &media_type);
    let refs = structural_refs(
        value,
        ctx.doc_uri,
        ctx.module_id,
        ctx.uri_to_group,
        ctx.index,
    );
    out.push(PopulatedEntity {
        entity: Entity {
            id: EntityId::new(COMPONENTS_GROUP, OpenApiCategory::Response.as_str(), name),
            category: OpenApiCategory::Response,
            title: name.to_string(),
            doc: description(value),
            source_span: None,
            body: EntityBody::Response(ResponseBody {
                status,
                severity,
                media_type,
                fence_language: ctx.doc.fence_language().to_string(),
                fence_body,
                protocols,
            }),
        },
        refs,
    });
}

fn push_request_body_entity(
    value: &Value,
    name: &str,
    ctx: &PopulateCtx<'_>,
    out: &mut Vec<PopulatedEntity>,
) {
    let required = value
        .get("required")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let fence_body = serialize_fence(value, ctx.doc);
    let refs = structural_refs(
        value,
        ctx.doc_uri,
        ctx.module_id,
        ctx.uri_to_group,
        ctx.index,
    );
    out.push(PopulatedEntity {
        entity: Entity {
            id: EntityId::new(
                COMPONENTS_GROUP,
                OpenApiCategory::RequestBody.as_str(),
                name,
            ),
            category: OpenApiCategory::RequestBody,
            title: name.to_string(),
            doc: description(value),
            source_span: None,
            body: EntityBody::RequestBody(RequestBodyBody {
                required,
                fence_language: ctx.doc.fence_language().to_string(),
                fence_body,
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
    let scheme_type = value
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let refs = structural_refs(
        value,
        ctx.doc_uri,
        ctx.module_id,
        ctx.uri_to_group,
        ctx.index,
    );
    out.push(PopulatedEntity {
        entity: Entity {
            id: EntityId::new(
                COMPONENTS_GROUP,
                OpenApiCategory::SecurityScheme.as_str(),
                name,
            ),
            category: OpenApiCategory::SecurityScheme,
            title: name.to_string(),
            doc: description(value),
            source_span: None,
            body: EntityBody::SecurityScheme(SecuritySchemeBody {
                scheme_type,
                fence_language: ctx.doc.fence_language().to_string(),
                fence_body: serialize_fence(value, ctx.doc),
            }),
        },
        refs,
    });
}

fn response_fence(value: &Value, doc: &Doc) -> (String, String, String) {
    let status = value
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let media_type = value
        .get("content")
        .and_then(|c| c.as_object())
        .and_then(|m| m.keys().next())
        .cloned()
        .unwrap_or_default();
    (status, media_type, serialize_fence(value, doc))
}

fn description(value: &Value) -> Option<String> {
    value
        .get("description")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .filter(|s| !s.is_empty())
}

fn serialize_fence(value: &Value, doc: &Doc) -> String {
    if doc.is_yaml {
        serde_saphyr::to_string(value).unwrap_or_else(|_| "{}".into())
    } else {
        serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".into())
    }
}
