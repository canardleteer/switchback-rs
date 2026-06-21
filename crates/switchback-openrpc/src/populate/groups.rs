//! `x-tagGroup` / default / components group planning.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;
use switchback_jsonschema::envelope::Envelope;
use switchback_traits::{Group, GroupId, Source};

use crate::paths::{COMPONENTS_GROUP, DEFAULT_GROUP, slugify};

pub struct GroupPlan {
    pub groups: Vec<Group>,
}

pub fn build_groups(
    root: &Value,
    _envelope: &Envelope,
    entry_uri: &str,
    module_root: &std::path::Path,
    method_tags: &BTreeSet<String>,
    has_default: bool,
) -> GroupPlan {
    let tag_order = ordered_tag_groups(root, method_tags);
    let mut groups = Vec::new();

    for tag in &tag_order {
        let id = slugify(tag);
        groups.push(Group {
            id: GroupId::from(id.as_str()),
            dir: id.clone(),
            title: id.clone(),
            overview: tag_group_description(root, tag),
            source: None,
            entities: Vec::new(),
            source_path: module_root.join(entry_uri),
        });
    }

    if has_default || method_tags.contains(DEFAULT_GROUP) {
        groups.push(default_group(module_root, entry_uri));
    }

    groups.push(components_group(module_root, entry_uri));

    GroupPlan { groups }
}

fn ordered_tag_groups(root: &Value, method_tags: &BTreeSet<String>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut order = Vec::new();

    if let Some(tag_groups) = root
        .as_object()
        .and_then(|o| o.get("x-tagGroup"))
        .and_then(|v| v.as_array())
    {
        for tg in tag_groups {
            if let Some(name) = tg
                .as_object()
                .and_then(|o| o.get("name"))
                .and_then(|v| v.as_str())
                && seen.insert(name.to_string())
            {
                order.push(name.to_string());
            }
        }
    }

    for tag in method_tags {
        if tag == DEFAULT_GROUP || tag == COMPONENTS_GROUP {
            continue;
        }
        if seen.insert(tag.clone()) {
            order.push(tag.clone());
        }
    }

    order
}

fn tag_group_description(root: &Value, group_name: &str) -> Option<String> {
    root.get("x-tagGroup")
        .and_then(|v| v.as_array())
        .and_then(|groups| {
            groups.iter().find_map(|tg| {
                tg.as_object()
                    .filter(|o| o.get("name").and_then(|n| n.as_str()) == Some(group_name))
                    .and_then(|o| o.get("description"))
                    .and_then(|d| d.as_str())
                    .map(str::to_string)
            })
        })
        .filter(|s| !s.is_empty())
}

fn default_group(module_root: &std::path::Path, entry_uri: &str) -> Group {
    Group {
        id: GroupId::from(DEFAULT_GROUP),
        dir: DEFAULT_GROUP.to_string(),
        title: "Default".to_string(),
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

/// Map method tags to target group ids using `x-tagGroup` membership.
pub fn merge_method_tags(root: &Value, method_tags: &[String]) -> Vec<String> {
    if method_tags.is_empty() {
        return vec![DEFAULT_GROUP.to_string()];
    }

    let tag_to_group = tag_group_membership(root);
    let mut out = BTreeSet::new();
    for tag in method_tags {
        if let Some(group) = tag_to_group.get(tag) {
            out.insert(group.clone());
        } else {
            out.insert(slugify(tag));
        }
    }
    out.into_iter().collect()
}

fn tag_group_membership(root: &Value) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    let Some(groups) = root.get("x-tagGroup").and_then(|v| v.as_array()) else {
        return map;
    };
    for tg in groups {
        let Some(obj) = tg.as_object() else {
            continue;
        };
        let Some(name) = obj.get("name").and_then(|v| v.as_str()) else {
            continue;
        };
        let group_id = slugify(name);
        if let Some(tags) = obj.get("tags").and_then(|v| v.as_array()) {
            for tag in tags {
                if let Some(tag_name) = tag.as_str() {
                    map.insert(tag_name.to_string(), group_id.clone());
                }
            }
        }
        map.insert(name.to_string(), group_id);
    }
    map
}

pub fn collect_method_tags(root: &Value) -> (BTreeSet<String>, bool) {
    let mut tags = BTreeSet::new();
    let mut has_default = false;

    let Some(methods) = root.get("methods").and_then(|v| v.as_array()) else {
        return (tags, has_default);
    };

    for method in methods {
        let method_tags: Vec<String> = method
            .get("tags")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|t| t.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        if method_tags.is_empty() {
            has_default = true;
            tags.insert(DEFAULT_GROUP.to_string());
        } else {
            for tag in method_tags {
                tags.insert(tag);
            }
        }
    }

    (tags, has_default)
}
