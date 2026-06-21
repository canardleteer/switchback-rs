//! Catalog population: resolved schemas → switchback entities.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::Value;
use switchback_traits::{
    Entity, EntityBody, EntityCategory, EntityId, Group, GroupId, RefKind, Reference, Source,
    SpecVersion,
};

use crate::category::JsonSchemaCategory;
use crate::companion::discover_companions;
use crate::envelope::Envelope;
use crate::loader::Doc;
use crate::resolver::{NodeRef, RefIndex};
use crate::schema::{Schema, populate_schema_body};

pub struct PopulatedEntity {
    pub entity: Entity<JsonSchemaCategory>,
    pub refs: Vec<Reference>,
}

pub struct PopulatedContract {
    pub version: SpecVersion,
    pub module_id: String,
    pub groups: Vec<Group>,
    pub entities_by_group: BTreeMap<GroupId, Vec<PopulatedEntity>>,
    pub companions: Vec<switchback_traits::CompanionFile>,
    pub module_root: PathBuf,
}

pub struct ResolvedInput {
    pub module_root: PathBuf,
    pub entry_uris: Vec<String>,
    pub docs: Vec<Doc>,
    pub index: RefIndex,
}

struct PopulateCtx<'a> {
    doc: &'a Doc,
    doc_uri: &'a str,
    group_id: &'a str,
    module_id: &'a str,
    uri_to_group: &'a BTreeMap<String, String>,
    index: &'a RefIndex,
}

pub fn populate(resolved: &ResolvedInput) -> switchback_traits::Result<PopulatedContract> {
    let doc_map: BTreeMap<&str, &Doc> = resolved.docs.iter().map(|d| (d.uri.as_str(), d)).collect();
    let uri_to_group: BTreeMap<String, String> = resolved
        .entry_uris
        .iter()
        .map(|uri| (uri.clone(), group_id_from_uri(uri)))
        .collect();

    let companions =
        discover_companions(&resolved.docs, &resolved.entry_uris, &resolved.module_root)?;

    let module_id = resolved
        .entry_uris
        .first()
        .map(|u| group_id_from_uri(u))
        .unwrap_or_else(|| "default".into());

    let mut groups = Vec::new();
    let mut entities_by_group = BTreeMap::new();

    for entry_uri in &resolved.entry_uris {
        let Some(doc) = doc_map.get(entry_uri.as_str()) else {
            continue;
        };
        let group_id = group_id_from_uri(entry_uri);
        let dir = group_dir_from_uri(entry_uri);
        let envelope = Envelope::from_value(&doc.value);
        let title = envelope.title_or_fallback(&group_id);

        groups.push(Group {
            id: GroupId::from(group_id.as_str()),
            dir,
            title: title.to_string(),
            overview: envelope.info.description.clone(),
            source: Some(Source {
                file: entry_uri.clone(),
                span: None,
            }),
            entities: Vec::new(),
            source_path: resolved.module_root.join(entry_uri),
        });

        let mut entities = Vec::new();
        let ctx = PopulateCtx {
            doc,
            doc_uri: entry_uri,
            group_id: &group_id,
            module_id: &module_id,
            uri_to_group: &uri_to_group,
            index: &resolved.index,
        };
        collect_schema_entities(&doc.value, &ctx, &mut entities, true);
        entities_by_group.insert(GroupId::from(group_id.as_str()), entities);
    }

    Ok(PopulatedContract {
        version: SpecVersion::from("2020-12"),
        module_id,
        groups,
        entities_by_group,
        companions,
        module_root: resolved.module_root.clone(),
    })
}

fn group_id_from_uri(uri: &str) -> String {
    Path::new(uri)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("schema")
        .to_string()
}

fn group_dir_from_uri(uri: &str) -> String {
    let parent = Path::new(uri).parent().unwrap_or(Path::new(""));
    crate::paths::normalize_rel_dir(parent)
        .to_string_lossy()
        .into_owned()
}

fn collect_schema_entities(
    root: &Value,
    ctx: &PopulateCtx<'_>,
    out: &mut Vec<PopulatedEntity>,
    include_root: bool,
) {
    if include_root && is_named_root_schema(root) {
        let name = entity_name_from_value(root, ctx.group_id);
        push_entity(root, ctx, &name, out);
    }

    let defs_key = if root.as_object().and_then(|o| o.get("$defs")).is_some() {
        "$defs"
    } else {
        "definitions"
    };

    if let Some(defs) = root
        .as_object()
        .and_then(|o| o.get(defs_key))
        .and_then(|v| v.as_object())
    {
        for (name, schema_val) in defs {
            if Schema::is_schema_value(schema_val) {
                push_entity(schema_val, ctx, name, out);
            }
        }
    }
}

fn is_named_root_schema(value: &Value) -> bool {
    if !Schema::is_schema_value(value) {
        return false;
    }
    let Some(obj) = value.as_object() else {
        return matches!(value, Value::Bool(_));
    };
    if obj.contains_key("openapi")
        || obj.contains_key("asyncapi")
        || obj.contains_key("openrpc")
        || obj.contains_key("swagger")
    {
        return false;
    }
    obj.contains_key("type")
        || obj.contains_key("$ref")
        || obj.contains_key("properties")
        || obj.contains_key("allOf")
        || obj.contains_key("title")
        || obj.contains_key("$id")
}

