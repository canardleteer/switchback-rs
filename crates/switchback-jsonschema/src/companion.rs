//! Companion markdown discovery beside schema inputs.

use std::path::Path;

use switchback_traits::{
    CompanionDiscovery, CompanionStrategy, ContractFamily, discover_ancestors_companions,
    normalize_rel_dir,
};

use crate::family::JsonSchemaFamily;

pub use switchback_traits::companion_output_name_from_segments;

pub fn companion_output_name(rel_dir: &Path, stem: &str) -> String {
    switchback_traits::companion_output_name_from_path(rel_dir, stem)
}

pub fn discover_companions(
    _docs: &[crate::loader::Doc],
    entry_uris: &[String],
    module_root: &Path,
) -> switchback_traits::Result<Vec<switchback_traits::CompanionFile>> {
    let strategy = JsonSchemaFamily.companion_strategy();
    if !matches!(strategy.discovery(), CompanionDiscovery::Ancestors) {
        return Ok(Vec::new());
    }

    let mut anchors = Vec::new();
    for uri in entry_uris {
        let abs = module_root.join(uri);
        let Some(parent) = abs.parent() else {
            continue;
        };
        let rel_parent = parent
            .strip_prefix(module_root)
            .map(normalize_rel_dir)
            .unwrap_or_else(|_| normalize_rel_dir(parent));
        anchors.push(rel_parent);
    }
    anchors.sort();
    anchors.dedup();

    discover_ancestors_companions(strategy, &["md"], &anchors, &[module_root.to_path_buf()])
}
