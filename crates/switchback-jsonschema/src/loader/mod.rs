//! Read YAML/JSON documents and collect transitively referenced files.

mod doc;

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context};
use serde_json::Value;
use walkdir::WalkDir;

pub use doc::Doc;

/// Collect entry and referenced documents from disk.
#[derive(Clone, Debug)]
pub struct Loader {
    pub roots: Vec<PathBuf>,
    pub allow_url_refs: bool,
}

impl Loader {
    pub fn new(roots: Vec<PathBuf>) -> Self {
        Self {
            roots,
            allow_url_refs: false,
        }
    }

    pub fn with_url_refs(mut self, allow: bool) -> Self {
        self.allow_url_refs = allow;
        self
    }

    /// Load entry paths (files or directories) into a deduplicated document set.
    pub fn load_entries(
        &self,
        module_root: &Path,
        entries: &[PathBuf],
    ) -> anyhow::Result<Vec<Doc>> {
        let entry_files = collect_entry_files(entries, module_root)?;
        let mut docs_by_uri: BTreeMap<String, Doc> = BTreeMap::new();
        let mut queue: VecDeque<PathBuf> = entry_files.into_iter().collect();
        let mut seen_paths: BTreeSet<PathBuf> = BTreeSet::new();

        while let Some(abs_path) = queue.pop_front() {
            let canonical = abs_path
                .canonicalize()
                .with_context(|| format!("canonicalize {}", abs_path.display()))?;
            if !seen_paths.insert(canonical.clone()) {
                continue;
            }

            let uri = relativize_uri(module_root, &canonical)?;
            if docs_by_uri.contains_key(&uri) {
                continue;
            }

            let doc = read_doc(&uri, &canonical)?;
            collect_external_ref_targets(
                &doc.value,
                &doc.path,
                module_root,
                &self.roots,
                &mut queue,
            );
            docs_by_uri.insert(uri, doc);
        }

        Ok(docs_by_uri.into_values().collect())
    }
}

/// Output of loading and resolving `$ref`s.
#[derive(Clone, Debug)]
pub struct Resolved {
    pub docs: Vec<Doc>,
    pub index: crate::resolver::RefIndex,
}

const SCHEMA_EXTENSIONS: &[&str] = &["yaml", "yml", "json"];

fn collect_entry_files(entries: &[PathBuf], module_root: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = BTreeSet::new();
    for entry in entries {
        let abs = if entry.is_absolute() {
            entry.clone()
        } else {
            module_root.join(entry)
        };
        if abs.is_dir() {
            for result in WalkDir::new(&abs).into_iter().filter_map(|e| e.ok()) {
                let path = result.path();
                if path.is_file() && is_schema_file(path) {
                    files.insert(path.to_path_buf());
                }
            }
        } else if abs.is_file() {
            if is_schema_file(&abs) {
                files.insert(abs);
            } else {
                return Err(anyhow!("not a schema file: {}", abs.display()));
            }
        } else {
            return Err(anyhow!("input not found: {}", abs.display()));
        }
    }
    Ok(files.into_iter().collect())
}

fn is_schema_file(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => SCHEMA_EXTENSIONS.contains(&ext),
        None => path.is_file(),
    }
}

fn relativize_uri(module_root: &Path, abs_path: &Path) -> anyhow::Result<String> {
    let rel = abs_path
        .strip_prefix(
            module_root
                .canonicalize()
                .unwrap_or_else(|_| module_root.to_path_buf()),
        )
        .or_else(|_| abs_path.strip_prefix(module_root))
        .with_context(|| {
            format!(
                "path {} is outside module root {}",
                abs_path.display(),
                module_root.display()
            )
        })?;
    Ok(rel.to_string_lossy().replace('\\', "/"))
}

fn read_doc(uri: &str, path: &Path) -> anyhow::Result<Doc> {
    let raw_bytes = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let (value, is_yaml, media_type) = match ext {
        "yaml" | "yml" => {
            let value: Value = serde_saphyr::from_slice(&raw_bytes)
                .with_context(|| format!("parse YAML {}", path.display()))?;
            (value, true, "application/yaml".to_string())
        }
        "json" | "" => {
            let value: Value = serde_json::from_slice(&raw_bytes)
                .with_context(|| format!("parse JSON {}", path.display()))?;
            (value, false, "application/json".to_string())
        }
        _ => return Err(anyhow!("unsupported extension for {}", path.display())),
    };
    Ok(Doc {
        uri: uri.to_string(),
        path: path.to_path_buf(),
        raw_bytes,
        value,
        is_yaml,
        media_type,
    })
}

fn collect_external_ref_targets(
    value: &Value,
    base_path: &Path,
    module_root: &Path,
    search_roots: &[PathBuf],
    queue: &mut VecDeque<PathBuf>,
) {
    match value {
        Value::Object(map) => {
            if let Some(Value::String(ref_key)) = map.get("$ref") {
                if let Some(file_part) = external_file_part(ref_key) {
                    if let Some(resolved) =
                        resolve_external_path(base_path, module_root, search_roots, file_part)
                    {
                        queue.push_back(resolved);
                    }
                }
            }
            for v in map.values() {
                collect_external_ref_targets(v, base_path, module_root, search_roots, queue);
            }
        }
        Value::Array(items) => {
            for v in items {
                collect_external_ref_targets(v, base_path, module_root, search_roots, queue);
            }
        }
        _ => {}
    }
}

fn external_file_part(ref_key: &str) -> Option<&str> {
    if ref_key.starts_with('#') || ref_key.starts_with("http://") || ref_key.starts_with("https://")
    {
        return None;
    }
    ref_key.split('#').next().filter(|s| !s.is_empty())
}

fn resolve_external_path(
    base_path: &Path,
    module_root: &Path,
    search_roots: &[PathBuf],
    file_part: &str,
) -> Option<PathBuf> {
    let file_part = crate::paths::strip_dot_slash(file_part);
    let rel = PathBuf::from(file_part);
    let candidate = if rel.is_absolute() {
        rel.clone()
    } else {
        base_path.parent().unwrap_or(base_path).join(&rel)
    };
    if candidate.is_file() {
        return Some(candidate);
    }
    for root in search_roots {
        let try_path = root.join(&rel);
        if try_path.is_file() {
            return Some(try_path);
        }
    }
    let try_module = module_root.join(&rel);
    if try_module.is_file() {
        return Some(try_module);
    }
    None
}
