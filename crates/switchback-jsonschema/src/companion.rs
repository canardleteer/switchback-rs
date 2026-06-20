//! Companion markdown discovery beside schema inputs.

use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

use switchback_traits::{CompanionDiscovery, CompanionFile, CompanionStrategy, ContractFamily};

use crate::family::JsonSchemaFamily;
use crate::paths::normalize_rel_dir;

pub fn companion_output_name_from_segments(source_dir: &[&str], stem: &str) -> String {
    if source_dir.is_empty() {
        format!("{stem}.md")
    } else {
        format!("{}.{}.md", source_dir.join("."), stem)
    }
}

pub fn companion_output_name(rel_dir: &Path, stem: &str) -> String {
    let segments: Vec<String> = rel_dir
        .components()
        .filter_map(|c| match c {
            Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect();
    let segment_refs: Vec<&str> = segments.iter().map(String::as_str).collect();
    companion_output_name_from_segments(&segment_refs, stem)
}

pub fn discover_companions(
    _docs: &[crate::loader::Doc],
    entry_uris: &[String],
    module_root: &Path,
) -> switchback_traits::Result<Vec<CompanionFile>> {
    let strategy = JsonSchemaFamily.companion_strategy();
    if !matches!(strategy.discovery(), CompanionDiscovery::Ancestors) {
        return Ok(Vec::new());
    }

    let mut seen = BTreeMap::new();
    for uri in entry_uris {
        let abs = module_root.join(uri);
        let Some(parent) = abs.parent() else {
            continue;
        };
        let rel_parent = parent
            .strip_prefix(module_root)
            .map(normalize_rel_dir)
            .unwrap_or_else(|_| normalize_rel_dir(parent));
        let mut dir = rel_parent;
        loop {
            collect_md_in_dir(&dir, module_root, &mut seen)?;
            if dir.as_os_str().is_empty() {
                break;
            }
            if !dir.pop() {
                break;
            }
        }
    }

    Ok(seen
        .into_values()
        .map(|doc| CompanionFile {
            output_name: doc.output_name,
            bytes: doc.bytes,
            source_path: doc.source_path,
        })
        .collect())
}

struct DiscoveredCompanion {
    output_name: String,
    bytes: Vec<u8>,
    source_path: PathBuf,
}

fn collect_md_in_dir(
    dir: &Path,
    module_root: &Path,
    seen: &mut BTreeMap<String, DiscoveredCompanion>,
) -> switchback_traits::Result<()> {
    let fs_dir = module_root.join(dir);
    if !fs_dir.is_dir() {
        return Ok(());
    }

    for entry in std::fs::read_dir(&fs_dir)
        .map_err(|e| switchback_traits::SwitchbackError::load(e.to_string()))?
    {
        let entry = entry.map_err(|e| switchback_traits::SwitchbackError::load(e.to_string()))?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        if stem.starts_with('.') {
            continue;
        }
        let rel_dir = normalize_rel_dir(dir);
        let output_name = companion_output_name(&rel_dir, stem);
        if seen.contains_key(&output_name) {
            continue;
        }
        let bytes = std::fs::read(&path)
            .map_err(|e| switchback_traits::SwitchbackError::load(e.to_string()))?;
        seen.insert(
            output_name.clone(),
            DiscoveredCompanion {
                output_name,
                bytes,
                source_path: path,
            },
        );
    }
    Ok(())
}
