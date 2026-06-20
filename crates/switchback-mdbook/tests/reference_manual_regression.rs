//! Mixed-family reference manual render regression (Acme HTTP + gRPC, three versions).
//!
//! Refresh: `UPDATE=1 cargo test -p switchback-mdbook reference_manual_acme_v1 -- --nocapture`

mod common;

use common::{load_reference_manual_acme_v1, render_manual};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use switchback_traits::Layout;

const GOLDEN_ROOT: &str = "tests/fixtures/golden/reference_manual_acme_v1";

#[test]
fn reference_manual_acme_v1_matches_golden() {
    let update = std::env::var("UPDATE").is_ok();
    let manual = load_reference_manual_acme_v1();
    let opts = switchback_mdbook::parse_parameter(&Some("layout=package,init".into()))
        .expect("parse options");
    let rendered = render_manual(&manual, &opts);

    let paths = [
        "src/SUMMARY.md",
        "src/packages/openapi.acme.example.v1.md",
        "src/packages/openapi.acme.example.v2.md",
        "src/packages/openapi.acme.example.v3alpha1.md",
        "src/packages/protobuf.acme.example.v1.md",
        "src/packages/protobuf.acme.example.v2.md",
        "src/packages/protobuf.acme.example.v3alpha1.md",
    ];
    let subset: BTreeMap<String, String> = paths
        .iter()
        .map(|p| {
            (
                (*p).to_string(),
                rendered
                    .get(*p)
                    .unwrap_or_else(|| panic!("missing {p}"))
                    .clone(),
            )
        })
        .collect();

    let golden_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GOLDEN_ROOT);
    if update {
        fs::create_dir_all(&golden_dir).expect("mkdir golden");
        for (rel, content) in &subset {
            let path = golden_dir.join(rel);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("mkdir parent");
            }
            fs::write(path, content).expect("write golden");
        }
        return;
    }

    for (rel, expected) in &subset {
        let path = golden_dir.join(rel);
        let got = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read golden {rel}: {e}"));
        assert_eq!(
            expected,
            &got.replace("\r\n", "\n").replace('\r', "\n"),
            "golden mismatch at {rel}"
        );
    }

    let sum = subset.get("src/SUMMARY.md").expect("SUMMARY");
    assert!(
        sum.contains("HTTP (OpenAPI)"),
        "expected HTTP section in SUMMARY: {sum}"
    );
    assert!(
        sum.contains("gRPC (Protobuf)"),
        "expected gRPC section in SUMMARY: {sum}"
    );
    assert!(
        sum.contains("openapi.acme.example.v2"),
        "expected v2 OpenAPI group in SUMMARY: {sum}"
    );
    assert!(
        sum.contains("protobuf.acme.example.v3alpha1"),
        "expected v3alpha1 protobuf group in SUMMARY: {sum}"
    );
    let _ = Layout::Package;
}
