//! Build [`OperationBody`] values from AsyncAPI operations.

use std::collections::BTreeMap;

use serde_json::Value;
use switchback_traits::{Entity, EntityBody, EntityCategory, EntityId, OperationBody};

use crate::category::AsyncApiCategory;
use crate::paths::slugify;
use crate::populate::PopulateCtx;
use crate::populate::PopulatedEntity;
use crate::populate::bindings::append_generic_bindings;
use crate::populate::groups::merge_entity_tags;
use crate::populate::mermaid::{merge_doc_with_mermaid, operation_sequence_diagram};
use crate::populate::protocol_attach;
use crate::populate::refs::structural_refs;

pub fn operation_name(action: &str, channel_or_id: &str) -> String {
    format!("{} {channel_or_id}", action.to_ascii_uppercase())
}

pub fn populate_operations_3x(
    root: &Value,
    ctx: &PopulateCtx<'_>,
    entities_by_group: &mut BTreeMap<String, Vec<PopulatedEntity>>,
) {
    let Some(ops) = root.get("operations").and_then(|v| v.as_object()) else {
        return;
    };

    for (op_id, op_val) in ops {
        let channel_ref = op_val
            .get("channel")
            .and_then(|c| c.get("$ref"))
            .and_then(|v| v.as_str())
            .and_then(|r| r.rsplit('/').next())
            .unwrap_or(op_id);
        let action = op_val
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("send");
        push_operation_entity(
            channel_ref,
            action,
            op_val,
            op_val.get("bindings"),
            ctx,
            entities_by_group,
            Some(op_id),
        );
    }
}

pub fn push_operation_entity(
    channel_name: &str,
    action: &str,
    op_value: &Value,
    channel_bindings: Option<&Value>,
    ctx: &PopulateCtx<'_>,
    entities_by_group: &mut BTreeMap<String, Vec<PopulatedEntity>>,
    operation_id_override: Option<&str>,
) {
    let op_id = operation_id_override
        .map(str::to_string)
        .or_else(|| {
            op_value
                .get("operationId")
                .and_then(|v| v.as_str())
                .map(str::to_string)
        })
        .unwrap_or_else(|| format!("{action}-{channel_name}"));

    let tags: Vec<String> = op_value
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
        merge_entity_tags(&tags)
    };

    let name = operation_name(action, channel_name);
    let title = op_value
        .get("summary")
        .or_else(|| op_value.get("operationId"))
        .and_then(|v| v.as_str())
        .unwrap_or(&name)
        .to_string();

    let bindings = op_value.get("bindings").or(channel_bindings);
    let mut protocols = bindings
        .map(protocol_attach::bindings_protocols)
        .unwrap_or_default();
    if protocols.is_empty()
        && let Some(channel_bindings) = channel_bindings
    {
        protocols = protocol_attach::bindings_protocols(channel_bindings);
    }

    let mut fence_body = serialize_fence(op_value, ctx.doc.fence_language());
    fence_body = append_generic_bindings(&fence_body, bindings, ctx.doc.fence_language());

    let mermaid = operation_sequence_diagram(channel_name, action, &op_id);
    let doc_text = merge_doc_with_mermaid(
        op_value
            .get("description")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .filter(|s| !s.is_empty()),
        &mermaid,
    );

    let refs = structural_refs(
        op_value,
        ctx.doc_uri,
        ctx.module_id,
        ctx.uri_to_group,
        ctx.index,
    );

    let body = OperationBody {
        signature: name.clone(),
        fence_language: ctx.doc.fence_language().to_string(),
        fence_body,
        parameters: Vec::new(),
        responses: Vec::new(),
        request_body: None,
        protocols,
    };

    for group_id in target_groups {
        let group_key = slugify(&group_id);
        let pe = PopulatedEntity {
            entity: Entity {
                id: EntityId::new(
                    group_key.as_str(),
                    AsyncApiCategory::Operation.as_str(),
                    &op_id,
                ),
                category: AsyncApiCategory::Operation,
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

fn serialize_fence(value: &Value, fence_language: &str) -> String {
    if fence_language == "yaml" {
        serde_saphyr::to_string(value).unwrap_or_else(|_| "{}".into())
    } else {
        serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".into())
    }
}
