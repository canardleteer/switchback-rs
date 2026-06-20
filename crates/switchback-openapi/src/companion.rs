//! Beside companion markdown discovery for OpenAPI entry files.

use std::collections::BTreeSet;
use std::path::Path;

use switchback_traits::{
    companion_output_name_from_path, title_from_markdown, CompanionFile, ContractFamily, Result,
    SwitchbackError,
};

use crate::family::OpenApiFamily;

pub fn discover_companions(
    entry_uris: &[String],
    module_root: &Path,
) -> Result<Vec<CompanionFile>> {
    let _strategy = OpenApiFamily.companion_strategy();
    let mut out = Vec::new();

    for uri in entry_uris {
        let abs = module_root.join(uri);
        let Some(parent) = abs.parent() else {
            continue;
        };
        let stem = abs
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("openapi");
        let companion_path = parent.join(format!("{stem}.md"));
        if !companion_path.is_file() {
            continue;
        }
        push_companion(&mut out, module_root, &companion_path, stem)?;
    }

    Ok(out)
}

/// Discover beside companions plus any `.md` under `module_root` (multi-entry corpora).
pub fn discover_companions_multi(
    entry_uris: &[String],
    module_root: &Path,
) -> Result<Vec<CompanionFile>> {
    let mut out = discover_companions(entry_uris, module_root)?;
    let mut seen: BTreeSet<String> = out.iter().map(|c| c.output_name.clone()).collect();

    for entry in walkdir::WalkDir::new(module_root)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let rel_dir = path
            .parent()
            .unwrap_or(module_root)
            .strip_prefix(module_root)
            .unwrap_or(path);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("companion");
        let output_name = companion_output_name_from_path(rel_dir, stem);
        if !seen.insert(output_name) {
            continue;
        }
        push_companion(&mut out, module_root, path, stem)?;
    }

    Ok(out)
}

fn push_companion(
    out: &mut Vec<CompanionFile>,
    module_root: &Path,
    companion_path: &Path,
    stem: &str,
) -> Result<()> {
    let bytes = std::fs::read(companion_path).map_err(|e| {
        SwitchbackError::load(format!("read companion {}: {e}", companion_path.display()))
    })?;
    let title = title_from_markdown(stem, &bytes);
    let rel_dir = companion_path
        .parent()
        .unwrap_or(module_root)
        .strip_prefix(module_root)
        .unwrap_or(companion_path.parent().unwrap_or(module_root));
    let source_dir = switchback_traits::source_dir_string(rel_dir);
    let output_name = companion_output_name_from_path(rel_dir, stem);
    out.push(CompanionFile {
        output_name,
        bytes,
        source_path: companion_path.to_path_buf(),
        title,
        source_dir,
        stem: stem.to_string(),
    });
    Ok(())
}
