//! OpenRPC fixture-driven end-to-end example: load test corpora → render mdBook projects.
//!
//! ```text
//! cargo run -p mdbook-openrpc-example -- -o /tmp/openrpc-books
//! cargo run -p mdbook-openrpc-example -- --fixture metrics-1.3 -o /tmp/metrics-book
//! cargo run -p mdbook-openrpc-example -- --tier micro -o /tmp/openrpc-micro
//! ```

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::{Parser, ValueEnum};
use switchback_codec_pb::ProtobufCodec;
use switchback_mdbook::{MdBookRenderer, write_output_files};
use switchback_openrpc::{
    EXAMPLE_FIXTURES, ExampleFixture, ExampleTier, default_example_fixtures, example_fixture,
    fixtures_for_tier, load_example,
};
use switchback_traits::{Layout, Options, ReferenceManual, SyncRenderer, SyncSwitchbackCodec};

#[derive(Parser)]
#[command(name = "mdbook-openrpc")]
struct Cli {
    /// Output directory. Each fixture renders to `{output}/{fixture-id}/`.
    #[arg(short, long, default_value = "openrpc-books")]
    output: PathBuf,
    /// Render only these fixture ids (repeatable). Default: Acme JSON-RPC corpus.
    #[arg(long = "fixture", value_name = "ID")]
    fixtures: Vec<String>,
    /// Which fixture tier to render when `--fixture` is not set.
    #[arg(long, value_enum, default_value_t = TierArg::Example)]
    tier: TierArg,
    /// Render every catalogued fixture (upstream + micro).
    #[arg(long)]
    all_fixtures: bool,
    /// List fixture ids and exit.
    #[arg(long)]
    list_fixtures: bool,
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
    /// Skip protobuf highlighting in init `book.toml`.
    #[arg(long)]
    no_proto_highlight: bool,
    /// Skip CEL highlighting in init `book.toml`.
    #[arg(long, visible_alias = "no-cel-highlight")]
    no_cel_highlight: bool,
    /// Link formatter name (default mdbook-relative).
    #[arg(long)]
    link_format: Option<String>,
    /// Render from a serialized switchback wire artifact instead of loading a fixture.
    #[arg(long, conflicts_with_all = ["fixtures", "tier", "all_fixtures"])]
    via_binpb: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, ValueEnum, Default)]
enum TierArg {
    #[default]
    Example,
    Upstream,
    Micro,
    All,
}

impl TierArg {
    fn fixtures(self) -> Vec<&'static ExampleFixture> {
        match self {
            Self::Example => default_example_fixtures().collect(),
            Self::Upstream => fixtures_for_tier(ExampleTier::Upstream).collect(),
            Self::Micro => fixtures_for_tier(ExampleTier::Micro).collect(),
            Self::All => EXAMPLE_FIXTURES.iter().collect(),
        }
    }
}

fn parse_layout(s: &str) -> Result<Layout> {
    match s {
        "package" => Ok(Layout::Package),
        "entity" => Ok(Layout::Entity),
        "split" => Ok(Layout::Split),
        other => bail!("unknown layout {other:?}; use package, entity, or split"),
    }
}

fn selected_fixtures(cli: &Cli) -> Result<Vec<&'static ExampleFixture>> {
    if cli.all_fixtures {
        return Ok(EXAMPLE_FIXTURES.iter().collect());
    }
    if !cli.fixtures.is_empty() {
        let mut out = Vec::with_capacity(cli.fixtures.len());
        for id in &cli.fixtures {
            let Some(fixture) = example_fixture(id) else {
                bail!(
                    "unknown fixture {id:?}; run with --list-fixtures (known: {})",
                    EXAMPLE_FIXTURES
                        .iter()
                        .map(|f| f.id)
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            };
            out.push(fixture);
        }
        return Ok(out);
    }
    Ok(cli.tier.fixtures())
}

fn load_manual(cli: &Cli, fixture: &ExampleFixture) -> Result<ReferenceManual> {
    if let Some(path) = &cli.via_binpb {
        let bytes = fs::read(path).with_context(|| format!("read {}", path.display()))?;
        let codec = ProtobufCodec;
        return SyncSwitchbackCodec::deserialize(&codec, &bytes)
            .with_context(|| format!("deserialize switchback from {}", path.display()));
    }
    load_example(fixture).with_context(|| format!("load fixture {}", fixture.id))
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
        no_proto_highlight: cli.no_proto_highlight,
        no_cel_highlight: cli.no_cel_highlight,
        link_format: cli.link_format.clone(),
        ignore_git: true,
        ..Default::default()
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.list_fixtures {
        for fixture in EXAMPLE_FIXTURES {
            eprintln!(
                "{}\t{}\t{}",
                fixture.id,
                fixture.tier.as_str(),
                fixture.relative
            );
        }
        return Ok(());
    }

    if cli.via_binpb.is_some() {
        let fixture = EXAMPLE_FIXTURES.first().expect("catalogue non-empty");
        let manual = load_manual(&cli, fixture)?;
        let opts = render_opts(&cli, &manual);
        let files = MdBookRenderer
            .render(&manual, &opts)
            .context("render mdBook markdown")?;
        write_output_files(&cli.output, &files).context("write output files")?;
        eprintln!("wrote {} files under {}", files.len(), cli.output.display());
        return Ok(());
    }

    let fixtures = selected_fixtures(&cli)?;
    if fixtures.is_empty() {
        bail!("no fixtures selected");
    }

    fs::create_dir_all(&cli.output)
        .with_context(|| format!("create output directory {}", cli.output.display()))?;

    let mut total_files = 0usize;
    let fixture_count = fixtures.len();
    for fixture in fixtures {
        let manual = load_manual(&cli, fixture)?;
        let opts = render_opts(&cli, &manual);
        let files = MdBookRenderer
            .render(&manual, &opts)
            .with_context(|| format!("render {}", fixture.id))?;
        let book_dir = cli.output.join(fixture.id);
        write_output_files(&book_dir, &files)
            .with_context(|| format!("write {}", book_dir.display()))?;
        eprintln!(
            "wrote {} files under {} ({})",
            files.len(),
            book_dir.display(),
            fixture.label
        );
        total_files += files.len();
    }

    eprintln!(
        "done: {} fixture(s), {} file(s) under {}",
        fixture_count,
        total_files,
        cli.output.display()
    );
    Ok(())
}
