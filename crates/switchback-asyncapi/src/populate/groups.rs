//! Tag / `x-tagGroups` / components / untagged group planning.

use std::collections::BTreeSet;

use serde_json::Value;
use switchback_jsonschema::envelope::Envelope;
use switchback_traits::{Group, GroupId, Source};

use crate::paths::{COMPONENTS_GROUP, UNTAGGED_GROUP, slugify};

pub struct GroupPlan {
    pub groups: Vec<Group>,
}

pub fn build_groups(
    root: &Value,
    _envelope: &Envelope,
    entry_uri: &str,
    module_root: &std::path::Path,
    entity_tags: &BTreeSet<String>,
    has_untagged: bool,
) -> GroupPlan {
    let tag_order = ordered_tags(root, entity_tags);
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

    if has_untagged || entity_tags.contains(UNTAGGED_GROUP) {
        groups.push(untagged_group(module_root, entry_uri));
    }

    groups.push(components_group(module_root, entry_uri));

    GroupPlan { groups }
}

fn ordered_tags(root: &Value, entity_tags: &BTreeSet<String>) -> Vec<String> {
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

    for tag in entity_tags {
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

pub fn merge_entity_tags(tags: &[String]) -> Vec<String> {
    let mut out = BTreeSet::new();
    for t in tags {
        out.insert(t.clone());
    }
    if out.is_empty() {
        vec![UNTAGGED_GROUP.to_string()]
    } else {
        out.into_iter().collect()
    }
}

pub fn collect_entity_tags(root: &Value, is_v3: bool) -> (BTreeSet<String>, bool) {
    let mut tags = BTreeSet::new();
    let mut has_untagged = false;

    if is_v3 && let Some(ops) = root.get("operations").and_then(|v| v.as_object()) {
        for op in ops.values() {
            scan_tags(op, &mut tags, &mut has_untagged);
        }
    }

    if let Some(channels) = root.get("channels").and_then(|v| v.as_object()) {
        for channel in channels.values() {
            scan_tags(channel, &mut tags, &mut has_untagged);
            if !is_v3 {
                for action in ["publish", "subscribe"] {
                    if let Some(op) = channel.get(action) {
                        scan_tags(op, &mut tags, &mut has_untagged);
                    }
                }
            }
        }
    }

    (tags, has_untagged)
}

fn scan_tags(value: &Value, tags: &mut BTreeSet<String>, has_untagged: &mut bool) {
    let Some(arr) = value.get("tags").and_then(|v| v.as_array()) else {
        return;
    };
    let names: Vec<String> = arr
        .iter()
        .filter_map(|t| t.as_str().map(str::to_string))
        .collect();
    if names.is_empty() {
        *has_untagged = true;
        tags.insert(UNTAGGED_GROUP.to_string());
    } else {
        for name in names {
            tags.insert(name);
        }
    }
}
