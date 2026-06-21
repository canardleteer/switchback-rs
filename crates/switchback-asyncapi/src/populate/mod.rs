//! AsyncAPI document population.

mod bindings;
mod channels;
mod components;
mod entry_group;
mod groups;
mod mermaid;
mod messages;
mod operations;
pub mod protocol_attach;
mod refs;
mod schema_dispatch;
mod schema_outbreak;
mod version;

use std::collections::BTreeMap;
use std::path::PathBuf;

use switchback_jsonschema::envelope::Envelope;
use switchback_jsonschema::loader::Doc;
use switchback_jsonschema::resolver::RefIndex;
use switchback_traits::{Group, GroupId, SpecVersion};

use crate::companion::{discover_companions, discover_companions_multi};
use crate::paths::{COMPONENTS_GROUP, module_id_from_id_title_or_stem};
use crate::populate::channels::populate_channels;
use crate::populate::components::populate_components;
use crate::populate::entry_group::{entry_group_dir, entry_group_id, entry_group_scope};
use crate::populate::groups::{build_groups, collect_entity_tags};
use crate::populate::operations::populate_operations_3x;

pub use refs::{ref_to_reference, resolve_ref_target, structural_refs};
pub use version::{is_asyncapi_3, parse_asyncapi_version};

pub struct PopulatedEntity {
    pub entity: switchback_traits::Entity<crate::category::AsyncApiCategory>,
    pub refs: Vec<switchback_traits::Reference>,
}

pub struct PopulatedContract {
    pub version: SpecVersion,
    pub module_id: String,
    pub document_title: Option<String>,
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

pub struct PopulateCtx<'a> {
    pub doc: &'a Doc,
    pub doc_uri: &'a str,
    pub module_id: &'a str,
    pub uri_to_group: &'a BTreeMap<String, String>,
    pub index: &'a RefIndex,
    pub entry_group: Option<&'a str>,
}

pub fn populate(resolved: &ResolvedInput) -> switchback_traits::Result<PopulatedContract> {
    if resolved.entry_uris.is_empty() {
        return Err(switchback_traits::SwitchbackError::load(
            "no AsyncAPI entry document",
        ));
    }
    if resolved.entry_uris.len() <= 1 {
        populate_single_entry(resolved)
    } else {
        populate_multi_entry(resolved)
    }
}

fn populate_single_entry(resolved: &ResolvedInput) -> switchback_traits::Result<PopulatedContract> {
    let entry_uri = resolved
        .entry_uris
        .first()
        .expect("single entry has one uri");

    let doc = resolved
        .docs
        .iter()
        .find(|d| d.uri == *entry_uri)
        .ok_or_else(|| {
            switchback_traits::SwitchbackError::load(format!("missing entry doc {entry_uri}"))
        })?;

    let version = parse_asyncapi_version(&doc.value)?;
    let raw_version = doc
        .value
        .get("asyncapi")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let is_v3 = is_asyncapi_3(raw_version);
    let envelope = Envelope::from_value(&doc.value);
    let module_id = module_id_from_id_title_or_stem(&doc.value, entry_uri);

    if let Some(group_id) = entry_group_scope(entry_uri, &doc.value) {
        return populate_scoped_entry(resolved, entry_uri, doc, &module_id, &group_id, is_v3);
    }

    let uri_to_group: BTreeMap<String, String> = resolved
        .docs
        .iter()
        .map(|d| (d.uri.clone(), COMPONENTS_GROUP.to_string()))
        .collect();

    let (entity_tags, has_untagged) = collect_entity_tags(&doc.value, is_v3);
    let group_plan = build_groups(
        &doc.value,
        &envelope,
        entry_uri,
        &resolved.module_root,
        &entity_tags,
        has_untagged,
    );

    let ctx = PopulateCtx {
        doc,
        doc_uri: entry_uri,
        module_id: &module_id,
        uri_to_group: &uri_to_group,
        index: &resolved.index,
        entry_group: None,
    };

    let mut entities_by_group: BTreeMap<String, Vec<PopulatedEntity>> = BTreeMap::new();

    let mut component_entities = Vec::new();
    populate_components(&doc.value, &ctx, &mut component_entities);
    entities_by_group.insert(COMPONENTS_GROUP.to_string(), component_entities);

    populate_channels(&doc.value, &ctx, &mut entities_by_group, is_v3);
    if is_v3 {
        populate_operations_3x(&doc.value, &ctx, &mut entities_by_group);
    }

    let companions = discover_companions(&resolved.entry_uris, &resolved.module_root)?;

    let entities_by_group_id: BTreeMap<GroupId, Vec<PopulatedEntity>> = entities_by_group
        .into_iter()
        .map(|(k, v)| (GroupId::from(k.as_str()), v))
        .collect();

    Ok(PopulatedContract {
        version,
        module_id,
        document_title: envelope.info.title.clone(),
        groups: group_plan.groups,
        entities_by_group: entities_by_group_id,
        companions,
        module_root: resolved.module_root.clone(),
    })
}

