//! Buf-driven end-to-end example: load protobuf fixtures → render mdBook markdown.
//!
//! ```text
//! cargo run -p mdbook-protobuf-example -- -o /tmp/api-book
//! cargo run -p mdbook-protobuf-example -- --layout entity --summary -o /tmp/api-book
//! cargo run -p mdbook-protobuf-example -- --via-binpb /tmp/switchback.binpb -o /tmp/api-book
//! ```

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use switchback_codec_pb::ProtobufCodec;
use switchback_mdbook::{write_output_files, MdBookRenderer};
use switchback_protobuf::examples::{fixtures_proto_dir, EXAMPLE_PROTO_INPUTS};
use switchback_protobuf::load::{ensure_test_proto_deps, load, LoadArgs};
use switchback_protobuf::Compiler;
use switchback_traits::{Layout, Options, ReferenceManual, SyncRenderer, SyncSwitchbackCodec};

#[derive(Parser)]
#[command(name = "mdbook-protobuf")]
struct Cli {
    /// Output directory for the mdBook tree (markdown under `src/packages/`).
    #[arg(short, long, default_value = "api-book")]
    output: PathBuf,
    /// Page layout: package, entity, or split.
    #[arg(long, value_parser = parse_layout, default_value = "package")]
    layout: Layout,
    /// Regenerate `src/SUMMARY.md`.
    #[arg(long)]
    summary: bool,
    /// Scaffold a full mdBook project (`book.toml`, theme, README).
    #[arg(long)]
    init: bool,
    /// Link formatter name (default mdbook-relative).
    #[arg(long)]
    link_format: Option<String>,
    /// Skip copying companion proto markdown files.
    #[arg(long)]
    no_proto_markdown: bool,
    /// Render from a serialized switchback wire artifact instead of loading protos.
    #[arg(long)]
    via_binpb: Option<PathBuf>,
}

fn parse_layout(s: &str) -> Result<Layout> {
    match s {
        "package" => Ok(Layout::Package),
        "entity" => Ok(Layout::Entity),
        "split" => Ok(Layout::Split),
        other => anyhow::bail!("unknown layout {other:?}; use package, entity, or split"),
    }
}

fn load_manual(cli: &Cli) -> Result<ReferenceManual> {
    if let Some(path) = &cli.via_binpb {
        let bytes = fs::read(path).with_context(|| format!("read {}", path.display()))?;
        let codec = ProtobufCodec;
        return SyncSwitchbackCodec::deserialize(&codec, &bytes)
            .with_context(|| format!("deserialize switchback from {}", path.display()));
    }

    let module_root = fixtures_proto_dir();
    let export = ensure_test_proto_deps(&module_root, None).context("export proto deps")?;
    load(&LoadArgs {
        compiler: Compiler::Buf,
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
    })
    .context("load protobuf fixtures")
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let manual = load_manual(&cli)?;

    let opts = Options {
        layout: cli.layout,
        summary: cli.summary,
        init: cli.init,
        link_format: cli.link_format,
        no_proto_markdown: cli.no_proto_markdown,
        ..Default::default()
    };

    let files = MdBookRenderer
        .render(&manual, &opts)
        .context("render mdBook markdown")?;
    write_output_files(&cli.output, &files).context("write output files")?;

    eprintln!("wrote {} files under {}", files.len(), cli.output.display());
    Ok(())
}
