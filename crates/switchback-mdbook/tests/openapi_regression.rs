//! OpenAPI render regression against golden fixtures.
//!
//! Refresh baselines: `UPDATE=1 cargo test -p switchback-mdbook openapi_regression -- --nocapture`

mod common;

use common::{collect_tree, load_openapi_tictactoe, render_manual};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use switchback_traits::Layout;

const GOLDEN_ROOT: &str = "tests/fixtures/golden/openapi_tictactoe";

struct Scenario {
    name: &'static str,
    layout: Layout,
}

const SCENARIOS: &[Scenario] = &[
    Scenario {
        name: "package",
        layout: Layout::Package,
    },
    Scenario {
        name: "entity",
        layout: Layout::Entity,
    },
    Scenario {
        name: "split",
        layout: Layout::Split,
    },
];

#[test]
fn openapi_render_matches_golden() {
    let update = std::env::var("UPDATE").is_ok();
    for scenario in SCENARIOS {
        let manual = load_openapi_tictactoe();
        let opts = switchback_mdbook::parse_parameter(&Some(format!(
            "layout={}",
            match scenario.layout {
                Layout::Package => "package",
                Layout::Entity => "entity",
                Layout::Split => "split",
            }
        )))
        .expect("parse");
        let rendered = render_manual(&manual, &opts);
        let golden_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(GOLDEN_ROOT)
            .join(scenario.name);

        if update {
            fs::create_dir_all(&golden_dir).expect("mkdir golden");
            write_tree(&golden_dir, &rendered);
            continue;
        }

        let golden = read_tree(&golden_dir);
        assert_trees_eq(scenario.name, &golden, &rendered);
    }
}

fn read_tree(root: &Path) -> BTreeMap<String, String> {
    collect_tree(root)
}

fn write_tree(root: &Path, tree: &BTreeMap<String, String>) {
    for (rel, content) in tree {
        let path = root.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("mkdir");
        }
        fs::write(path, content).expect("write golden");
    }
}

fn assert_trees_eq(
    name: &str,
    golden: &BTreeMap<String, String>,
    rendered: &BTreeMap<String, String>,
) {
    for (path, expected) in golden {
        let actual = rendered
            .get(path)
            .unwrap_or_else(|| panic!("{name}: missing rendered file {path}"));
        assert_eq!(expected, actual, "{name}: content mismatch at {path}");
    }
    for path in rendered.keys() {
        assert!(
            golden.contains_key(path),
            "{name}: unexpected rendered file {path}"
        );
    }
}

#[test]
fn openapi_put_operation_decomposition() {
    let manual = load_openapi_tictactoe();
    let opts = switchback_mdbook::parse_parameter(&Some("layout=entity".into())).expect("parse");
    let rendered = render_manual(&manual, &opts);
    let put_page = rendered
        .get("src/packages/gameplay/operations/PUT -board-{row}-{column}.md")
        .expect("PUT operation page");
    assert!(
        put_page.contains("`progressUrl` | header | `string` | optional |"),
        "expected progressUrl parameter type in table"
    );
    assert!(
        put_page.contains("Places a mark on the board"),
        "expected operation description in prose"
    );
    let description_pos = put_page
        .find("Places a mark on the board")
        .expect("description");
    let details_pos = put_page
        .find("<details>")
        .expect("collapsed operation source");
    assert!(
        description_pos < details_pos,
        "description should appear before collapsed YAML source"
    );
}

#[test]
fn openapi_split_index_category_sections() {
    let manual = load_openapi_tictactoe();
    let opts = switchback_mdbook::parse_parameter(&Some("layout=split".into())).expect("parse");
    let rendered = render_manual(&manual, &opts);
    let gameplay_index = rendered
        .get("src/packages/gameplay/index.md")
        .expect("gameplay index");
    assert!(
        gameplay_index.contains("## Operations"),
        "gameplay index should have Operations section"
    );
    let components_index = rendered
        .get("src/packages/components/index.md")
        .expect("components index");
    assert!(
        components_index.contains("## Schemas"),
        "components index should have Schemas section"
    );
    assert!(
        components_index.contains("## Parameters"),
        "components index should have Parameters section"
    );
    assert!(
        components_index.contains("## Security schemes"),
        "components index should have Security schemes section"
    );
    assert!(
        !components_index.contains("## Operations"),
        "components index should not list operations"
    );
}
