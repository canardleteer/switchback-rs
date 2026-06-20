//! Format-agnostic path helpers for entity output layout.

/// Relative output path for an entity page under a group's markdown tree.
///
/// Shape: `{group_dir}/{category_dir}/{name}.md` using forward slashes.
/// Does not include [`Options::markdown_root`](crate::Options::markdown_root).
///
/// Full slug parity with protobuf-mdbook is deferred; this strips path
/// separators from `name` only.
pub fn entity_rel_path(group_dir: &str, category_dir: &str, name: &str) -> String {
    let group = group_dir.trim_matches('/');
    let category = category_dir.trim_matches('/');
    let safe_name = sanitize_entity_name(name);
    if group.is_empty() {
        format!("{category}/{safe_name}.md")
    } else {
        format!("{group}/{category}/{safe_name}.md")
    }
}

fn sanitize_entity_name(name: &str) -> String {
    name.replace(['/', '\\'], "-")
}

#[cfg(test)]
mod tests {
    use super::entity_rel_path;

    #[test]
    fn empty_group_dir() {
        assert_eq!(entity_rel_path("", "schemas", "Pet"), "schemas/Pet.md");
    }

    #[test]
    fn nested_group_dir() {
        assert_eq!(
            entity_rel_path("v1/pets", "operations", "listPets"),
            "v1/pets/operations/listPets.md"
        );
    }

    #[test]
    fn hyphenated_category_dir() {
        assert_eq!(
            entity_rel_path("default", "request-bodies", "CreatePet"),
            "default/request-bodies/CreatePet.md"
        );
    }

    #[test]
    fn strips_path_separators_in_name() {
        assert_eq!(
            entity_rel_path("g", "schemas", "foo/bar"),
            "g/schemas/foo-bar.md"
        );
    }
}
