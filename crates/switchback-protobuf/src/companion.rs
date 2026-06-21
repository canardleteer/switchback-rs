//! Companion markdown discovery beside protobuf inputs.

use std::path::{Component, Path, PathBuf};

use switchback_traits::{
    CompanionDiscovery, CompanionStrategy, ContractFamily, discover_ancestors_companions,
    normalize_rel_dir,
};

use crate::descriptor::FileDescriptorProto;
use crate::family::ProtobufFamily;

pub use switchback_traits::companion_output_name_from_segments;

pub fn companion_output_name(rel_dir: &Path, stem: &str) -> String {
    switchback_traits::companion_output_name_from_path(rel_dir, stem)
}

pub fn discover_companions(
    proto_files: &[FileDescriptorProto],
    file_to_generate: &[String],
    search_roots: &[PathBuf],
    _module_root: &Path,
) -> switchback_traits::Result<Vec<switchback_traits::CompanionFile>> {
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

    discover_ancestors_companions(strategy, &["md"], &proto_dirs, &roots)
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
