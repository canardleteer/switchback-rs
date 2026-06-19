//! Companion markdown discovery beside protobuf inputs.

use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

use switchback_traits::{CompanionDiscovery, CompanionFile, CompanionStrategy, ContractFamily};

use crate::descriptor::FileDescriptorProto;
use crate::family::ProtobufFamily;
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
    proto_files: &[FileDescriptorProto],
    file_to_generate: &[String],
    search_roots: &[PathBuf],
    _module_root: &Path,
) -> switchback_traits::Result<Vec<CompanionFile>> {
    let strategy = ProtobufFamily.companion_strategy();
    if !matches!(strategy.discovery(), CompanionDiscovery::Ancestors) {
        return Ok(Vec::new());
    }

    let roots = if search_roots.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        search_roots.to_vec()
    };

    let proto_dirs = collect_proto_dirs(proto_files, file_to_generate, &roots)?;
    if proto_dirs.is_empty() {
        return Ok(Vec::new());
    }

    let mut seen = BTreeMap::new();
    for proto_dir in &proto_dirs {
        let mut dir = proto_dir.clone();
        loop {
            if !dir.as_os_str().is_empty() {
                collect_md_in_dir(&dir, &roots, &mut seen)?;
            }
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

fn collect_proto_dirs(
    proto_files: &[FileDescriptorProto],
    file_to_generate: &[String],
    search_roots: &[PathBuf],
) -> switchback_traits::Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();
    for name in file_to_generate {
        let rel = Path::new(name);
        if rel.components().any(|c| matches!(c, Component::ParentDir)) {
            continue;
        }
        let parent = rel.parent().unwrap_or(Path::new("")).to_path_buf();
        let _file = proto_files
            .iter()
            .find(|f| f.name.as_deref() == Some(name.as_str()));
        let resolved = resolve_proto_dir(&parent, search_roots);
        dirs.push(resolved.unwrap_or(parent));
    }
    dirs.sort();
    dirs.dedup();
    Ok(dirs)
}

fn resolve_proto_dir(rel_dir: &Path, search_roots: &[PathBuf]) -> Option<PathBuf> {
    for root in search_roots {
        let candidate = root.join(rel_dir);
        if candidate.is_dir() {
            return Some(normalize_rel_dir(rel_dir));
        }
    }
    Some(normalize_rel_dir(rel_dir))
}

fn collect_md_in_dir(
    dir: &Path,
    search_roots: &[PathBuf],
    seen: &mut BTreeMap<String, DiscoveredCompanion>,
) -> switchback_traits::Result<()> {
    let fs_dir = search_roots
        .iter()
        .map(|r| r.join(dir))
        .find(|p| p.is_dir());
    let Some(fs_dir) = fs_dir else {
        return Ok(());
    };

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
