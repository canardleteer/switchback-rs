//! Build [`OperationBody`] values from OpenRPC methods.

use serde_json::Value;
use switchback_jsonschema::loader::Doc;
use switchback_jsonschema::resolver::resolve_pointer;
use switchback_traits::{
    Entity, EntityBody, EntityCategory, EntityId, OperationBody, ParameterRef, RefKind, Reference,
    ResponseRef, ResponseSeverity,
};

use crate::category::OpenRpcCategory;
use crate::populate::PopulateCtx;
use crate::populate::PopulatedEntity;
use crate::populate::groups::merge_method_tags;
use crate::populate::refs::{schema_ref_from_value, structural_refs};
use crate::populate::schema_type_label::{parameter_type_label, schema_type_label};

pub fn build_operation_body(
    method_name: &str,
    method_value: &Value,
    doc: &Doc,
    ctx: &PopulateCtx<'_>,
) -> OperationBody {
    let parameters = collect_parameter_refs(method_value, ctx);
    let result_type = result_type_label(method_value, ctx);
    let signature = format!(
        "**{method_name}** ({}) -> {result_type}",
        parameters
            .iter()
            .map(|p| p.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );
    let responses = collect_result_refs(method_value, ctx);
    let fence_body = serialize_fence(method_value, doc);

    OperationBody {
        signature,
        fence_language: doc.fence_language().to_string(),
        fence_body,
        parameters,
        responses,
        request_body: None,
        protocols: Vec::new(),
    }
}

pub fn populate_methods(
    root: &Value,
    ctx: &PopulateCtx<'_>,
    entities_by_group: &mut std::collections::BTreeMap<String, Vec<PopulatedEntity>>,
) {
    let Some(methods) = root.get("methods").and_then(|v| v.as_array()) else {
        return;
    };

    for method in methods {
        let Some(method_name) = method.get("name").and_then(|v| v.as_str()) else {
            continue;
        };
        let method_tags: Vec<String> = method
            .get("tags")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|t| t.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        let target_groups = if let Some(group) = ctx.entry_group {
            vec![group.to_string()]
        } else {
            merge_method_tags(root, &method_tags)
        };

        let title = method
            .get("summary")
            .and_then(|v| v.as_str())
            .unwrap_or(method_name)
            .to_string();
        let body = build_operation_body(method_name, method, ctx.doc, ctx);
        let doc_text = method
            .get("description")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .filter(|s| !s.is_empty());
        let refs = structural_refs(
            method,
            ctx.doc_uri,
            ctx.module_id,
            ctx.uri_to_group,
            ctx.index,
        );

        for group_id in target_groups {
            let group_key = crate::paths::slugify(&group_id);
            let pe = PopulatedEntity {
                entity: Entity {
                    id: EntityId::new(
                        group_key.as_str(),
                        OpenRpcCategory::Operation.as_str(),
                        method_name,
                    ),
                    category: OpenRpcCategory::Operation,
                    title: title.clone(),
                    doc: doc_text.clone(),
                    source_span: None,
                    body: EntityBody::Operation(body.clone()),
                },
                refs: refs.clone(),
            };
            entities_by_group.entry(group_key).or_default().push(pe);
        }
    }
}

fn collect_parameter_refs(method: &Value, ctx: &PopulateCtx<'_>) -> Vec<ParameterRef> {
    let mut out = Vec::new();
    let Some(params) = method.get("params").and_then(|v| v.as_array()) else {
        return out;
    };

    for param in params {
        push_parameter_ref(param, ctx, &mut out);
    }
    out
}

fn push_parameter_ref(param: &Value, ctx: &PopulateCtx<'_>, out: &mut Vec<ParameterRef>) {
    let component_key = param
        .get("$ref")
        .and_then(|v| v.as_str())
        .and_then(|ref_key| ref_key.rsplit('/').next().map(str::to_string));

    let resolved = dereference_value(param, ctx);
    let Some(obj) = resolved.as_object() else {
        return;
    };

    let name = component_key.clone().unwrap_or_else(|| {
        obj.get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("parameter")
            .to_string()
    });
    let required = obj
        .get("required")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let description = obj
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let schema_val = obj.get("schema").unwrap_or(resolved);
    let type_label = parameter_type_label(resolved, schema_val);
    let schema_ref = schema_ref_from_value(
        schema_val,
        ctx.doc_uri,
        ctx.module_id,
        ctx.uri_to_group,
        ctx.index,
    )
    .unwrap_or_else(|| inline_schema_ref(ctx.module_id));

    out.push(ParameterRef {
        name: name.clone(),
        location: "param".to_string(),
        required,
        schema_ref,
        type_label,
        description,
        protocols: Vec::new(),
    });
}

fn collect_result_refs(method: &Value, ctx: &PopulateCtx<'_>) -> Vec<ResponseRef> {
    let Some(result) = method.get("result") else {
        return Vec::new();
    };
    let resolved = dereference_value(result, ctx);
    let result_name = resolved
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("result")
        .to_string();
    let description = resolved
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let schema_val = resolved.get("schema").unwrap_or(resolved);
    let schema_ref = schema_ref_from_value(
        schema_val,
        ctx.doc_uri,
        ctx.module_id,
        ctx.uri_to_group,
        ctx.index,
    )
    .unwrap_or_else(|| inline_schema_ref(ctx.module_id));

    vec![ResponseRef {
        status: result_name,
        severity: ResponseSeverity::Unspecified,
        schema_ref,
        media_type: String::new(),
        description,
        protocols: Vec::new(),
    }]
}

fn result_type_label(method: &Value, ctx: &PopulateCtx<'_>) -> String {
    let Some(result) = method.get("result") else {
        return "void".to_string();
    };
    let resolved = dereference_value(result, ctx);
    let schema_val = resolved.get("schema").unwrap_or(resolved);
    let label = schema_type_label(schema_val);
    if label.is_empty() {
        "void".to_string()
    } else {
        label
    }
}

fn dereference_value<'a>(value: &'a Value, ctx: &PopulateCtx<'a>) -> &'a Value {
    if let Some(Value::String(ref_key)) = value.get("$ref")
        && let Some(pointer) = json_pointer_from_ref(ref_key)
        && let Some(resolved) = resolve_pointer(&ctx.doc.value, pointer)
    {
        return resolved;
    }
    value
}

fn json_pointer_from_ref(ref_key: &str) -> Option<&str> {
    let pointer = if let Some((_, ptr)) = ref_key.split_once('#') {
        ptr.trim_start_matches('/')
    } else if ref_key.starts_with('#') {
        ref_key.trim_start_matches('/')
    } else {
        return None;
    };
    if pointer.is_empty() {
        None
    } else {
        Some(pointer)
    }
}

fn inline_schema_ref(module_id: &str) -> Reference {
    Reference {
        target: switchback_traits::EntityRef {
            module: module_id.to_string(),
            group: String::new(),
            category: String::new(),
            name: String::new(),
        },
        kind: RefKind::Inline,
    }
}
fn serialize_fence(value: &Value, doc: &Doc) -> String {
    if doc.is_yaml {
        serde_saphyr::to_string(value).unwrap_or_else(|_| "{}".into())
    } else {
        serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".into())
    }
}
