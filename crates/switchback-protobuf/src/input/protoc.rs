//! protoc compilation and include-path resolution.

use crate::input::{compile_to_fds, tool_exists, ResolveArgs, ResolvedInput};
use crate::paths::collect_proto_inputs;
use crate::proto_deps;
use anyhow::{bail, Context, Result};
use std::path::{Component, Path, PathBuf};
use std::process::Command;

pub fn compile_with_protoc(args: &ResolveArgs, module_root: &Path) -> Result<ResolvedInput> {
    if args.inputs.is_empty() {
        bail!("no inputs provided for protoc compilation");
    }

    let mut include_paths = args.proto_paths.clone();
    if include_paths.is_empty() {
        include_paths.push(module_root.to_path_buf());
    }

    if let Some(export_dir) = &args.proto_deps_export {
        proto_deps::ensure_proto_deps_export(
            module_root,
            export_dir,
            false,
            args.buf_path.as_deref(),
        )?;
        if !include_paths.iter().any(|p| p == export_dir) {
            include_paths.push(export_dir.clone());
        }
    }

    let protoc_inputs = resolve_protoc_file_args(&args.inputs, &include_paths)?;
    if protoc_inputs.is_empty() {
        bail!("no .proto inputs found");
    }

    let protoc = resolve_protoc_path(args.protoc_path.as_deref())?;
    let proto_file = compile_to_fds(|fds_path| {
        let mut cmd = Command::new(&protoc);
        cmd.arg("--descriptor_set_out").arg(fds_path);
        cmd.arg("--include_imports");
        cmd.arg("--include_source_info");
        for inc in &include_paths {
            cmd.arg("-I").arg(inc);
        }
        for (protoc_arg, _) in &protoc_inputs {
            cmd.arg(protoc_arg);
        }

        let status = cmd
            .status()
            .with_context(|| format!("spawn {}", protoc.display()))?;
        if !status.success() {
            bail!("protoc failed");
        }
        Ok(())
    })?;
    let file_to_generate: Vec<String> = protoc_inputs.into_iter().map(|(_, name)| name).collect();

    Ok(ResolvedInput {
        proto_file,
        file_to_generate,
        proto_search_paths: include_paths,
        module_root: module_root.to_path_buf(),
    })
}

pub fn resolve_protoc_path(explicit: Option<&Path>) -> Result<PathBuf> {
    if let Some(path) = explicit {
        return Ok(path.to_path_buf());
    }
    if tool_exists("protoc") {
        return Ok(PathBuf::from("protoc"));
    }
    #[cfg(feature = "protoc")]
    {
        protoc_bin_vendored::protoc_bin_path().map_err(|e| {
            anyhow::anyhow!(
                "protoc not found on PATH and vendored protoc unavailable ({e}); \
                 install protoc or enable the `protoc` feature"
            )
        })
    }
    #[cfg(not(feature = "protoc"))]
    {
        bail!("protoc not found on PATH; enable the `protoc` feature")
    }
}

fn resolve_protoc_file_args(
    inputs: &[PathBuf],
    include_paths: &[PathBuf],
) -> Result<Vec<(PathBuf, String)>> {
    let mut canon_includes = Vec::new();
    for inc in include_paths {
        let c = inc.canonicalize().unwrap_or_else(|_| inc.clone());
        canon_includes.push(c);
    }
    canon_includes.sort_by_key(|b| std::cmp::Reverse(b.components().count()));

    let (abs_files, _) = collect_proto_inputs(inputs)?;
    let mut out = Vec::new();
    for abs in abs_files {
        let canonical = abs.canonicalize().unwrap_or(abs);
        let mut matched = false;
        for inc in &canon_includes {
            if let Ok(rel) = canonical.strip_prefix(inc) {
                let name = rel
                    .components()
                    .filter_map(|c| match c {
                        Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
                        Component::CurDir => None,
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("/");
                if name.is_empty() {
                    continue;
                }
                out.push((PathBuf::from(&name), name));
                matched = true;
                break;
            }
        }
        if !matched {
            bail!(
                "{} is not under any --proto-path (-I) directory; add an include path",
                canonical.display()
            );
        }
    }
    out.sort_by(|a, b| a.1.cmp(&b.1));
    out.dedup_by(|a, b| a.1 == b.1);
    Ok(out)
}
