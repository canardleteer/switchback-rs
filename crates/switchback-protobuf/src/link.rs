//! Protobuf FQN intra-link extraction from entity prose.

use switchback_traits::{
    Entity, EntityCategory, EntityRef, IntraLink, LinkExtractor, LinkTarget, ResolvedManual, anchor,
};

use crate::category::ProtobufCategory;
use crate::descriptor_util::split_proto_type_name;
use crate::family::ProtobufFamily;

/// Extracts bare fully-qualified protobuf type names from leading-comment prose.
#[derive(Clone, Copy, Debug, Default)]
pub struct ProtobufLinkExtractor;

/// Plan-facing alias for [`ProtobufLinkExtractor`].
pub type ProtobufFqnLinkExtractor = ProtobufLinkExtractor;

impl LinkExtractor for ProtobufLinkExtractor {
    type Family = ProtobufFamily;

    fn name(&self) -> &'static str {
        "protobuf-fqn"
    }

    fn extract<C: EntityCategory>(
        &self,
        entity: &Entity<C>,
        manual: &ResolvedManual,
    ) -> Vec<IntraLink> {
        let Some(doc) = entity.doc.as_deref() else {
            return Vec::new();
        };
        extract_fqns_from_doc(doc, entity, manual)
    }
}

fn extract_fqns_from_doc(
    doc: &str,
    entity: &Entity<impl EntityCategory>,
    manual: &ResolvedManual,
) -> Vec<IntraLink> {
    let mut links = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for (byte_start, fqn) in find_fqn_spans(doc) {
        if !seen.insert(fqn.clone()) {
            continue;
        }
        let byte_end = byte_start + fqn.len() as u32;
        let Some(target) = resolve_fqn(&fqn, entity, manual) else {
            continue;
        };
        links.push(IntraLink {
            anchor: anchor("doc", byte_start, byte_end),
            target: LinkTarget::Entity(target),
            raw: fqn,
        });
    }
    links
}

fn find_fqn_spans(doc: &str) -> Vec<(u32, String)> {
    let mut out = Vec::new();
    let bytes = doc.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        let Some(rest) = doc.get(i..) else {
            break;
        };
        let Some(m) = FQN_RE.find(rest) else {
            break;
        };
        let fqn = m.as_str().to_string();
        let start = (i + m.start()) as u32;
        out.push((start, fqn));
        i += m.start() + m.len().max(1);
    }
    out
}

fn resolve_fqn(
    fqn: &str,
    entity: &Entity<impl EntityCategory>,
    manual: &ResolvedManual,
) -> Option<EntityRef> {
    let dotted = if fqn.starts_with('.') {
        fqn.to_string()
    } else {
        format!(".{fqn}")
    };
    let (pkg, name) = split_proto_type_name(&dotted)?;
    if entity.id.group.as_str() == pkg
        && let Some(found) = find_in_group(manual, pkg, name)
    {
        return Some(found);
    }
    find_anywhere(manual, pkg, name)
}

fn find_in_group(manual: &ResolvedManual, group: &str, name: &str) -> Option<EntityRef> {
    manual.entities.iter().find_map(|indexed| {
        if indexed.group_id.as_str() != group || indexed.entity.name != name {
            return None;
        }
        if indexed.entity.category != ProtobufCategory::Schema.as_str() {
            return None;
        }
        Some(EntityRef {
            module: indexed.module_id.as_str().to_string(),
            group: group.to_string(),
            category: indexed.entity.category.clone(),
            name: name.to_string(),
        })
    })
}

fn find_anywhere(manual: &ResolvedManual, group: &str, name: &str) -> Option<EntityRef> {
    manual.entities.iter().find_map(|indexed| {
        if indexed.group_id.as_str() != group || indexed.entity.name != name {
            return None;
        }
        if indexed.entity.category != ProtobufCategory::Schema.as_str() {
            return None;
        }
        Some(EntityRef {
            module: indexed.module_id.as_str().to_string(),
            group: group.to_string(),
            category: indexed.entity.category.clone(),
            name: name.to_string(),
        })
    })
}

static FQN_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r"\b([a-z][a-z0-9_]*(?:\.[a-z][a-z0-9_]*)+\.[A-Z][A-Za-z0-9_]*)\b")
        .expect("valid fqn regex")
});

#[cfg(test)]
mod tests {
    use super::*;
    use switchback_traits::{
        EntityBody, EntityId, EntityRef, IndexedEntity, LinkTarget, ModuleId, SchemaBody,
        StoredEntity,
    };

    fn manual_with_user_entity() -> ResolvedManual {
        let entity = StoredEntity {
            name: "User".into(),
            category: "schema".into(),
            title: "User".into(),
            doc: None,
            source: None,
            refs: Vec::new(),
            intra_links: Vec::new(),
            body: EntityBody::Schema(SchemaBody {
                fence_language: "protobuf".into(),
                fence_body: "message User {}".into(),
                payload_format: String::new(),
                properties: Vec::new(),
            }),
        };
        ResolvedManual::new(vec![IndexedEntity {
            module_id: ModuleId::from("acme.example.v1"),
            contract_family: "protobuf".into(),
            group_id: "acme.example.v1".into(),
            entity,
        }])
    }

    #[test]
    fn extracts_resolvable_fqn_in_doc() {
        let manual = manual_with_user_entity();
        let entity = Entity {
            id: EntityId::new("acme.example.v1", "schema", "Pet"),
            category: ProtobufCategory::Schema,
            title: "Pet".into(),
            doc: Some("See acme.example.v1.User for details.".into()),
            source_span: None,
            body: EntityBody::Schema(SchemaBody::default()),
        };
        let links = ProtobufLinkExtractor.extract(&entity, &manual);
        assert_eq!(links.len(), 1);
        assert!(matches!(
            links[0].target,
            LinkTarget::Entity(EntityRef { ref name, .. }) if name == "User"
        ));
    }

    #[test]
    fn same_group_beats_cross_group_lookup() {
        let manual = manual_with_user_entity();
        let entity = Entity {
            id: EntityId::new("acme.example.v1", "schema", "Pet"),
            category: ProtobufCategory::Schema,
            title: "Pet".into(),
            doc: Some("Prefers acme.example.v1.User over homonyms.".into()),
            source_span: None,
            body: EntityBody::Schema(SchemaBody::default()),
        };
        let links = ProtobufLinkExtractor.extract(&entity, &manual);
        assert_eq!(links.len(), 1);
        assert_eq!(
            links[0].target,
            LinkTarget::Entity(EntityRef {
                module: "acme.example.v1".into(),
                group: "acme.example.v1".into(),
                category: "schema".into(),
                name: "User".into(),
            })
        );
    }
}
