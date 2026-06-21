//! Buf-driven end-to-end example: load protobuf fixtures → render an mdBook project.
//!
//! Default output matches `protobuf-mdbook --init` (scaffold + package SUMMARY +
//! API markdown). Use `--markdown-only` for refresh-style output without
//! `book.toml` / theme / README.
//!
//! ```text
//! cargo run -p mdbook-protobuf-example -- -o /tmp/api-book
//! cargo run -p mdbook-protobuf-example -- --layout entity -o /tmp/api-book
//! cargo run -p mdbook-protobuf-example -- --markdown-only -o /tmp/api-book-out
//! cargo run -p mdbook-protobuf-example -- --via-binpb /tmp/switchback.binpb -o /tmp/api-book
//! ```

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use switchback_codec_pb::ProtobufCodec;
use switchback_mdbook::{MdBookRenderer, write_output_files};
use switchback_protobuf::Compiler;
use switchback_protobuf::examples::{EXAMPLE_PROTO_INPUTS, fixtures_proto_dir};
use switchback_protobuf::load::{LoadArgs, ensure_test_proto_deps, load};
use switchback_traits::{Layout, Options, ReferenceManual, SyncRenderer, SyncSwitchbackCodec};

#[derive(Parser)]
#[command(name = "mdbook-protobuf")]
struct Cli {
    /// Output directory for the mdBook project (default `./api-book`).
    #[arg(short, long, default_value = "api-book")]
    output: PathBuf,
    /// Page layout: package, entity, or split.
    #[arg(long, value_parser = parse_layout, default_value = "package")]
    layout: Layout,
    /// Emit API markdown only (no mdBook scaffold; protobuf-mdbook without `--init`).
    #[arg(long, visible_alias = "no-init")]
    markdown_only: bool,
    /// Regenerate `src/SUMMARY.md` (only with `--markdown-only`; init includes SUMMARY).
    #[arg(long, requires = "markdown_only")]
    summary: bool,
    /// `book.toml` title when init (default **Protobuf documentation**).
    #[arg(long)]
    title: Option<String>,
    /// Skip protobuf highlighting in init `book.toml`.
    #[arg(long)]
    no_proto_highlight: bool,
    /// Skip CEL highlighting in init `book.toml`.
    #[arg(long)]
    no_cel_highlight: bool,
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

    let init = !cli.markdown_only;
    let opts = Options {
        layout: cli.layout,
        summary: cli.summary,
        init,
        title: cli.title,
        no_proto_highlight: cli.no_proto_highlight,
        no_cel_highlight: cli.no_cel_highlight,
        link_format: cli.link_format,
        no_proto_markdown: cli.no_proto_markdown,
        ignore_git: true,
        ..Default::default()
    };

    let files = MdBookRenderer
        .render(&manual, &opts)
        .context("render mdBook markdown")?;
    write_output_files(&cli.output, &files).context("write output files")?;

    eprintln!("wrote {} files under {}", files.len(), cli.output.display());
    Ok(())
}
