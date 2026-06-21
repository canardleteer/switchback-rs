//! AsyncAPI document population (skeleton: version gate and empty entities).

mod version;

use std::collections::BTreeMap;
use std::path::PathBuf;

use switchback_jsonschema::envelope::Envelope;
use switchback_jsonschema::loader::Doc;
use switchback_jsonschema::resolver::RefIndex;
use switchback_traits::{Group, GroupId, Source, SpecVersion};

use crate::companion::discover_companions;
use crate::paths::{COMPONENTS_GROUP, module_id_from_id_title_or_stem};

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
    let entry_uri = resolved
        .entry_uris
        .first()
        .ok_or_else(|| switchback_traits::SwitchbackError::load("no AsyncAPI entry document"))?;

    let doc = resolved
        .docs
        .iter()
        .find(|d| d.uri == *entry_uri)
        .ok_or_else(|| {
            switchback_traits::SwitchbackError::load(format!("missing entry doc {entry_uri}"))
        })?;

    let version = parse_asyncapi_version(&doc.value)?;
    let envelope = Envelope::from_value(&doc.value);
    let module_id = module_id_from_id_title_or_stem(&doc.value, entry_uri);
    let companions = discover_companions(&resolved.entry_uris, &resolved.module_root)?;

    let mut entities_by_group = BTreeMap::new();
    entities_by_group.insert(GroupId::from(COMPONENTS_GROUP), Vec::new());

    Ok(PopulatedContract {
        version,
        module_id,
        document_title: envelope.info.title.clone(),
        groups: vec![Group {
            id: GroupId::from(COMPONENTS_GROUP),
            dir: COMPONENTS_GROUP.to_string(),
            title: "Components".to_string(),
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
