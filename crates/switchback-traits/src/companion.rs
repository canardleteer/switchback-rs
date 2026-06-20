//! Shared companion discovery helpers and nav metadata parsing.

use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

use crate::traits::CompanionStrategy;
use crate::{CompanionFile, Result, SwitchbackError};

/// Dot-separated output filename for a companion under `source_dir` segments.
pub fn companion_output_name_from_segments(source_dir: &[&str], stem: &str) -> String {
    if source_dir.is_empty() {
        format!("{stem}.md")
    } else {
        format!("{}.{}.md", source_dir.join("."), stem)
    }
}

/// Dot-separated output filename from a relative directory path and stem.
pub fn companion_output_name_from_path(rel_dir: &Path, stem: &str) -> String {
    let segments: Vec<String> = path_segments(rel_dir);
    let segment_refs: Vec<&str> = segments.iter().map(String::as_str).collect();
    companion_output_name_from_segments(&segment_refs, stem)
}

/// Normal path components for a relative directory (drops `.` and `..`).
pub fn path_segments(rel_dir: &Path) -> Vec<String> {
    rel_dir
        .components()
        .filter_map(|c| match c {
            Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect()
}

/// Normalize a relative directory path to normal components only.
pub fn normalize_rel_dir(path: &Path) -> PathBuf {
    PathBuf::from(path_segments(path).join("/"))
}

/// Slash-separated source directory string for wire nav metadata.
pub fn source_dir_string(dir: &Path) -> String {
    path_segments(dir).join("/")
}

/// Title from the first `#` heading in markdown, else humanized stem.
pub fn title_from_markdown(stem: &str, content: &[u8]) -> String {
    let text = String::from_utf8_lossy(content);
    for line in text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix('#') {
            let title = rest.trim_start_matches('#').trim();
            if !title.is_empty() {
                return title.to_string();
            }
        }
    }
    humanize_stem(stem)
}

fn humanize_stem(stem: &str) -> String {
    if stem.eq_ignore_ascii_case("readme") {
        return "README".to_string();
    }
    stem.replace(['-', '_'], " ")
}

/// Dot-separated module path implied by a companion output filename.
///
/// `acme.example.v1.README.md` with stem `README` → `acme.example.v1`.
pub fn module_path_from_output(output_rel: &str, stem: &str) -> Option<String> {
    let base = output_rel.strip_suffix(".md")?;
    let suffix = format!(".{stem}");
    base.strip_suffix(&suffix).map(str::to_string)
}

/// Inverse of dot-encoded output name for legacy artifacts missing `source_dir`.
pub fn source_dir_from_output(output_rel: &str, stem: &str) -> String {
    module_path_from_output(output_rel, stem)
        .map(|p| p.replace('.', "/"))
        .unwrap_or_default()
}

/// Discover companion markdown by walking ancestor directories from each anchor.
pub fn discover_ancestors_companions<S: CompanionStrategy>(
    strategy: &S,
    companion_extensions: &[&str],
    anchor_dirs: &[PathBuf],
    search_roots: &[PathBuf],
) -> Result<Vec<CompanionFile>> {
    let roots = if search_roots.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        search_roots.to_vec()
    };

    let mut seen = BTreeMap::new();
    for anchor in anchor_dirs {
        let mut dir = normalize_rel_dir(anchor);
        loop {
            if !dir.as_os_str().is_empty() {
                collect_md_in_dir(strategy, companion_extensions, &dir, &roots, &mut seen)?;
            }
            if dir.as_os_str().is_empty() {
                break;
            }
            if !dir.pop() {
                break;
            }
        }
    }

    Ok(seen.into_values().collect())
}

