//! OpenAPI document population.

mod components;
mod groups;
mod operation;
mod paths;
mod refs;
mod schema_type_label;

use std::collections::BTreeMap;
use std::path::PathBuf;

use switchback_jsonschema::envelope::Envelope;
use switchback_jsonschema::loader::Doc;
use switchback_jsonschema::resolver::RefIndex;
use switchback_traits::{Group, GroupId, SpecVersion};

use crate::companion::discover_companions;
use crate::paths::{module_id_from_title_or_stem, COMPONENTS_GROUP, UNTAGGED_GROUP};
use crate::populate::groups::build_groups;
use crate::populate::paths::{collect_operation_tags, populate_path_operations};

pub use refs::{ref_to_reference, resolve_ref_target, structural_refs};

pub struct PopulatedEntity {
    pub entity: switchback_traits::Entity<crate::category::OpenApiCategory>,
    pub refs: Vec<switchback_traits::Reference>,
}

pub struct PopulatedContract {
    pub version: SpecVersion,
    pub module_id: String,
    /// `info.title` from the entry document, when present.
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
}

pub fn populate(resolved: &ResolvedInput) -> switchback_traits::Result<PopulatedContract> {
    let entry_uri = resolved
        .entry_uris
        .first()
        .ok_or_else(|| switchback_traits::SwitchbackError::load("no OpenAPI entry document"))?;

    let doc = resolved
        .docs
        .iter()
        .find(|d| d.uri == *entry_uri)
        .ok_or_else(|| {
            switchback_traits::SwitchbackError::load(format!("missing entry doc {entry_uri}"))
        })?;

    if doc.value.get("swagger").is_some() {
        return Err(switchback_traits::SwitchbackError::load(
            "Swagger 2.0 is not supported; use OpenAPI 3.0.x or 3.1.x",
        ));
    }

    let version = parse_openapi_version(&doc.value)?;
    let envelope = Envelope::from_value(&doc.value);
    let module_id = module_id_from_title_or_stem(envelope.info.title.as_deref(), entry_uri);

    let uri_to_group: BTreeMap<String, String> = resolved
        .docs
        .iter()
        .map(|d| (d.uri.clone(), COMPONENTS_GROUP.to_string()))
        .collect();

    let (op_tags, has_untagged) = collect_operation_tags(&doc.value);
    let mut tag_set = op_tags;
    if has_untagged {
        tag_set.insert(UNTAGGED_GROUP.to_string());
    }

    let group_plan = build_groups(
        &doc.value,
        &envelope,
        entry_uri,
        &resolved.module_root,
        &tag_set,
        has_untagged,
    );

    let ctx = PopulateCtx {
        doc,
        doc_uri: entry_uri,
        module_id: &module_id,
        uri_to_group: &uri_to_group,
        index: &resolved.index,
    };

    let mut entities_by_group: BTreeMap<String, Vec<PopulatedEntity>> = BTreeMap::new();

    let mut component_entities = Vec::new();
    components::populate_components(&doc.value, &ctx, &mut component_entities);
    entities_by_group.insert(COMPONENTS_GROUP.to_string(), component_entities);

    populate_path_operations(&doc.value, &ctx, &mut entities_by_group);

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

pub fn parse_openapi_version(root: &serde_json::Value) -> switchback_traits::Result<SpecVersion> {
    let version = root
        .get("openapi")
        .and_then(|v| v.as_str())
        .ok_or_else(|| switchback_traits::SwitchbackError::load("missing openapi version field"))?;
    if version.starts_with("3.1") {
        Ok(SpecVersion::from("3.1.0"))
    } else if version.starts_with("3.0") {
        Ok(SpecVersion::from("3.0.3"))
    } else {
        Err(switchback_traits::SwitchbackError::load(format!(
            "unsupported OpenAPI version {version}"
        )))
    }
}
