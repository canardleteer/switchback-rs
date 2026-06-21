//! Populate channel entities.

use std::collections::BTreeMap;

use serde_json::Value;
use switchback_traits::{Entity, EntityBody, EntityCategory, EntityId, ChannelBody};

use crate::category::AsyncApiCategory;
use crate::paths::{COMPONENTS_GROUP, slugify};
use crate::populate::PopulateCtx;
use crate::populate::PopulatedEntity;
use crate::populate::bindings::append_generic_bindings;
use crate::populate::groups::merge_entity_tags;
use crate::populate::protocol_attach;
use crate::populate::refs::structural_refs;

pub fn populate_channels(
    root: &Value,
    ctx: &PopulateCtx<'_>,
    entities_by_group: &mut BTreeMap<String, Vec<PopulatedEntity>>,
    is_v3: bool,
) {
    let Some(channels) = root.get("channels").and_then(|v| v.as_object()) else {
        return;
    };

    for (name, channel_val) in channels {
        let tags: Vec<String> = channel_val
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

        let address = channel_val
            .get("address")
            .or_else(|| channel_val.get("title"))
            .and_then(|v| v.as_str())
            .unwrap_or(name);
        let bindings = channel_val.get("bindings");
        let mut fence_body = serialize_fence(channel_val, ctx.doc.fence_language());
        fence_body = append_generic_bindings(&fence_body, bindings, ctx.doc.fence_language());
        let _protocols = bindings.map(protocol_attach::bindings_protocols);

        let doc_text = channel_val
            .get("description")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .filter(|s| !s.is_empty());
        let refs = structural_refs(
            channel_val,
            ctx.doc_uri,
            ctx.module_id,
            ctx.uri_to_group,
            ctx.index,
        );

        let signature = if is_v3 {
            format!("channel {address}")
        } else {
            format!("channel {name}")
        };

        for group_id in target_groups {
            let group_key = slugify(&group_id);
            let pe = PopulatedEntity {
                entity: Entity {
                    id: EntityId::new(
                        group_key.as_str(),
                        AsyncApiCategory::Channel.as_str(),
                        name,
                    ),
                    category: AsyncApiCategory::Channel,
                    title: channel_val
                        .get("title")
                        .or_else(|| channel_val.get("summary"))
                        .and_then(|v| v.as_str())
                        .unwrap_or(name)
                        .to_string(),
                    doc: doc_text.clone(),
                    source_span: None,
                    body: EntityBody::Channel(ChannelBody {
                        signature: signature.clone(),
                        fence_language: ctx.doc.fence_language().to_string(),
                        fence_body: fence_body.clone(),
                    }),
                },
                refs: refs.clone(),
            };
            entities_by_group.entry(group_key).or_default().push(pe);
        }

        if !is_v3 {
            populate_channel_operations_2x(name, channel_val, ctx, entities_by_group);
        }
    }
}

fn populate_channel_operations_2x(
    channel_name: &str,
    channel_val: &Value,
    ctx: &PopulateCtx<'_>,
    entities_by_group: &mut BTreeMap<String, Vec<PopulatedEntity>>,
) {
    for action in ["publish", "subscribe"] {
        let Some(op) = channel_val.get(action) else {
            continue;
        };
        crate::populate::operations::push_operation_entity(
            channel_name,
            action,
            op,
            channel_val.get("bindings"),
            ctx,
            entities_by_group,
            None,
        );
    }
}

fn serialize_fence(value: &Value, fence_language: &str) -> String {
    if fence_language == "yaml" {
        serde_saphyr::to_string(value).unwrap_or_else(|_| "{}".into())
    } else {
        serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".into())
    }
}

pub fn entity_group<'a>(ctx: &'a PopulateCtx<'a>) -> &'a str {
    ctx.entry_group.unwrap_or(COMPONENTS_GROUP)
}
