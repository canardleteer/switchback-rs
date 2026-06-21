//! Shared helpers for switchback-jsonschema integration tests.

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use switchback_codec_pb::{DEFAULT_SWITCHBACK_FILENAME, ProtobufCodec};
use switchback_jsonschema::examples::EXAMPLE_CATALOG_INPUTS;
use switchback_jsonschema::{LoadArgs, load, restore_sources};
use switchback_traits::{ReferenceManual, SyncSwitchbackCodec};

pub fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub use switchback_jsonschema::examples::fixtures_catalog_dir;

pub fn load_catalog() -> ReferenceManual {
    let module_root = fixtures_catalog_dir();
    let args = LoadArgs {
        module_root: module_root.clone(),
        inputs: EXAMPLE_CATALOG_INPUTS
            .iter()
            .map(|p| module_root.join(p))
            .collect(),
        search_roots: vec![module_root.clone()],
        title: None,
    };
    load(&args).unwrap_or_else(|e| panic!("load examples catalog: {e}"))
}

pub fn normalize(mut manual: ReferenceManual) -> ReferenceManual {
    for module in &mut manual.modules {
        for contract in &mut module.contracts {
            contract.groups.sort_by(|a, b| a.id.cmp(&b.id));
            for group in &mut contract.groups {
                group.source_path = PathBuf::new();
                group.entities.sort_by(|a, b| a.name.cmp(&b.name));
            }
        }
    }
    manual
        .sources
        .sort_by(|a, b| a.source_ref.uri.cmp(&b.source_ref.uri));
    manual
}

pub fn restore_sources_map(manual: &ReferenceManual, module_root: &Path) {
    restore_sources(manual, module_root).expect("restore sources");
}

pub fn codec_roundtrip(manual: &ReferenceManual) -> ReferenceManual {
    let codec = ProtobufCodec;
    let bytes = SyncSwitchbackCodec::serialize(&codec, manual).expect("serialize");
    assert!(!bytes.is_empty());
    assert_eq!(DEFAULT_SWITCHBACK_FILENAME, "switchback.binpb");
    SyncSwitchbackCodec::deserialize(&codec, &bytes).expect("deserialize")
}

pub fn assert_sources_match_inputs(manual: &ReferenceManual, module_root: &Path, inputs: &[&str]) {
    let by_uri: BTreeMap<_, _> = manual
        .sources
        .iter()
        .map(|d| (d.source_ref.uri.as_str(), d))
        .collect();
    for input in inputs {
        let expected = fs::read(module_root.join(input))
            .unwrap_or_else(|e| panic!("read fixture {input}: {e}"));
        let doc = by_uri
            .get(input)
            .unwrap_or_else(|| panic!("missing source for {input}"));
        assert_eq!(expected, doc.content, "source bytes for {input}");
    }
}

pub fn count_entities(manual: &ReferenceManual) -> usize {
    manual
        .modules
        .iter()
        .flat_map(|m| &m.contracts)
        .flat_map(|c| &c.groups)
        .map(|g| g.entities.len())
        .sum()
}

pub fn count_refs(manual: &ReferenceManual) -> usize {
    manual
        .modules
        .iter()
        .flat_map(|m| &m.contracts)
        .flat_map(|c| &c.groups)
        .flat_map(|g| &g.entities)
        .map(|e| e.refs.len())
        .sum()
}
