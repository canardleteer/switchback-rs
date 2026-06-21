//! Load `.proto` inputs into a switchback [`ReferenceManual`].

use std::path::{Path, PathBuf};

use switchback_traits::ReferenceManual;

use crate::input::{Compiler, ResolveArgs, resolve_inputs};
use crate::manual::build_reference_manual;
use crate::populate::populate;

/// Arguments for [`load`].
#[derive(Clone, Debug)]
pub struct LoadArgs {
    pub compiler: Compiler,
    pub module_root: PathBuf,
    pub inputs: Vec<PathBuf>,
    pub proto_paths: Vec<PathBuf>,
    pub protoc_path: Option<PathBuf>,
    pub buf_path: Option<PathBuf>,
    pub proto_deps_export: Option<PathBuf>,
    pub title: Option<String>,
}

impl LoadArgs {
    pub fn examples(module_root: impl Into<PathBuf>, inputs: &[&str], compiler: Compiler) -> Self {
        Self {
            compiler,
            module_root: module_root.into(),
            inputs: inputs.iter().map(|p| PathBuf::from(*p)).collect(),
            proto_paths: Vec::new(),
            protoc_path: None,
            buf_path: None,
            proto_deps_export: None,
            title: None,
        }
    }
}

/// Parse protobuf inputs and return a populated reference manual.
pub fn load(args: &LoadArgs) -> switchback_traits::Result<ReferenceManual> {
    let resolve = ResolveArgs {
        compiler: args.compiler,
        inputs: args
            .inputs
            .iter()
            .map(|p| {
                if p.is_relative() {
                    args.module_root.join(p)
                } else {
                    p.clone()
                }
            })
            .collect(),
        proto_paths: if args.proto_paths.is_empty() {
            vec![args.module_root.clone()]
        } else {
            args.proto_paths.clone()
        },
        protoc_path: args.protoc_path.clone(),
        buf_path: args.buf_path.clone(),
        proto_deps_export: args.proto_deps_export.clone(),
        descriptor_sets: Vec::new(),
    };

    let resolved = resolve_inputs(&resolve)
        .map_err(|e| switchback_traits::SwitchbackError::load(e.to_string()))?;
    let populated = populate(&resolved)?;
    build_reference_manual(populated, &resolved, args.title.clone())
}

/// Default proto deps export directory under the crate target dir (for tests).
pub fn default_proto_deps_export() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/proto-deps")
}

/// Ensure BSR deps are exported for protoc-based compiles on the examples module.
pub fn ensure_test_proto_deps(
    module_root: &Path,
    buf_path: Option<&Path>,
) -> switchback_traits::Result<PathBuf> {
    let export = default_proto_deps_export();
    crate::proto_deps::ensure_proto_deps_export(module_root, &export, false, buf_path)
        .map_err(|e| switchback_traits::SwitchbackError::load(e.to_string()))
}
