//! Assemble a [`ReferenceManual`] from a populated protobuf contract.

use std::path::Path;

use sha2::{Digest, Sha256};
use switchback_codec_pb::WIRE_VERSION;
use switchback_traits::{
    companion_files_to_stored, ContractFamily, Document, EntityCategory, LinkExtractor,
    ManualContract, Module, ModuleId, ReferenceManual, ResolvedManual, Source, SourceRef,
    StoredEntity,
};

use crate::family::ProtobufFamily;
use crate::input::ResolvedInput;
use crate::link::ProtobufLinkExtractor;
use crate::populate::PopulatedContract;

pub fn build_reference_manual(
    populated: PopulatedContract,
    resolved: &ResolvedInput,
    title: Option<String>,
) -> switchback_traits::Result<ReferenceManual> {
    let family = ProtobufFamily;
    let module_id = ModuleId::from(populated.module_id.as_str());
    let manual_title = title.unwrap_or_else(|| family.default_title().to_string());

    let sources = build_sources(resolved)?;
    let mut groups = populated.groups;
    let extractor = ProtobufLinkExtractor;

    for group in &mut groups {
        let stored: Vec<StoredEntity> = populated
            .entities_by_group
            .get(&group.id)
            .map(|entities| {
                entities
                    .iter()
                    .map(|pe| stored_entity_from_populated(pe, &module_id, &extractor, None))
                    .collect()
            })
            .unwrap_or_default();
        group.entities = stored;
    }

    let mut manual = ReferenceManual {
        switchback_version: WIRE_VERSION.to_string(),
        title: manual_title.clone(),
        sources,
        modules: vec![Module {
            id: module_id.clone(),
            title: manual_title,
            overview: String::new(),
            contracts: vec![ManualContract {
                family: family.name().to_string(),
                version: populated.version,
                groups,
                companions: companion_files_to_stored(&populated.companions, "text/markdown"),
            }],
        }],
    };

    let resolved_manual = ResolvedManual::from_reference_manual(&manual);
    for contract in &mut manual.modules[0].contracts {
        for group in &mut contract.groups {
            if let Some(entities) = populated.entities_by_group.get(&group.id) {
                group.entities = entities
                    .iter()
                    .map(|pe| {
                        stored_entity_from_populated(
                            pe,
                            &module_id,
                            &extractor,
                            Some(&resolved_manual),
                        )
                    })
                    .collect();
            }
        }
    }

    Ok(manual)
}

fn stored_entity_from_populated(
    pe: &crate::populate::PopulatedEntity,
    _module_id: &ModuleId,
    extractor: &ProtobufLinkExtractor,
    resolved: Option<&ResolvedManual>,
) -> StoredEntity {
    let intra_links = resolved
        .map(|manual| extractor.extract(&pe.entity, manual))
        .unwrap_or_default();
    StoredEntity {
        name: pe.entity.id.name.clone(),
        category: pe.entity.category.as_str().to_string(),
        title: pe.entity.title.clone(),
        doc: pe.entity.doc.clone(),
        source: entity_source(&pe.source_file),
        refs: pe.refs.clone(),
        intra_links,
        body: pe.entity.body.clone(),
    }
}

fn entity_source(file: &str) -> Option<Source> {
    if file.is_empty() {
        None
    } else {
        Some(Source {
            file: file.to_string(),
            span: None,
        })
    }
}

fn build_sources(resolved: &ResolvedInput) -> switchback_traits::Result<Vec<Document>> {
    let mut sources = Vec::new();
    for name in &resolved.file_to_generate {
        let path = resolved.module_root.join(name);
        let content = std::fs::read(&path).map_err(|e| {
            switchback_traits::SwitchbackError::load(format!(
                "read source proto {}: {e}",
                path.display()
            ))
        })?;
        let hash = hex_sha256(&content);
        sources.push(Document {
            source_ref: SourceRef {
                uri: name.clone(),
                commit: String::new(),
                content_hash: hash,
            },
            media_type: "text/x-protobuf".to_string(),
            content,
        });
    }
    sources.sort_by(|a, b| a.source_ref.uri.cmp(&b.source_ref.uri));
    Ok(sources)
}

fn hex_sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn restore_sources(
    manual: &ReferenceManual,
    module_root: &Path,
) -> switchback_traits::Result<()> {
    for doc in &manual.sources {
        let out_path = module_root.join(&doc.source_ref.uri);
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                switchback_traits::SwitchbackError::load(format!(
                    "create directory {}: {e}",
                    parent.display()
                ))
            })?;
        }
        std::fs::write(&out_path, &doc.content).map_err(|e| {
            switchback_traits::SwitchbackError::load(format!(
                "write restored proto {}: {e}",
                out_path.display()
            ))
        })?;
    }
    Ok(())
}
