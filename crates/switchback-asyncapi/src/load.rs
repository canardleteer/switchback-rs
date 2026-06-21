//! Load AsyncAPI inputs into a switchback [`ReferenceManual`].

use std::path::{Path, PathBuf};

use switchback_jsonschema::resolve_inputs;
use switchback_traits::ReferenceManual;

use crate::manual::build_reference_manual;
use crate::populate::{ResolvedInput, populate};

/// Arguments for [`load`].
#[derive(Clone, Debug)]
pub struct LoadArgs {
    pub module_root: PathBuf,
    pub inputs: Vec<PathBuf>,
    pub search_roots: Vec<PathBuf>,
    pub title: Option<String>,
}

impl LoadArgs {
    pub fn examples(module_root: impl Into<PathBuf>, inputs: &[&str]) -> Self {
        Self {
            module_root: module_root.into(),
            inputs: inputs.iter().map(|p| PathBuf::from(*p)).collect(),
            search_roots: Vec::new(),
            title: None,
        }
    }
}

/// Parse AsyncAPI inputs and return a populated reference manual.
pub fn load(args: &LoadArgs) -> switchback_traits::Result<ReferenceManual> {
    let module_root = args.module_root.clone();
    let inputs: Vec<PathBuf> = args
        .inputs
        .iter()
        .map(|p| {
            if p.is_relative() {
                module_root.join(p)
            } else {
                p.clone()
            }
        })
        .collect();

    let search_roots = if args.search_roots.is_empty() {
        vec![module_root.clone()]
    } else {
        args.search_roots.clone()
    };

    let entry_uris = entry_uris_for_inputs(&module_root, &inputs)?;
    let resolved = resolve_inputs(&module_root, &inputs, &search_roots)?;
    let populated_input = ResolvedInput {
        module_root: module_root.clone(),
        entry_uris,
        docs: resolved.docs,
        index: resolved.index,
    };

    let populated = populate(&populated_input)?;
    build_reference_manual(populated, &populated_input, args.title.clone())
}

fn entry_uris_for_inputs(
    module_root: &Path,
    inputs: &[PathBuf],
) -> switchback_traits::Result<Vec<String>> {
    let canonical_root = module_root
        .canonicalize()
        .unwrap_or_else(|_| module_root.to_path_buf());
    let mut uris = Vec::new();
    for input in inputs {
        let abs = if input.is_absolute() {
            input.clone()
        } else {
            module_root.join(input)
        };
        if abs.is_file() {
            if is_asyncapi_file(&abs) {
                uris.push(relativize(&canonical_root, &abs)?);
            }
        } else if abs.is_dir() {
            for entry in walkdir::WalkDir::new(&abs)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() && is_asyncapi_file(path) {
                    uris.push(relativize(&canonical_root, path)?);
                }
            }
        }
    }
    uris.sort();
    uris.dedup();
    Ok(uris)
}

fn is_asyncapi_file(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some("yaml" | "yml" | "json") => {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str())
                && matches!(stem, "asyncapi")
            {
                return true;
            }
            looks_like_asyncapi(path)
        }
        _ => false,
    }
}

fn looks_like_asyncapi(path: &Path) -> bool {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(_) => return false,
    };
    let value: Option<serde_json::Value> = if path
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| matches!(ext, "yaml" | "yml"))
    {
        serde_saphyr::from_slice(&bytes).ok()
    } else {
        serde_json::from_slice(&bytes).ok()
    };
    value.is_some_and(|v| v.get("asyncapi").is_some())
}

fn relativize(module_root: &Path, path: &Path) -> switchback_traits::Result<String> {
    let canonical = path
        .canonicalize()
        .map_err(|e| switchback_traits::SwitchbackError::load(e.to_string()))?;
    canonical
        .strip_prefix(module_root)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .map_err(|_| {
            switchback_traits::SwitchbackError::load(format!(
                "path {} outside module root {}",
                path.display(),
                module_root.display()
            ))
        })
}
