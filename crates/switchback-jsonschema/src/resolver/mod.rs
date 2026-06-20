//! `$ref` resolution and cross-document index.

mod pointer;

use std::collections::{BTreeMap, BTreeSet};

use anyhow::{anyhow, Context};
use serde_json::Value;

pub use pointer::{escape_token, resolve_pointer};

use crate::loader::Doc;

/// Logical address of a node in the resolved document graph.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeRef {
    pub doc_uri: String,
    pub pointer: String,
}

impl NodeRef {
    pub fn root(doc_uri: impl Into<String>) -> Self {
        Self {
            doc_uri: doc_uri.into(),
            pointer: String::new(),
        }
    }

    pub fn with_pointer(doc_uri: impl Into<String>, pointer: impl Into<String>) -> Self {
        Self {
            doc_uri: doc_uri.into(),
            pointer: pointer.into(),
        }
    }
}

/// Index of resolvable `$ref` targets across loaded documents.
#[derive(Clone, Debug, Default)]
pub struct RefIndex {
    pub nodes: BTreeMap<NodeRef, Value>,
    pub ref_targets: BTreeMap<NodeRef, NodeRef>,
}

/// Resolve `$ref` chains and build a cross-document index.
#[derive(Clone, Debug, Default)]
pub struct RefResolver;

impl RefResolver {
    pub fn resolve(docs: &[Doc]) -> anyhow::Result<RefIndex> {
        let mut index = RefIndex::default();
        let doc_map: BTreeMap<&str, &Doc> = docs.iter().map(|d| (d.uri.as_str(), d)).collect();

        for doc in docs {
            walk_value(
                &doc.value,
                &doc.uri,
                String::new(),
                &doc_map,
                &mut index,
                &mut BTreeSet::new(),
            )?;
        }
        Ok(index)
    }
}

fn walk_value(
    value: &Value,
    doc_uri: &str,
    pointer: String,
    docs: &BTreeMap<&str, &Doc>,
    index: &mut RefIndex,
    stack: &mut BTreeSet<NodeRef>,
) -> anyhow::Result<()> {
    let here = NodeRef {
        doc_uri: doc_uri.to_string(),
        pointer: pointer.clone(),
    };

    if let Value::Object(map) = value {
        if let Some(Value::String(ref_key)) = map.get("$ref") {
            let target = resolve_ref(ref_key, doc_uri, docs, stack)?;
            index.ref_targets.insert(here.clone(), target.clone());
            if let Some(target_value) = index.nodes.get(&target) {
                index.nodes.insert(here.clone(), target_value.clone());
            }
            return Ok(());
        }
    }

    index.nodes.insert(here.clone(), value.clone());
    if !stack.insert(here) {
        return Ok(());
    }

    match value {
        Value::Object(map) => {
            for (key, child) in map {
                if key == "$ref" {
                    continue;
                }
                let mut child_pointer = pointer.clone();
                if !child_pointer.is_empty() {
                    child_pointer.push('/');
                }
                child_pointer.push_str(&escape_token(key));
                walk_value(child, doc_uri, child_pointer, docs, index, stack)?;
            }
        }
        Value::Array(items) => {
            for (i, child) in items.iter().enumerate() {
                let child_pointer = if pointer.is_empty() {
                    i.to_string()
                } else {
                    format!("{pointer}/{i}")
                };
                walk_value(child, doc_uri, child_pointer, docs, index, stack)?;
            }
        }
        _ => {}
    }

    stack.remove(&NodeRef {
        doc_uri: doc_uri.to_string(),
        pointer,
    });
    Ok(())
}

fn resolve_ref(
    ref_key: &str,
    base_doc_uri: &str,
    docs: &BTreeMap<&str, &Doc>,
    stack: &BTreeSet<NodeRef>,
) -> anyhow::Result<NodeRef> {
    if ref_key.starts_with("http://") || ref_key.starts_with("https://") {
        return Ok(NodeRef {
            doc_uri: base_doc_uri.to_string(),
            pointer: format!("$external:{ref_key}"),
        });
    }

    let (file_part, pointer) = split_ref(ref_key);
    let doc_uri = if file_part.is_empty() {
        base_doc_uri.to_string()
    } else {
        let base_doc = docs
            .get(base_doc_uri)
            .with_context(|| format!("unknown base document {base_doc_uri}"))?;
        let file_part = crate::paths::strip_dot_slash(file_part);
        let candidate = crate::paths::normalize_path(
            &base_doc
                .path
                .parent()
                .unwrap_or(&base_doc.path)
                .join(file_part),
        );
        docs.values()
            .find(|d| crate::paths::normalize_path(&d.path) == candidate)
            .map(|d| d.uri.clone())
            .ok_or_else(|| anyhow!("unresolved external $ref file: {file_part}"))?
    };

    let target = NodeRef {
        doc_uri,
        pointer: normalize_pointer(pointer),
    };

    if stack.contains(&target) {
        return Ok(NodeRef {
            doc_uri: target.doc_uri,
            pointer: format!("{}#recursive", target.pointer),
        });
    }

    Ok(target)
}

fn split_ref(ref_key: &str) -> (&str, &str) {
    if let Some((file, ptr)) = ref_key.split_once('#') {
        (file, ptr)
    } else if ref_key.starts_with('#') {
        ("", ref_key.trim_start_matches('#'))
    } else {
        (ref_key, "")
    }
}

fn normalize_pointer(pointer: &str) -> String {
    pointer.trim_start_matches('/').to_string()
}
