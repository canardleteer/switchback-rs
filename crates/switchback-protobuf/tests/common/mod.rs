//! Shared helpers for switchback-protobuf integration tests.

#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use switchback_codec_pb::{ProtobufCodec, DEFAULT_SWITCHBACK_FILENAME};
use switchback_protobuf::examples::{fixtures_proto_dir, EXAMPLE_PROTO_INPUTS};
use switchback_protobuf::input::{resolve_buf_path, Compiler};
use switchback_protobuf::load::{ensure_test_proto_deps, load, LoadArgs};
use switchback_protobuf::restore_sources;
use switchback_traits::{ReferenceManual, SyncSwitchbackCodec};

pub fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn fixtures_dir() -> PathBuf {
    manifest_dir().join("tests/fixtures")
}

/// Compilers enabled for the current build (default features: protoc + buf).
#[allow(clippy::vec_init_then_push)] // cfg-gated pushes; `vec![]` is awkward here
pub fn mirrored_compilers() -> Vec<Compiler> {
    let mut compilers = Vec::new();
    #[cfg(feature = "protoc")]
    compilers.push(Compiler::Protoc);
    #[cfg(feature = "buf")]
    compilers.push(Compiler::Buf);
    compilers
}

pub fn ensure_proto_deps_export() -> PathBuf {
    ensure_test_proto_deps(&fixtures_proto_dir(), None).expect("ensure proto deps export")
}

pub fn load_examples(compiler: Compiler) -> ReferenceManual {
    let module_root = fixtures_proto_dir();
    let export = ensure_proto_deps_export();
    let args = LoadArgs {
        compiler,
        module_root: module_root.clone(),
        inputs: EXAMPLE_PROTO_INPUTS
            .iter()
            .map(|p| PathBuf::from(*p))
            .collect(),
        proto_paths: vec![module_root.clone(), export.clone()],
        protoc_path: None,
        buf_path: None,
        proto_deps_export: Some(export),
        title: None,
    };
    load(&args).unwrap_or_else(|e| panic!("load examples ({compiler:?}): {e}"))
}

pub fn load_fixture(name: &str, compiler: Compiler) -> ReferenceManual {
    let fixture_dir = fixtures_dir();
    let export = ensure_proto_deps_export();
    let args = LoadArgs {
        compiler,
        module_root: fixture_dir.clone(),
        inputs: vec![PathBuf::from(name)],
        proto_paths: vec![fixture_dir.clone(), export.clone()],
        protoc_path: None,
        buf_path: None,
        proto_deps_export: Some(export),
        title: None,
    };
    load(&args).unwrap_or_else(|e| panic!("load fixture {name} ({compiler:?}): {e}"))
}

pub fn load_loose_fixture(name: &str, compiler: Compiler) -> ReferenceManual {
    let fixture_dir = fixtures_dir().join("loose");
    let args = LoadArgs {
        compiler,
        module_root: fixture_dir.clone(),
        inputs: vec![PathBuf::from(name)],
        proto_paths: vec![fixture_dir.clone()],
        protoc_path: None,
        buf_path: None,
        proto_deps_export: None,
        title: None,
    };
    load(&args).unwrap_or_else(|e| panic!("load loose fixture {name} ({compiler:?}): {e}"))
}

pub fn normalize(mut manual: ReferenceManual) -> ReferenceManual {
    manual
        .sources
        .sort_by(|a, b| a.source_ref.uri.cmp(&b.source_ref.uri));
    for module in &mut manual.modules {
        for contract in &mut module.contracts {
            contract
                .companions
                .sort_by(|a, b| a.output_name.cmp(&b.output_name));
            for group in &mut contract.groups {
                group.source_path = PathBuf::new();
                group.entities.sort_by(|a, b| {
                    (a.category.as_str(), a.name.as_str())
                        .cmp(&(b.category.as_str(), b.name.as_str()))
                });
            }
            contract.groups.sort_by(|a, b| a.id.cmp(&b.id));
        }
    }
    manual
}

pub fn restore_sources_map(
    manual: &ReferenceManual,
    module_root: &Path,
) -> HashMap<PathBuf, Vec<u8>> {
    restore_sources(manual, module_root).expect("restore sources");
    let mut out = HashMap::new();
    for doc in &manual.sources {
        let path = module_root.join(&doc.source_ref.uri);
        let bytes = fs::read(&path).expect("read restored source");
        out.insert(PathBuf::from(&doc.source_ref.uri), bytes);
    }
    out
}

pub fn rebuild_buf_module(
    manual: &ReferenceManual,
    fixture_proto_dir: &Path,
    out_dir: &Path,
) -> io::Result<()> {
    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    copy_dir_all(fixture_proto_dir, out_dir)?;
    restore_sources(manual, out_dir).map_err(|e| io::Error::other(e.to_string()))
}

pub fn run_buf_lint_format(module_root: &Path) {
    let buf = resolve_buf_path(None).expect("resolve buf path");
    let lint = Command::new(&buf)
        .current_dir(module_root)
        .arg("lint")
        .status()
        .expect("spawn buf lint");
    assert!(
        lint.success(),
        "buf lint failed in {}",
        module_root.display()
    );

    let format = Command::new(&buf)
        .current_dir(module_root)
        .args(["format", "--diff"])
        .status()
        .expect("spawn buf format --diff");
    assert!(
        format.success(),
        "buf format --diff failed in {}",
        module_root.display()
    );
}

pub fn assert_sources_match_inputs(manual: &ReferenceManual, module_root: &Path, inputs: &[&str]) {
    assert_eq!(manual.sources.len(), inputs.len(), "sources count");
    for input in inputs {
        let fixture_path = module_root.join(input);
        let fixture_bytes = fs::read(&fixture_path)
            .unwrap_or_else(|e| panic!("read fixture proto {}: {e}", fixture_path.display()));
        let doc = manual
            .sources
            .iter()
            .find(|d| d.source_ref.uri == *input)
            .unwrap_or_else(|| panic!("missing source for {input}"));
        assert_eq!(doc.content, fixture_bytes, "source bytes for {input}");
        assert!(
            input.contains('/'),
            "expected directory-preserving uri for {input}"
        );
        assert!(
            doc.source_ref.uri.contains('/'),
            "source uri must preserve directories: {}",
            doc.source_ref.uri
        );
    }
}

pub fn codec_roundtrip(manual: &ReferenceManual) -> ReferenceManual {
    let codec = ProtobufCodec;
    let bytes = SyncSwitchbackCodec::serialize(&codec, manual).expect("serialize");
    assert!(!bytes.is_empty());
    assert_eq!(DEFAULT_SWITCHBACK_FILENAME, "switchback.binpb");
    SyncSwitchbackCodec::deserialize(&codec, &bytes).expect("deserialize")
}

fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_all(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}
