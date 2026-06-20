//! Load JSON Schema inputs into a switchback [`ReferenceManual`].

use std::path::{Path, PathBuf};

use switchback_traits::ReferenceManual;

use crate::loader::{Loader, Resolved};
use crate::manual::build_reference_manual;
use crate::populate::{populate, ResolvedInput};
use crate::resolver::RefResolver;

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

/// Parse JSON Schema catalog inputs and return a populated reference manual.
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
    let loader = Loader::new(search_roots);
    let docs = loader
        .load_entries(&module_root, &inputs)
        .map_err(|e| switchback_traits::SwitchbackError::load(e.to_string()))?;
    let index = RefResolver::resolve(&docs)
        .map_err(|e| switchback_traits::SwitchbackError::load(e.to_string()))?;

    let resolved = ResolvedInput {
        module_root,
        entry_uris,
        docs,
        index,
    };

    let populated = populate(&resolved)?;
    build_reference_manual(populated, &resolved, args.title.clone())
}

/// Resolve and index documents without populating a manual (shared-layer API).
pub fn resolve_inputs(
    module_root: &Path,
    inputs: &[PathBuf],
    search_roots: &[PathBuf],
) -> switchback_traits::Result<Resolved> {
    let roots = if search_roots.is_empty() {
        vec![module_root.to_path_buf()]
    } else {
        search_roots.to_vec()
    };
    let loader = Loader::new(roots);
    let docs = loader
        .load_entries(module_root, inputs)
        .map_err(|e| switchback_traits::SwitchbackError::load(e.to_string()))?;
    let index = RefResolver::resolve(&docs)
        .map_err(|e| switchback_traits::SwitchbackError::load(e.to_string()))?;
    Ok(Resolved { docs, index })
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
        if abs.is_dir() {
            for entry in walkdir::WalkDir::new(&abs)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() && is_schema_file(path) {
                    uris.push(relativize(&canonical_root, path)?);
                }
            }
        } else if abs.is_file() {
            uris.push(relativize(&canonical_root, &abs)?);
        }
    }
    uris.sort();
    uris.dedup();
    Ok(uris)
}

fn is_schema_file(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => matches!(ext, "yaml" | "yml" | "json"),
        None => path.is_file(),
    }
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
