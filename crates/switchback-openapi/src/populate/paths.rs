//! Walk `paths` and `webhooks` and emit operation entities.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;
use switchback_traits::{Entity, EntityBody, EntityCategory, EntityId};

use crate::category::OpenApiCategory;
use crate::paths::{HTTP_METHODS, UNTAGGED_GROUP};
use crate::populate::groups::merge_operation_tags;
use crate::populate::operation::build_operation_body;
use crate::populate::refs::structural_refs;
use crate::populate::PopulateCtx;
use crate::populate::PopulatedEntity;

pub fn collect_operation_tags(root: &Value) -> (BTreeSet<String>, bool) {
    let mut tags = BTreeSet::new();
    let mut has_untagged = false;

    if let Some(paths) = root.get("paths").and_then(|v| v.as_object()) {
        for (path, path_item) in paths {
            if path.starts_with('x') && path != "x-" {
                // extension keys on paths object
                if path.starts_with("x-") {
                    continue;
                }
            }
            scan_path_item(path, path_item, &mut tags, &mut has_untagged);
        }
    }

    if let Some(webhooks) = root.get("webhooks").and_then(|v| v.as_object()) {
        for (path, path_item) in webhooks {
            scan_path_item(path, path_item, &mut tags, &mut has_untagged);
        }
    }

    (tags, has_untagged)
}

fn scan_path_item(
    _path: &str,
    path_item: &Value,
    tags: &mut BTreeSet<String>,
    has_untagged: &mut bool,
) {
    let path_tags: Vec<String> = path_item
        .get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|t| t.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();

    for method in HTTP_METHODS {
        if let Some(op) = path_item.get(*method) {
            let op_tags: Vec<String> = op
                .get("tags")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|t| t.as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_default();
            let merged = merge_operation_tags(&op_tags, &path_tags);
            if merged == [UNTAGGED_GROUP.to_string()] {
                *has_untagged = true;
                tags.insert(UNTAGGED_GROUP.to_string());
            } else {
                for t in merged {
                    tags.insert(t);
                }
            }
        }
    }
}

pub fn populate_path_operations(
    root: &Value,
    ctx: &PopulateCtx<'_>,
    entities_by_group: &mut BTreeMap<String, Vec<PopulatedEntity>>,
) {
    if let Some(paths) = root.get("paths").and_then(|v| v.as_object()) {
        for (path, path_item) in paths {
            if path.starts_with("x-") {
                continue;
            }
            populate_path_item(path, path_item, ctx, entities_by_group);
        }
    }

    if let Some(webhooks) = root.get("webhooks").and_then(|v| v.as_object()) {
        for (path, path_item) in webhooks {
            populate_path_item(path, path_item, ctx, entities_by_group);
        }
    }
}

fn populate_path_item(
    path: &str,
    path_item: &Value,
    ctx: &PopulateCtx<'_>,
    entities_by_group: &mut BTreeMap<String, Vec<PopulatedEntity>>,
) {
    let path_tags: Vec<String> = path_item
        .get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|t| t.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();

    for method in HTTP_METHODS {
        let Some(op) = path_item.get(*method) else {
            continue;
        };
        let op_tags: Vec<String> = op
            .get("tags")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|t| t.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        let target_groups = merge_operation_tags(&op_tags, &path_tags);
        let name = crate::populate::operation::operation_name(method, path);
        let title = op
            .get("summary")
            .or_else(|| op.get("operationId"))
            .and_then(|v| v.as_str())
            .unwrap_or(&name)
            .to_string();
        let body = build_operation_body(method, path, op, path_item, ctx.doc, ctx);
        let doc_text = op
            .get("description")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .filter(|s| !s.is_empty());
        let refs = structural_refs(op, ctx.doc_uri, ctx.module_id, ctx.uri_to_group, ctx.index);

        for group_id in target_groups {
            let group_key = if group_id == UNTAGGED_GROUP {
                UNTAGGED_GROUP.to_string()
            } else {
                crate::paths::slugify(&group_id)
            };
            let pe = PopulatedEntity {
                entity: Entity {
                    id: EntityId::new(
                        group_key.as_str(),
                        OpenApiCategory::Operation.as_str(),
                        &name,
                    ),
                    category: OpenApiCategory::Operation,
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
