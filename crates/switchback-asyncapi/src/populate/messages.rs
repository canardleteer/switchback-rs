//! Populate message entities from components and inline payloads.

use serde_json::Value;
use switchback_traits::{Entity, EntityBody, EntityCategory, EntityId, MessageBody};

use crate::category::AsyncApiCategory;
use crate::paths::COMPONENTS_GROUP;
use crate::populate::PopulateCtx;
use crate::populate::PopulatedEntity;
use crate::populate::bindings::append_generic_bindings;
use crate::populate::refs::structural_refs;

fn entity_group<'a>(ctx: &'a PopulateCtx<'a>) -> &'a str {
    ctx.entry_group.unwrap_or(COMPONENTS_GROUP)
}

pub fn populate_component_messages(
    root: &Value,
    ctx: &PopulateCtx<'_>,
    out: &mut Vec<PopulatedEntity>,
) {
    let Some(messages) = root
        .get("components")
        .and_then(|c| c.get("messages"))
        .and_then(|v| v.as_object())
    else {
        return;
    };

    for (name, msg_val) in messages {
        push_message_entity(msg_val, name, ctx, out);
    }
}

pub fn push_message_entity(
    value: &Value,
    name: &str,
    ctx: &PopulateCtx<'_>,
    out: &mut Vec<PopulatedEntity>,
) {
    let group = entity_group(ctx);
    let mut fence_body = serialize_fence(value, ctx.doc.fence_language());
    fence_body = append_generic_bindings(
        &fence_body,
        value.get("bindings"),
        ctx.doc.fence_language(),
    );
    let doc_text = value
        .get("description")
        .or_else(|| value.get("summary"))
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .filter(|s| !s.is_empty());
    let refs = structural_refs(
        value,
        ctx.doc_uri,
        ctx.module_id,
        ctx.uri_to_group,
        ctx.index,
    );
    out.push(PopulatedEntity {
        entity: Entity {
            id: EntityId::new(group, AsyncApiCategory::Message.as_str(), name),
            category: AsyncApiCategory::Message,
            title: value
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or(name)
                .to_string(),
            doc: doc_text,
            source_span: None,
            body: EntityBody::Message(MessageBody {
                fence_language: ctx.doc.fence_language().to_string(),
                fence_body,
            }),
        },
        refs,
    });
}

fn serialize_fence(value: &Value, fence_language: &str) -> String {
    if fence_language == "yaml" {
        serde_saphyr::to_string(value).unwrap_or_else(|_| "{}".into())
    } else {
        serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".into())
    }
}

fn description(value: &Value) -> Option<String> {
    value
        .get("description")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .filter(|s| !s.is_empty())
}
