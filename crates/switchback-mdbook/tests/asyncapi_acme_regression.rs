//! Acme AsyncAPI render regression against golden fixtures.
//!
//! Refresh baselines: `UPDATE=1 cargo test -p switchback-mdbook asyncapi_acme_regression -- --nocapture`

mod common;

use common::{collect_tree, load_asyncapi_acme, render_manual};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use switchback_traits::Layout;

const GOLDEN_ROOT: &str = "tests/fixtures/golden/asyncapi_acme";

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
fn asyncapi_acme_render_matches_golden() {
    let update = std::env::var("UPDATE").is_ok();
    for scenario in SCENARIOS {
        let manual = load_asyncapi_acme();
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
    let golden_paths: Vec<_> = golden.keys().collect();
    let rendered_paths: Vec<_> = rendered.keys().collect();
    assert_eq!(
        golden_paths, rendered_paths,
        "{name}: path list mismatch\n  golden: {golden_paths:?}\n  rendered: {rendered_paths:?}"
    );
    for key in golden.keys() {
        assert_eq!(
            golden.get(key).unwrap(),
            rendered.get(key).unwrap(),
            "{name}: content mismatch at {key}"
        );
    }
}
