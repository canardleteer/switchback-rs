//! Build [`OperationBody`] values from path and webhook items.

use serde_json::Value;
use switchback_jsonschema::loader::Doc;
use switchback_jsonschema::resolver::resolve_pointer;
use switchback_traits::{
    OperationBody, OperationRequestBodyRef, ParameterRef, RefKind, Reference, ResponseRef,
};

use crate::populate::http_attach;

use crate::populate::PopulateCtx;
use crate::populate::refs::schema_ref_from_value;
use crate::populate::schema_type_label::{parameter_type_label, schema_type_label};

pub fn operation_name(method: &str, path: &str) -> String {
    format!("{} {path}", method.to_ascii_uppercase())
}

pub fn build_operation_body(
    method: &str,
    path: &str,
    op_value: &Value,
    path_item: &Value,
    doc: &Doc,
    ctx: &PopulateCtx<'_>,
) -> OperationBody {
    let (signature, protocols) = http_attach::populate_operation(method, path, op_value);
    let fence_body = serialize_fence(op_value, doc);
    let parameters = collect_parameter_refs(op_value, path_item, ctx);
    let responses = collect_response_refs(op_value, ctx);
    let request_body = collect_request_body_ref(op_value, ctx);

    OperationBody {
        signature,
        fence_language: doc.fence_language().to_string(),
        fence_body,
        parameters,
        responses,
        request_body,
        protocols,
    }
}

fn collect_parameter_refs(
    op_value: &Value,
    path_item: &Value,
    ctx: &PopulateCtx<'_>,
) -> Vec<ParameterRef> {
    let mut out = Vec::new();
    let mut seen = std::collections::BTreeSet::new();

    for source in [path_item, op_value] {
        if let Some(params) = source.get("parameters").and_then(|v| v.as_array()) {
            for param in params {
                push_parameter_ref(param, ctx, &mut out, &mut seen);
            }
        }
    }

    out
}

fn collect_request_body_ref(
    op_value: &Value,
    ctx: &PopulateCtx<'_>,
) -> Option<OperationRequestBodyRef> {
    let body = op_value.get("requestBody")?;
    let resolved = dereference_value(body, ctx);
    let obj = resolved.as_object()?;

    let required = obj
        .get("required")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if let Some(Value::String(ref_key)) = obj.get("$ref") {
        let schema_ref = crate::populate::refs::ref_to_reference(
            ref_key,
            ctx.doc_uri,
            ctx.module_id,
            ctx.uri_to_group,
            ctx.index,
        )?;
        let type_label = ref_key.rsplit('/').next().unwrap_or("body").to_string();
        return Some(OperationRequestBodyRef {
            required,
            media_type: String::new(),
            schema_ref,
            type_label,
        });
    }

    let content = obj.get("content")?.as_object()?;
    let (media_type, media_obj) = content.iter().next()?;
    let schema_val = media_obj.get("schema")?;
    let schema_ref = schema_ref_from_value(
        schema_val,
        ctx.doc_uri,
        ctx.module_id,
        ctx.uri_to_group,
        ctx.index,
    )
    .unwrap_or_else(|| inline_schema_ref(ctx.module_id));
    let type_label = schema_type_label(schema_val);

    Some(OperationRequestBodyRef {
        required,
        media_type: media_type.clone(),
        schema_ref,
        type_label,
    })
}

fn push_parameter_ref(
    param: &Value,
    ctx: &PopulateCtx<'_>,
    out: &mut Vec<ParameterRef>,
    seen: &mut std::collections::BTreeSet<String>,
) {
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

    if !seen.insert(name.clone()) {
        return;
    }

    let location = obj
        .get("in")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
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
        location: location.clone(),
        required,
        schema_ref,
        type_label,
        description,
        protocols: vec![http_attach::parameter_attachment(
            &name, &location, required,
        )],
    });
}

fn collect_response_refs(op_value: &Value, ctx: &PopulateCtx<'_>) -> Vec<ResponseRef> {
    let mut out = Vec::new();
    let Some(responses) = op_value.get("responses").and_then(|v| v.as_object()) else {
        return out;
    };

    for (status, resp) in responses {
        if status.starts_with('x') {
            continue;
        }
        if let Some(Value::String(ref_key)) = resp.get("$ref") {
            if let Some(reference) = crate::populate::refs::ref_to_reference(
                ref_key,
                ctx.doc_uri,
                ctx.module_id,
                ctx.uri_to_group,
                ctx.index,
            ) {
                out.push(http_attach::response_ref(status, "", "", reference));
            }
            continue;
        }
        let description = resp
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let media_type = resp
            .get("content")
            .and_then(|c| c.as_object())
            .and_then(|m| m.keys().next())
            .cloned()
            .unwrap_or_default();
        let schema_val = resp
            .get("content")
            .and_then(|c| c.as_object())
            .and_then(|m| m.values().next())
            .and_then(|mt| mt.get("schema"));
        let schema_ref = schema_val
            .and_then(|s| {
                schema_ref_from_value(s, ctx.doc_uri, ctx.module_id, ctx.uri_to_group, ctx.index)
            })
            .unwrap_or_else(|| placeholder_ref(ctx.module_id, status));
        out.push(http_attach::response_ref(
            status,
            &description,
            &media_type,
            schema_ref,
        ));
    }
    out
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

fn placeholder_ref(module_id: &str, name: &str) -> switchback_traits::Reference {
    switchback_traits::Reference {
        target: switchback_traits::EntityRef {
            module: module_id.to_string(),
            group: crate::paths::COMPONENTS_GROUP.to_string(),
            category: "schema".to_string(),
            name: name.to_string(),
        },
        kind: switchback_traits::RefKind::Internal,
    }
}

fn serialize_fence(value: &Value, doc: &Doc) -> String {
    if doc.is_yaml {
        serde_saphyr::to_string(value).unwrap_or_else(|_| "{}".into())
    } else {
        serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".into())
    }
}
