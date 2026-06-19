//! Resolve filesystem inputs into protobuf descriptors.

#[cfg(feature = "buf")]
mod buf;
mod fds;
mod merge;
#[cfg(feature = "protoc")]
mod protoc;

use crate::descriptor::FileDescriptorProto;
use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

#[cfg(feature = "buf")]
pub use buf::{compile_with_buf, resolve_buf_path};
pub use fds::load_descriptor_set;
pub use merge::{filter_file_to_generate, merge_proto_files};
#[cfg(feature = "protoc")]
pub use protoc::{compile_with_protoc, resolve_protoc_path};

/// Compiler for `.proto` inputs when not using a prebuilt descriptor set.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Compiler {
    #[default]
    Buf,
    Protoc,
}

/// Arguments for input resolution.
#[derive(Clone, Debug, Default)]
pub struct ResolveArgs {
    pub compiler: Compiler,
    pub descriptor_sets: Vec<PathBuf>,
    pub inputs: Vec<PathBuf>,
    pub proto_paths: Vec<PathBuf>,
    pub protoc_path: Option<PathBuf>,
    pub buf_path: Option<PathBuf>,
    pub proto_deps_export: Option<PathBuf>,
}

/// Resolved descriptor payload before entity population.
#[derive(Clone, Debug)]
pub struct ResolvedInput {
    pub proto_file: Vec<FileDescriptorProto>,
    pub file_to_generate: Vec<String>,
    pub proto_search_paths: Vec<PathBuf>,
    pub module_root: PathBuf,
}

/// Resolve inputs into descriptors and generation targets.
pub fn resolve_inputs(args: &ResolveArgs) -> Result<ResolvedInput> {
    let mut proto_file = Vec::new();
    let mut file_to_generate = Vec::new();
    let mut proto_search_paths = args.proto_paths.clone();
    let module_root = infer_module_root(&args.inputs)?;

    for path in &args.descriptor_sets {
        let (files, names) = load_descriptor_set(path)?;
        merge_proto_files(&mut proto_file, files);
        file_to_generate.extend(names);
    }

    if !args.descriptor_sets.is_empty() && args.inputs.is_empty() {
        // descriptor-set only
    } else if !args.inputs.is_empty() {
        let compiled = match args.compiler {
            #[cfg(feature = "buf")]
            Compiler::Buf => compile_with_buf(args, &module_root)?,
            #[cfg(not(feature = "buf"))]
            Compiler::Buf => bail!("buf compiler support disabled; enable the `buf` feature"),
            #[cfg(feature = "protoc")]
            Compiler::Protoc => compile_with_protoc(args, &module_root)?,
            #[cfg(not(feature = "protoc"))]
            Compiler::Protoc => {
                bail!("protoc compiler support disabled; enable the `protoc` feature")
            }
        };
        merge_proto_files(&mut proto_file, compiled.proto_file);
        if file_to_generate.is_empty() {
            file_to_generate = compiled.file_to_generate;
        } else {
            file_to_generate = filter_file_to_generate(&proto_file, &file_to_generate);
        }
        for p in compiled.proto_search_paths {
            if !proto_search_paths.iter().any(|x| x == &p) {
                proto_search_paths.push(p);
            }
        }
    } else {
        bail!("no inputs: pass proto paths or --descriptor-set");
    }

    if proto_file.is_empty() {
        bail!("no protobuf descriptors resolved from inputs");
    }

    file_to_generate.sort();
    file_to_generate.dedup();
    if file_to_generate.is_empty() {
        bail!("file_to_generate is empty after resolving inputs");
    }

    Ok(ResolvedInput {
        proto_file,
        file_to_generate,
        proto_search_paths,
        module_root,
    })
}

pub(crate) fn compile_to_fds(
    runner: impl FnOnce(&Path) -> Result<()>,
) -> Result<Vec<FileDescriptorProto>> {
    let fds_file = tempfile::Builder::new()
        .prefix("switchback-protobuf-")
        .suffix(".binpb")
        .tempfile()
        .context("create temp descriptor set")?;
    let fds_path = fds_file.path();
    runner(fds_path)?;
    let (proto_file, _) = load_descriptor_set(fds_path)?;
    Ok(proto_file)
}

fn infer_module_root(inputs: &[PathBuf]) -> Result<PathBuf> {
    for input in inputs {
        let path = if input.is_file() {
            input.parent().context("input file has no parent")?
        } else {
            input.as_path()
        };
        let mut dir = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        loop {
            if dir.join("buf.yaml").is_file() || dir.join("buf.yml").is_file() {
                return Ok(dir);
            }
            if !dir.pop() {
                break;
            }
        }
    }
    if let Some(first) = inputs.first() {
        if first.is_file() {
            return Ok(first
                .parent()
                .context("input file has no parent")?
                .to_path_buf());
        }
        return Ok(first.clone());
    }
    bail!("no inputs to infer module root");
}

#[cfg(feature = "buf")]
#[allow(dead_code)] // referenced when `buf` feature is disabled
pub(crate) const BUF_INSTALL_HINT: &str =
    "cargo install buf-toolchain --locked --version 1.70.0-hotfix.1";

pub(crate) fn tool_exists(name: &str) -> bool {
    use std::process::{Command, Stdio};
    Command::new(name)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok()
        .is_some_and(|s| s.success())
        || Command::new(name)
            .arg("version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .ok()
            .is_some_and(|s| s.success())
}