fn collect_md_in_dir<S: CompanionStrategy>(
    strategy: &S,
    companion_extensions: &[&str],
    dir: &Path,
    search_roots: &[PathBuf],
    seen: &mut BTreeMap<String, CompanionFile>,
) -> Result<()> {
    let fs_dir = search_roots
        .iter()
        .map(|r| r.join(dir))
        .find(|p| p.is_dir());
    let Some(fs_dir) = fs_dir else {
        return Ok(());
    };

    for entry in std::fs::read_dir(&fs_dir).map_err(|e| SwitchbackError::load(e.to_string()))? {
        let entry = entry.map_err(|e| SwitchbackError::load(e.to_string()))?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
            continue;
        };
        if !companion_extensions
            .iter()
            .any(|allowed| ext.eq_ignore_ascii_case(allowed.trim_start_matches('.')))
        {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        if stem.starts_with('.') {
            continue;
        }
        let stem = stem.to_string();
        let rel_dir = normalize_rel_dir(dir);
        let segments = path_segments(&rel_dir);
        let segment_refs: Vec<&str> = segments.iter().map(String::as_str).collect();
        let output_name = strategy.output_name(&segment_refs, &stem);
        if seen.contains_key(&output_name) {
            continue;
        }
        let bytes = std::fs::read(&path).map_err(|e| SwitchbackError::load(e.to_string()))?;
        let title = strategy.companion_title(&stem, &bytes);
        seen.insert(
            output_name.clone(),
            CompanionFile {
                output_name,
                bytes,
                source_path: path,
                title,
                source_dir: source_dir_string(&rel_dir),
                stem,
            },
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CompanionDiscovery, CompanionStrategy};
    use std::fs;
    use tempfile::TempDir;

    struct TestCompanion;

    impl CompanionStrategy for TestCompanion {
        fn discovery(&self) -> CompanionDiscovery {
            CompanionDiscovery::Ancestors
        }

        fn output_name(&self, source_dir: &[&str], stem: &str) -> String {
            companion_output_name_from_segments(source_dir, stem)
        }

        fn companion_media_types(&self) -> &'static [&'static str] {
            &["text/markdown"]
        }
    }

    #[test]
    fn module_path_from_output_parses() {
        assert_eq!(
            module_path_from_output("acme.example.v1.README.md", "README"),
            Some("acme.example.v1".into())
        );
        assert_eq!(
            module_path_from_output("acme.README.md", "README"),
            Some("acme".into())
        );
    }

    #[test]
    fn title_from_markdown_uses_heading() {
        assert_eq!(title_from_markdown("README", b"# Acme APIs\n"), "Acme APIs");
        assert_eq!(
            title_from_markdown("MOVING-TO-V2", b"# Moving to v2\n"),
            "Moving to v2"
        );
    }

    #[test]
    fn discovers_intermediate_and_leaf_companions() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("acme/example/v1")).unwrap();
        fs::create_dir_all(root.join("acme/example/v2")).unwrap();
        fs::write(root.join("acme/README.md"), "# Acme\n").unwrap();
        fs::write(root.join("acme/example/README.md"), "# Example\n").unwrap();
        fs::write(root.join("acme/example/v1/README.md"), "# V1\n").unwrap();
        fs::write(root.join("acme/example/v1/MOVING-TO-V2.md"), "# Moving\n").unwrap();

        let anchors = vec![
            PathBuf::from("acme/example/v1"),
            PathBuf::from("acme/example/v2"),
        ];
        let docs =
            discover_ancestors_companions(&TestCompanion, &["md"], &anchors, &[root.to_path_buf()])
                .unwrap();
        let names: Vec<_> = docs.iter().map(|d| d.output_name.as_str()).collect();
        assert!(names.contains(&"acme.README.md"));
        assert!(names.contains(&"acme.example.README.md"));
        assert!(names.contains(&"acme.example.v1.README.md"));
        assert!(names.contains(&"acme.example.v1.MOVING-TO-V2.md"));
        let acme = docs
            .iter()
            .find(|d| d.output_name == "acme.README.md")
            .unwrap();
        assert_eq!(acme.title, "Acme");
        assert_eq!(acme.source_dir, "acme");
        assert_eq!(acme.stem, "README");
    }

    #[test]
    fn partial_inputs_skip_other_branch() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("a/b/c/d/e/f/g/h/v1")).unwrap();
        fs::create_dir_all(root.join("a/b/c/d/e/f/g/h/v2")).unwrap();
        fs::write(root.join("a/b/NOTES.md"), "# Notes\n").unwrap();
        fs::write(root.join("a/b/c/d/e/f/g/h/v2/more-notes.md"), "# More\n").unwrap();

        let anchors = vec![PathBuf::from("a/b/c/d/e/f/g/h/v1")];
        let docs =
            discover_ancestors_companions(&TestCompanion, &["md"], &anchors, &[root.to_path_buf()])
                .unwrap();
        let names: Vec<_> = docs.iter().map(|d| d.output_name.as_str()).collect();
        assert!(names.contains(&"a.b.NOTES.md"));
        assert!(!names.iter().any(|n| n.contains("more-notes")));
    }
}
