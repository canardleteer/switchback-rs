//! Beside companion markdown discovery for OpenAPI entry files.

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
        let bytes = std::fs::read(&companion_path).map_err(|e| {
            SwitchbackError::load(format!("read companion {}: {e}", companion_path.display()))
        })?;
        let title = title_from_markdown(stem, &bytes);
        let rel_dir = parent.strip_prefix(module_root).unwrap_or(parent);
        let source_dir = switchback_traits::source_dir_string(rel_dir);
        let output_name = companion_output_name_from_path(rel_dir, stem);
        out.push(CompanionFile {
            output_name,
            bytes,
            source_path: companion_path,
            title,
            source_dir,
            stem: stem.to_string(),
        });
    }

    Ok(out)
}
