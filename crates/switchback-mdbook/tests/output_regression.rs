//! End-to-end render regression against golden fixtures.
//!
//! Refresh baselines: `cargo xtask update-golden`

mod common;

use common::{collect_tree, load_examples, render_manual};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use switchback_traits::Layout;

struct Scenario {
    name: &'static str,
    layout: Layout,
    summary: bool,
    init: bool,
    no_proto_markdown: bool,
    escape_tags: Option<switchback_traits::EscapeTags>,
    fixture: Option<&'static str>,
    paths_only: Option<&'static [&'static str]>,
    tier: &'static str,
}

const SCENARIOS: &[Scenario] = &[
    Scenario {
        name: "package_default",
        layout: Layout::Package,
        summary: false,
        init: false,
        no_proto_markdown: false,
        escape_tags: None,
        fixture: None,
        paths_only: None,
        tier: "port",
    },
    Scenario {
        name: "entity",
        layout: Layout::Entity,
        summary: false,
        init: false,
        no_proto_markdown: false,
        escape_tags: None,
        fixture: None,
        paths_only: None,
        tier: "port",
    },
    Scenario {
        name: "split",
        layout: Layout::Split,
        summary: false,
        init: false,
        no_proto_markdown: false,
        escape_tags: None,
        fixture: None,
        paths_only: None,
        tier: "port",
    },
    Scenario {
        name: "summary_companion",
        layout: Layout::Package,
        summary: true,
        init: false,
        no_proto_markdown: false,
        escape_tags: None,
        fixture: None,
        paths_only: Some(&["src/SUMMARY.md", "src/packages/acme.example.v1.md"]),
        tier: "port",
    },
    Scenario {
        name: "summary_companion_init",
        layout: Layout::Package,
        summary: false,
        init: true,
        no_proto_markdown: false,
        escape_tags: None,
        fixture: None,
        paths_only: Some(&["src/SUMMARY.md"]),
        tier: "new",
    },
    Scenario {
        name: "summary_entity_flat",
        layout: Layout::Entity,
        summary: true,
        init: false,
        no_proto_markdown: true,
        escape_tags: None,
        fixture: None,
        paths_only: Some(&["src/SUMMARY.md"]),
        tier: "port",
    },
    Scenario {
        name: "summary_split_flat",
        layout: Layout::Split,
        summary: true,
        init: false,
        no_proto_markdown: true,
        escape_tags: None,
        fixture: None,
        paths_only: Some(&["src/SUMMARY.md"]),
        tier: "port",
    },
    Scenario {
        name: "escape_tags_backticks",
        layout: Layout::Package,
        summary: false,
        init: false,
        no_proto_markdown: false,
        escape_tags: Some(switchback_traits::EscapeTags::Backticks),
        fixture: Some("escape_tags_comments.proto"),
        paths_only: Some(&["src/packages/acme.example.tagdoc.md"]),
        tier: "port",
    },
    Scenario {
        name: "escape_tags_entities",
        layout: Layout::Package,
        summary: false,
        init: false,
        no_proto_markdown: false,
        escape_tags: Some(switchback_traits::EscapeTags::Entities),
        fixture: Some("escape_tags_comments.proto"),
        paths_only: Some(&["src/packages/acme.example.tagdoc.md"]),
        tier: "port",
    },
    Scenario {
        // new: parse-time FQN intra-links in doc prose
        name: "intra_links_doc_fqn",
        layout: Layout::Package,
        summary: false,
        init: false,
        no_proto_markdown: false,
        escape_tags: None,
        fixture: None,
        paths_only: Some(&["src/packages/acme.example.v1.md"]),
        tier: "new",
    },
];

fn golden_dir() -> PathBuf {
    common::manifest_dir().join("tests/fixtures/golden")
}

fn generate_output(scenario: &Scenario) -> BTreeMap<String, String> {
    let manual = if let Some(fixture) = scenario.fixture {
        common::load_fixture(fixture)
    } else {
        load_examples()
    };
    let opts = switchback_traits::Options {
        layout: scenario.layout,
        summary: scenario.summary,
        init: scenario.init,
        ignore_git: scenario.init,
        no_proto_markdown: scenario.no_proto_markdown,
        escape_tags: scenario.escape_tags.unwrap_or_default(),
        ..Default::default()
    };
    let tree = render_manual(&manual, &opts);
    match scenario.paths_only {
        None => tree,
        Some(paths) => paths
            .iter()
            .map(|p| {
                let content = tree
                    .get(*p)
                    .unwrap_or_else(|| panic!("missing output path {p} in {}", scenario.name))
                    .clone();
                ((*p).to_string(), content)
            })
            .collect(),
    }
}

#[test]
fn output_regression_matches_golden() {
    let update = std::env::var("UPDATE_GOLDEN").ok().as_deref() == Some("1");
    for scenario in SCENARIOS {
        let actual = generate_output(scenario);
        if update {
            write_golden(scenario, &actual);
            eprintln!("updated golden [{}]: {}", scenario.tier, scenario.name);
            continue;
        }
        let expected = read_golden(scenario);
        assert_eq!(
            expected, actual,
            "golden mismatch [{}]: {}",
            scenario.tier, scenario.name
        );
    }
}

fn write_golden(scenario: &Scenario, tree: &BTreeMap<String, String>) {
    let dir = golden_dir().join(scenario.name);
    let _ = fs::remove_dir_all(&dir);
    for (rel, content) in tree {
        let path = dir.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create parent");
        }
        fs::write(path, content).expect("write golden");
    }
}

fn read_golden(scenario: &Scenario) -> BTreeMap<String, String> {
    let dir = golden_dir().join(scenario.name);
    assert!(dir.is_dir(), "missing golden dir: {}", dir.display());
    collect_tree(&dir)
}

#[test]
fn wire_codec_roundtrip_render_matches_direct() {
    use switchback_codec_pb::ProtobufCodec;
    use switchback_traits::SyncSwitchbackCodec;

    let manual = load_examples();
    let opts = switchback_traits::Options::default();
    let direct = render_manual(&manual, &opts);

    let codec = ProtobufCodec;
    let bytes = SyncSwitchbackCodec::serialize(&codec, &manual).expect("serialize");
    let roundtrip = SyncSwitchbackCodec::deserialize(&codec, &bytes).expect("deserialize");
    let via_wire = render_manual(&roundtrip, &opts);
    assert_eq!(
        direct, via_wire,
        "wire round-trip render must match direct render"
    );
}
