//! Shared helpers for switchback-openapi integration tests.

#![allow(dead_code)]

use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

use switchback_codec_pb::ProtobufCodec;
use switchback_openapi::{load, LoadArgs};
use switchback_traits::{ReferenceManual, SyncSwitchbackCodec};

#[derive(Debug, serde::Deserialize)]
struct LockFile {
    asset: Vec<LockAsset>,
}

#[derive(Debug, serde::Deserialize)]
struct LockAsset {
    path: String,
    sha256: String,
}

pub fn assert_lock_integrity(crate_root: &Path, all_len: usize) {
    let lock_path = crate_root.join("meta-schemas.lock.toml");
    let text = fs::read_to_string(&lock_path).expect("read lock");
    let lock: LockFile = toml::from_str(&text).expect("parse lock");
    assert_eq!(lock.asset.len(), all_len, "lock entry count vs ALL");
    for asset in &lock.asset {
        let path = crate_root.join("meta-schemas").join(&asset.path);
        let bytes = fs::read(&path).expect("read asset");
        let actual = hex_sha256(&bytes);
        assert_eq!(actual, asset.sha256, "sha256 mismatch for {}", asset.path);
    }
}

fn hex_sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn load_fixture(relative: &str) -> ReferenceManual {
    let module_root = fixture_module_root(relative);
    let input = module_root.join(fixture_basename(relative));
    let args = LoadArgs {
        module_root: module_root.clone(),
        inputs: vec![input],
        search_roots: vec![module_root],
        title: None,
    };
    load(&args).unwrap_or_else(|e| panic!("load {relative}: {e}"))
}

fn fixture_module_root(relative: &str) -> PathBuf {
    fixtures_dir().join(Path::new(relative).parent().unwrap_or(Path::new("")))
}

fn fixture_basename(relative: &str) -> PathBuf {
    PathBuf::from(Path::new(relative).file_name().expect("fixture file name"))
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

pub fn codec_roundtrip(manual: &ReferenceManual) -> ReferenceManual {
    let codec = ProtobufCodec;
    let bytes = SyncSwitchbackCodec::serialize(&codec, manual).expect("serialize");
    SyncSwitchbackCodec::deserialize(&codec, &bytes).expect("deserialize")
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

pub fn assert_sources_match_inputs(manual: &ReferenceManual, module_root: &Path, inputs: &[&str]) {
    for input in inputs {
        let expected = std::fs::read(module_root.join(input)).expect("read input");
        let doc = manual
            .sources
            .iter()
            .find(|d| d.source_ref.uri == *input)
            .unwrap_or_else(|| panic!("missing source {input}"));
        assert_eq!(expected, doc.content, "source bytes for {input}");
    }
}

pub fn restore_sources_map(manual: &ReferenceManual, module_root: &Path) {
    switchback_openapi::restore_sources(manual, module_root).expect("restore sources");
}

pub fn fixtures_dir() -> PathBuf {
    switchback_openapi::fixtures_dir()
}