fn populate_scoped_entry(
    resolved: &ResolvedInput,
    entry_uri: &str,
    doc: &Doc,
    module_id: &str,
    group_id: &str,
    is_v3: bool,
) -> switchback_traits::Result<PopulatedContract> {
    use switchback_traits::Source;

    let version = parse_asyncapi_version(&doc.value)?;
    let envelope = Envelope::from_value(&doc.value);
    let group_dir = entry_group_dir(group_id);

    let mut uri_to_group = BTreeMap::new();
    uri_to_group.insert(entry_uri.to_string(), group_id.to_string());
    for d in &resolved.docs {
        uri_to_group
            .entry(d.uri.clone())
            .or_insert_with(|| COMPONENTS_GROUP.to_string());
    }

    let ctx = PopulateCtx {
        doc,
        doc_uri: entry_uri,
        module_id,
        uri_to_group: &uri_to_group,
        index: &resolved.index,
        entry_group: Some(group_id),
    };

    let mut bucket: BTreeMap<String, Vec<PopulatedEntity>> = BTreeMap::new();
    let mut components = Vec::new();
    populate_components(&doc.value, &ctx, &mut components);
    bucket.insert(group_id.to_string(), components);
    populate_channels(&doc.value, &ctx, &mut bucket, is_v3);
    if is_v3 {
        populate_operations_3x(&doc.value, &ctx, &mut bucket);
    }

    let mut merged = Vec::new();
    if let Some(mut ents) = bucket.remove(group_id) {
        merged.append(&mut ents);
    }
    for (_, mut ents) in bucket {
        merged.append(&mut ents);
    }

    let mut entities_by_group = BTreeMap::new();
    entities_by_group.insert(GroupId::from(group_id), merged);

    let companions = discover_companions(&resolved.entry_uris, &resolved.module_root)?;

    Ok(PopulatedContract {
        version,
        module_id: module_id.to_string(),
        document_title: envelope.info.title.clone(),
        groups: vec![Group {
            id: GroupId::from(group_id),
            dir: group_dir,
            title: envelope
                .info
                .title
                .clone()
                .unwrap_or_else(|| group_id.to_string()),
            overview: envelope.info.description.clone(),
            source: Some(Source {
                file: entry_uri.to_string(),
                span: None,
            }),
            entities: Vec::new(),
            source_path: resolved.module_root.join(entry_uri),
        }],
        entities_by_group,
        companions,
        module_root: resolved.module_root.clone(),
    })
}

fn populate_multi_entry(resolved: &ResolvedInput) -> switchback_traits::Result<PopulatedContract> {
    use switchback_traits::Source;

    let doc_map: BTreeMap<&str, &Doc> = resolved.docs.iter().map(|d| (d.uri.as_str(), d)).collect();

    let mut uri_to_group = BTreeMap::new();
    for entry_uri in &resolved.entry_uris {
        let Some(doc) = doc_map.get(entry_uri.as_str()) else {
            return Err(switchback_traits::SwitchbackError::load(format!(
                "missing entry doc {entry_uri}"
            )));
        };
        uri_to_group.insert(entry_uri.clone(), entry_group_id(entry_uri, &doc.value));
    }

    for doc in &resolved.docs {
        uri_to_group
            .entry(doc.uri.clone())
            .or_insert_with(|| COMPONENTS_GROUP.to_string());
    }

    let first_uri = resolved
        .entry_uris
        .first()
        .expect("multi-entry has entries");
    let first_doc = doc_map
        .get(first_uri.as_str())
        .expect("first entry doc exists");
    let first_envelope = Envelope::from_value(&first_doc.value);
    let module_id = module_id_from_id_title_or_stem(&first_doc.value, first_uri);

    let mut version = SpecVersion::from("2.6.0");
    let mut document_title = first_envelope.info.title.clone();
    let mut groups = Vec::new();
    let mut entities_by_group: BTreeMap<GroupId, Vec<PopulatedEntity>> = BTreeMap::new();

    for entry_uri in &resolved.entry_uris {
        let doc = doc_map.get(entry_uri.as_str()).expect("entry doc in map");
        version = max_asyncapi_version(version, parse_asyncapi_version(&doc.value)?);
        let raw_version = doc
            .value
            .get("asyncapi")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let is_v3 = is_asyncapi_3(raw_version);
        let envelope = Envelope::from_value(&doc.value);
        if document_title.as_deref().unwrap_or("").is_empty() {
            document_title = envelope.info.title.clone();
        }
        let group_id = entry_group_id(entry_uri, &doc.value);
        let group_dir = entry_group_dir(&group_id);

        groups.push(Group {
            id: GroupId::from(group_id.as_str()),
            dir: group_dir,
            title: envelope
                .info
                .title
                .clone()
                .unwrap_or_else(|| group_id.clone()),
            overview: envelope.info.description.clone(),
            source: Some(Source {
                file: entry_uri.clone(),
                span: None,
            }),
            entities: Vec::new(),
            source_path: resolved.module_root.join(entry_uri),
        });

        let ctx = PopulateCtx {
            doc,
            doc_uri: entry_uri,
            module_id: &module_id,
            uri_to_group: &uri_to_group,
            index: &resolved.index,
            entry_group: Some(group_id.as_str()),
        };

        let mut bucket: BTreeMap<String, Vec<PopulatedEntity>> = BTreeMap::new();
        let mut components = Vec::new();
        populate_components(&doc.value, &ctx, &mut components);
        bucket.insert(group_id.clone(), components);
        populate_channels(&doc.value, &ctx, &mut bucket, is_v3);
        if is_v3 {
            populate_operations_3x(&doc.value, &ctx, &mut bucket);
        }

        let mut merged = Vec::new();
        if let Some(mut ents) = bucket.remove(&group_id) {
            merged.append(&mut ents);
        }
        for (_, mut ents) in bucket {
            merged.append(&mut ents);
        }
        entities_by_group.insert(GroupId::from(group_id.as_str()), merged);
    }

    let companions = discover_companions_multi(&resolved.entry_uris, &resolved.module_root)?;

    Ok(PopulatedContract {
        version,
        module_id,
        document_title,
        groups,
        entities_by_group,
        companions,
        module_root: resolved.module_root.clone(),
    })
}

fn max_asyncapi_version(a: SpecVersion, b: SpecVersion) -> SpecVersion {
    if a.as_str().starts_with("3") || b.as_str().starts_with("3") {
        SpecVersion::from("3.0.0")
    } else {
        a
    }
}
