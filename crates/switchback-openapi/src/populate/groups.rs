//! Tag / `x-tagGroups` / components / untagged group planning.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;
use switchback_jsonschema::envelope::Envelope;
use switchback_traits::{Group, GroupId, Source};

use crate::paths::{COMPONENTS_GROUP, UNTAGGED_GROUP, slugify};

pub struct GroupPlan {
    pub groups: Vec<Group>,
    pub _tag_order: Vec<String>,
    pub _tag_titles: BTreeMap<String, String>,
}

pub fn build_groups(
    root: &Value,
    envelope: &Envelope,
    entry_uri: &str,
    module_root: &std::path::Path,
    operation_tags: &BTreeSet<String>,
    has_untagged: bool,
) -> GroupPlan {
    let mut tag_titles: BTreeMap<String, String> = BTreeMap::new();
    for tag in &envelope.tags {
        tag_titles.insert(tag.name.clone(), tag.name.clone());
    }

    let tag_order = ordered_tags(root, operation_tags);

    let mut groups = Vec::new();

    for tag in &tag_order {
        let id = slugify(tag);
        groups.push(Group {
            id: GroupId::from(id.as_str()),
            dir: id.clone(),
            title: id.clone(),
            overview: tag_description(root, tag),
            source: None,
            entities: Vec::new(),
            source_path: module_root.join(entry_uri),
        });
    }

    if has_untagged || operation_tags.contains(UNTAGGED_GROUP) {
        groups.push(untagged_group(module_root, entry_uri));
    }

    groups.push(components_group(module_root, entry_uri));

    GroupPlan {
        groups,
        _tag_order: tag_order,
        _tag_titles: tag_titles,
    }
}

fn ordered_tags(root: &Value, operation_tags: &BTreeSet<String>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut order = Vec::new();

    if let Some(tag_groups) = root
        .as_object()
        .and_then(|o| o.get("x-tagGroups"))
        .and_then(|v| v.as_array())
    {
        for tg in tag_groups {
            if let Some(tags) = tg
                .as_object()
                .and_then(|o| o.get("tags"))
                .and_then(|v| v.as_array())
            {
                for tag in tags {
                    if let Some(name) = tag.as_str()
                        && seen.insert(name.to_string())
                    {
                        order.push(name.to_string());
                    }
                }
            }
        }
    }

    for tag in operation_tags {
        if tag == UNTAGGED_GROUP || tag == COMPONENTS_GROUP {
            continue;
        }
        if seen.insert(tag.clone()) {
            order.push(tag.clone());
        }
    }

    order
}

fn tag_description(root: &Value, tag_name: &str) -> Option<String> {
    root.get("tags")
        .and_then(|v| v.as_array())
        .and_then(|tags| {
            tags.iter().find_map(|t| {
                t.as_object()
                    .filter(|o| o.get("name").and_then(|n| n.as_str()) == Some(tag_name))
                    .and_then(|o| o.get("description"))
                    .and_then(|d| d.as_str())
                    .map(str::to_string)
            })
        })
        .filter(|s| !s.is_empty())
}

fn untagged_group(module_root: &std::path::Path, entry_uri: &str) -> Group {
    Group {
        id: GroupId::from(UNTAGGED_GROUP),
        dir: UNTAGGED_GROUP.to_string(),
        title: "Untagged".to_string(),
        overview: None,
        source: None,
        entities: Vec::new(),
        source_path: module_root.join(entry_uri),
    }
}

fn components_group(module_root: &std::path::Path, entry_uri: &str) -> Group {
    Group {
        id: GroupId::from(COMPONENTS_GROUP),
        dir: COMPONENTS_GROUP.to_string(),
        title: "Components".to_string(),
        overview: None,
        source: Some(Source {
            file: entry_uri.to_string(),
            span: None,
        }),
        entities: Vec::new(),
        source_path: module_root.join(entry_uri),
    }
}

pub fn merge_operation_tags(op_tags: &[String], path_tags: &[String]) -> Vec<String> {
    let mut out = BTreeSet::new();
    for t in path_tags.iter().chain(op_tags.iter()) {
        out.insert(t.clone());
    }
    if out.is_empty() {
        vec![UNTAGGED_GROUP.to_string()]
    } else {
        out.into_iter().collect()
    }
}