fn entity_name_from_value(value: &Value, fallback: &str) -> String {
    if let Some(obj) = value.as_object() {
        if let Some(title) = obj
            .get("title")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
        {
            return title.to_string();
        }
        if let Some(id) = obj.get("$id").and_then(|v| v.as_str())
            && let Some(segment) = id.rsplit('/').next().filter(|s| !s.is_empty())
        {
            return segment.to_string();
        }
    }
    fallback.to_string()
}

fn push_entity(value: &Value, ctx: &PopulateCtx<'_>, name: &str, out: &mut Vec<PopulatedEntity>) {
    let doc = ctx.doc.clone();
    let resolver = |base_uri: &str, file_part: &str| -> Option<String> {
        if file_part.is_empty() {
            return ctx.uri_to_group.get(base_uri).cloned();
        }
        let file_part = crate::paths::strip_dot_slash(file_part);
        let candidate = Path::new(base_uri)
            .parent()
            .unwrap_or(Path::new(""))
            .join(file_part);
        let normalized = candidate.to_string_lossy().replace('\\', "/");
        ctx.uri_to_group.get(&normalized).cloned()
    };

    let body = populate_schema_body(value, &doc, ctx.module_id, ctx.group_id, &resolver);
    let doc_text = value
        .as_object()
        .and_then(|o| o.get("description"))
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .filter(|s| !s.is_empty());

    let refs = structural_refs(
        value,
        ctx.doc_uri,
        ctx.group_id,
        ctx.module_id,
        ctx.uri_to_group,
        ctx.index,
    );

    out.push(PopulatedEntity {
        entity: Entity {
            id: EntityId::new(ctx.group_id, JsonSchemaCategory::Schema.as_str(), name),
            category: JsonSchemaCategory::Schema,
            title: name.to_string(),
            doc: doc_text,
            source_span: None,
            body: EntityBody::Schema(body),
        },
        refs,
    });
}

fn structural_refs(
    value: &Value,
    doc_uri: &str,
    group_id: &str,
    module_id: &str,
    uri_to_group: &BTreeMap<String, String>,
    index: &RefIndex,
) -> Vec<Reference> {
    let mut refs = Vec::new();
    collect_ref_strings(
        value,
        &mut refs,
        doc_uri,
        group_id,
        module_id,
        uri_to_group,
        index,
    );
    refs
}

fn collect_ref_strings(
    value: &Value,
    out: &mut Vec<Reference>,
    doc_uri: &str,
    group_id: &str,
    module_id: &str,
    uri_to_group: &BTreeMap<String, String>,
    index: &RefIndex,
) {
    match value {
        Value::Object(map) => {
            if let Some(Value::String(ref_key)) = map.get("$ref")
                && let Some(reference) =
                    ref_to_reference(ref_key, doc_uri, group_id, module_id, uri_to_group, index)
            {
                out.push(reference);
            }
            for v in map.values() {
                collect_ref_strings(v, out, doc_uri, group_id, module_id, uri_to_group, index);
            }
        }
        Value::Array(items) => {
            for v in items {
                collect_ref_strings(v, out, doc_uri, group_id, module_id, uri_to_group, index);
            }
        }
        _ => {}
    }
}

fn ref_to_reference(
    ref_key: &str,
    doc_uri: &str,
    group_id: &str,
    module_id: &str,
    uri_to_group: &BTreeMap<String, String>,
    index: &RefIndex,
) -> Option<Reference> {
    let (target_group, target_name) =
        resolve_ref_target(ref_key, doc_uri, group_id, uri_to_group, index)?;
    Some(Reference {
        target: EntityRef {
            module: module_id.to_string(),
            group: target_group,
            category: "schema".to_string(),
            name: target_name,
        },
        kind: if ref_key.contains("#/components/") {
            RefKind::Component
        } else {
            RefKind::Internal
        },
    })
}

use switchback_traits::EntityRef;

fn resolve_ref_target(
    ref_key: &str,
    doc_uri: &str,
    default_group: &str,
    uri_to_group: &BTreeMap<String, String>,
    index: &RefIndex,
) -> Option<(String, String)> {
    let (file_part, pointer) = if let Some((file, ptr)) = ref_key.split_once('#') {
        (file, ptr.trim_start_matches('/'))
    } else if ref_key.starts_with('#') {
        ("", ref_key.trim_start_matches('/'))
    } else {
        return None;
    };

    let target_doc_uri = if file_part.is_empty() {
        doc_uri.to_string()
    } else {
        let file_part = crate::paths::strip_dot_slash(file_part);
        let candidate = Path::new(doc_uri)
            .parent()
            .unwrap_or(Path::new(""))
            .join(file_part);
        candidate.to_string_lossy().replace('\\', "/")
    };

    let target_group = uri_to_group
        .get(&target_doc_uri)
        .cloned()
        .unwrap_or_else(|| default_group.to_string());

    let node = NodeRef::with_pointer(&target_doc_uri, pointer);
    if let Some(resolved) = index.ref_targets.get(&node)
        && resolved.pointer.ends_with("#recursive")
    {
        return Some((target_group, "recursive".into()));
    }

    let name = if pointer.starts_with("$defs/") || pointer.starts_with("definitions/") {
        pointer.split('/').nth(1)?.to_string()
    } else if pointer.starts_with("components/schemas/") {
        pointer.split('/').nth(2)?.to_string()
    } else if pointer.is_empty() {
        target_group.clone()
    } else {
        pointer.split('/').next_back()?.to_string()
    };

    Some((target_group, name))
}
