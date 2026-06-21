//! Assemble Acme OpenAPI + protobuf (v1, v2, v3alpha1) into one mdBook reference manual.
//!
//! ```text
//! cargo run -p reference-manual-example -- -o /tmp/acme-ref
//! cargo run -p reference-manual-example -- --via-binpb /tmp/acme-ref.binpb -o /tmp/acme-ref
//! ```

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
use switchback_assemble::{AssembleArgs, GroupPrefixPolicy, assemble_module};
use switchback_asyncapi::load::LoadArgs as AsyncApiLoadArgs;
use switchback_codec_pb::ProtobufCodec;
use switchback_mdbook::{MdBookRenderer, write_output_files};
use switchback_openapi::load::LoadArgs as OpenApiLoadArgs;
use switchback_openrpc::load::LoadArgs as OpenRpcLoadArgs;
use switchback_protobuf::Compiler;
use switchback_protobuf::load::{LoadArgs as ProtobufLoadArgs, ensure_test_proto_deps};
use switchback_traits::{Layout, Options, ReferenceManual, SyncRenderer, SyncSwitchbackCodec};

#[derive(Parser)]
#[command(name = "reference-manual")]
struct Cli {
    /// Output directory for the mdBook project.
    #[arg(short, long, default_value = "acme-ref-book")]
    output: PathBuf,
    /// Path to `module.yaml` (default: beside this binary's manifest dir).
    #[arg(long, default_value = "module.yaml")]
    module: PathBuf,
    /// Page layout: package, entity, or split.
    #[arg(long, value_parser = parse_layout, default_value = "package")]
    layout: Layout,
    /// Emit API markdown only (no mdBook scaffold).
    #[arg(long, visible_alias = "no-init")]
    markdown_only: bool,
    /// Regenerate `src/SUMMARY.md` (only with `--markdown-only`).
    #[arg(long, requires = "markdown_only")]
    summary: bool,
    /// `book.toml` title override when init.
    #[arg(long)]
    title: Option<String>,
    /// Render from a serialized switchback wire artifact.
    #[arg(long, conflicts_with = "module")]
    via_binpb: Option<PathBuf>,
    /// Write a serialized `.binpb` artifact beside the book output.
    #[arg(long)]
    emit_binpb: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct ModuleManifest {
    id: String,
    title: String,
    overview: String,
    contracts: Vec<ContractManifest>,
}

#[derive(Debug, Deserialize)]
struct ContractManifest {
    family: String,
    module_root: String,
    inputs: Vec<String>,
}

fn parse_layout(s: &str) -> Result<Layout> {
    match s {
        "package" => Ok(Layout::Package),
        "entity" => Ok(Layout::Entity),
        "split" => Ok(Layout::Split),
        other => anyhow::bail!("unknown layout {other:?}; use package, entity, or split"),
    }
}

fn example_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn resolve_path(base: &Path, rel: &str) -> PathBuf {
    let path = PathBuf::from(rel);
    if path.is_absolute() {
        path
    } else {
        base.join(path)
    }
}

fn load_from_manifest(path: &Path) -> Result<ReferenceManual> {
    let yaml = fs::read_to_string(path)
        .with_context(|| format!("read module manifest {}", path.display()))?;
    let manifest: ModuleManifest = serde_yaml::from_str(&yaml).context("parse module.yaml")?;
    let manifest_dir = path.parent().unwrap_or_else(|| Path::new("."));

    let mut openapi: Option<OpenApiLoadArgs> = None;
    let mut protobuf: Option<ProtobufLoadArgs> = None;
    let mut asyncapi: Option<AsyncApiLoadArgs> = None;
    let mut openrpc: Option<OpenRpcLoadArgs> = None;

    for contract in &manifest.contracts {
        let module_root = resolve_path(manifest_dir, &contract.module_root);
        let inputs: Vec<PathBuf> = contract.inputs.iter().map(PathBuf::from).collect();
        match contract.family.as_str() {
            "openapi" => {
                openapi = Some(OpenApiLoadArgs {
                    module_root: module_root.clone(),
                    inputs,
                    search_roots: vec![module_root],
                    title: None,
                });
            }
            "protobuf" => {
                let export =
                    ensure_test_proto_deps(&module_root, None).context("export proto deps")?;
                protobuf = Some(ProtobufLoadArgs {
                    compiler: Compiler::Buf,
                    module_root: module_root.clone(),
                    inputs,
                    proto_paths: vec![module_root.clone(), export.clone()],
                    protoc_path: None,
                    buf_path: None,
                    proto_deps_export: Some(export),
                    title: None,
                });
            }
            "asyncapi" => {
                asyncapi = Some(AsyncApiLoadArgs {
                    module_root: module_root.clone(),
                    inputs,
                    search_roots: vec![module_root],
                    title: None,
                });
            }
            "openrpc" => {
                openrpc = Some(OpenRpcLoadArgs {
                    module_root: module_root.clone(),
                    inputs,
                    search_roots: vec![module_root],
                    title: None,
                });
            }
            other => anyhow::bail!("unsupported contract family {other} in module.yaml"),
        }
    }

    assemble_module(&AssembleArgs {
        module_id: manifest.id,
        title: manifest.title,
        overview: manifest.overview,
        group_prefix: GroupPrefixPolicy::ContractFamily,
        openapi,
        protobuf,
        asyncapi,
        openrpc,
    })
    .map_err(|e| anyhow::anyhow!("{e}"))
}

fn render_opts(cli: &Cli, manual: &ReferenceManual) -> Options {
    let init = !cli.markdown_only;
    Options {
        layout: cli.layout,
        summary: cli.summary,
        init,
        title: cli.title.clone().or_else(|| {
            if init {
                Some(manual.title.clone())
            } else {
                None
            }
        }),
        ignore_git: true,
        ..Default::default()
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let manual = if let Some(path) = &cli.via_binpb {
        let bytes = fs::read(path).with_context(|| format!("read {}", path.display()))?;
        let codec = ProtobufCodec;
        SyncSwitchbackCodec::deserialize(&codec, &bytes)
            .with_context(|| format!("deserialize switchback from {}", path.display()))?
    } else {
        let manifest_path = if cli.module.is_absolute() {
            cli.module.clone()
        } else {
            example_root().join(&cli.module)
        };
        load_from_manifest(&manifest_path)?
    };

    if let Some(path) = &cli.emit_binpb {
        let codec = ProtobufCodec;
        let bytes = SyncSwitchbackCodec::serialize(&codec, &manual).context("serialize manual")?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).context("create binpb parent")?;
        }
        fs::write(path, bytes).with_context(|| format!("write {}", path.display()))?;
        eprintln!("wrote {}", path.display());
    }

    let opts = render_opts(&cli, &manual);
    let files = MdBookRenderer
        .render(&manual, &opts)
        .context("render mdBook markdown")?;
    fs::create_dir_all(&cli.output).with_context(|| format!("create {}", cli.output.display()))?;
    write_output_files(&cli.output, &files).context("write output files")?;
    eprintln!(
        "wrote {} files under {} (module {})",
        files.len(),
        cli.output.display(),
        manual.modules.first().map(|m| m.id.as_str()).unwrap_or("")
    );
    Ok(())
}
