//! Layout-aware output paths (protobuf-mdbook parity).

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::options::Layout;

/// Protobuf entity kind for layout path indexing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ProtobufEntityKind {
    /// Protobuf message type.
    Message,
    /// Protobuf enum type.
    Enum,
    /// Protobuf service definition.
    Service,
}

/// Layout entity key for path indexing (protobuf package + kind + name).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LayoutEntityKey {
    /// Protobuf package / group id string.
    pub package: String,
    /// Message, enum, or service kind.
    pub kind: ProtobufEntityKind,
    /// Entity name within the package.
    pub name: String,
}

/// Relative path to a package rollup page under `markdown_root`.
pub fn package_page_rel(markdown_root: &str, package: &str) -> PathBuf {
    PathBuf::from(format!("{markdown_root}/{package}.md"))
}

/// Relative path to a package index page (entity/split layouts).
pub fn package_index_rel(layout: Layout, markdown_root: &str, package: &str) -> PathBuf {
    match layout {
        Layout::Package => package_page_rel(markdown_root, package),
        Layout::Entity | Layout::Split => PathBuf::from(format!(
            "{markdown_root}/{}/index.md",
            package.replace('.', "/")
        )),
    }
}

/// Relative entity page path for entity/split layouts.
pub fn layout_entity_rel_path(
    layout: Layout,
    markdown_root: &str,
    key: &LayoutEntityKey,
) -> PathBuf {
    let pkg_file = key.package.replace('.', "/");
    match layout {
        Layout::Package => package_page_rel(markdown_root, &key.package),
        Layout::Entity | Layout::Split => PathBuf::from(match key.kind {
            ProtobufEntityKind::Message => {
                format!("{markdown_root}/{pkg_file}/messages/{}.md", key.name)
            }
            ProtobufEntityKind::Enum => {
                format!("{markdown_root}/{pkg_file}/enums/{}.md", key.name)
            }
            ProtobufEntityKind::Service => {
                format!("{markdown_root}/{pkg_file}/services/{}.md", key.name)
            }
        }),
    }
}

/// Heading anchor id compatible with mdBook HTML output.
pub fn heading_slug(name: &str) -> String {
    id_from_content(name)
}

/// Assigns mdBook-style unique heading ids in document order (appends `-1`, `-2`, … on collision).
pub fn unique_heading_ids(titles: impl IntoIterator<Item = impl AsRef<str>>) -> Vec<String> {
    let mut used = HashSet::new();
    titles
        .into_iter()
        .map(|title| {
            let base = id_from_content(title.as_ref());
            unique_id(&base, &mut used)
        })
        .collect()
}

fn unique_id(id: &str, used: &mut HashSet<String>) -> String {
    if used.insert(id.to_string()) {
        return id.to_string();
    }
    let mut counter = 1u32;
    loop {
        let candidate = format!("{id}-{counter}");
        if used.insert(candidate.clone()) {
            return candidate;
        }
        counter += 1;
    }
}

/// Relative POSIX path from `from_dir` to `target` (mdBook link form).
pub fn relative_path_from_dir(from_dir: &Path, target: &Path) -> String {
    let from_parts: Vec<_> = from_dir.components().collect();
    let target_parts: Vec<_> = target.components().collect();
    let mut i = 0;
    while i < from_parts.len() && i < target_parts.len() && from_parts[i] == target_parts[i] {
        i += 1;
    }
    let ups = from_parts.len().saturating_sub(i);
    let mut parts: Vec<String> = (0..ups).map(|_| "..".to_string()).collect();
    for c in &target_parts[i..] {
        parts.push(c.as_os_str().to_string_lossy().into_owned());
    }
    if parts.is_empty() {
        target
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned()
    } else {
        parts.join("/")
    }
}

fn id_from_content(content: &str) -> String {
    content
        .trim()
        .to_lowercase()
        .chars()
        .filter_map(|ch| {
            if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                Some(ch)
            } else if ch.is_whitespace() {
                Some('-')
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mdbook_slug_for_pascal_case_message() {
        assert_eq!(
            heading_slug("GetOrganizationsResponse"),
            "getorganizationsresponse"
        );
    }

    #[test]
    fn unique_heading_ids_deduplicates_collisions() {
        let ids = unique_heading_ids(["EchoRequest", "EchoResponse", "EchoRequest"]);
        assert_eq!(ids, ["echorequest", "echoresponse", "echorequest-1"]);
    }

    #[test]
    fn id_from_content_matches_mdbook_behavior() {
        let cases = [
            ("GetOrganizationsResponse", "getorganizationsresponse"),
            ("中文標題 CJK title", "中文標題-cjk-title"),
            ("_-_12345", "_-_12345"),
        ];
        for (input, expected) in cases {
            assert_eq!(id_from_content(input), expected, "input: {input:?}");
        }
    }

    #[test]
    fn entity_split_path() {
        let key = LayoutEntityKey {
            package: "acme.v1".into(),
            kind: ProtobufEntityKind::Message,
            name: "Pet".into(),
        };
        assert_eq!(
            layout_entity_rel_path(Layout::Entity, "src/packages", &key),
            PathBuf::from("src/packages/acme/v1/messages/Pet.md")
        );
    }
}
