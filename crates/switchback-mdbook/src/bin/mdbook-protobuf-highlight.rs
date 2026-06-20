//! mdBook preprocessor: build-time protobuf / CEL syntax highlighting.

use anyhow::{Context, Result};
use clap::{Arg, ArgMatches, Command};
use mdbook_preprocessor::book::Book;
use mdbook_preprocessor::errors::Error;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use std::io;
use std::path::PathBuf;
use std::process;
use switchback_mdbook::highlight::{
    config_from_mdbook, install_book_toml, transform_chapter, HighlightConfig, PREPROCESSOR_COMMAND,
};

struct ProtobufHighlight;

impl Preprocessor for ProtobufHighlight {
    fn name(&self) -> &str {
        "protobuf-highlight"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let config = config_from_mdbook(ctx);
        book.for_each_chapter_mut(|chapter| {
            if let Ok(md) = transform_chapter(&chapter.content, config) {
                chapter.content = md;
            }
        });
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> Result<bool, Error> {
        Ok(renderer == "html")
    }
}

fn make_app() -> Command {
    Command::new(PREPROCESSOR_COMMAND)
        .about("mdBook preprocessor for build-time protobuf and CEL highlighting")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported"),
        )
        .subcommand(
            Command::new("install").arg(
                Arg::new("dir")
                    .default_value(".")
                    .help("Book root directory containing book.toml"),
            ),
        )
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!(
            "mdbook-protobuf-highlight {} (mdbook {})",
            env!("CARGO_PKG_VERSION"),
            switchback_mdbook::mdbook_version()
        );
        return;
    }

    let matches = make_app().get_matches();
    if let Some(sub) = matches.subcommand_matches("supports") {
        handle_supports(sub);
    } else if let Some(sub) = matches.subcommand_matches("install") {
        if let Err(e) = handle_install(sub) {
            eprintln!("{e:#}");
            process::exit(1);
        }
    } else if let Err(e) = handle_preprocessing() {
        eprintln!("{e:#}");
        process::exit(1);
    }
}

fn handle_supports(sub: &ArgMatches) -> ! {
    let renderer = sub
        .get_one::<String>("renderer")
        .expect("renderer required");
    if ProtobufHighlight
        .supports_renderer(renderer)
        .unwrap_or(false)
    {
        process::exit(0);
    }
    process::exit(1);
}

fn handle_install(sub: &ArgMatches) -> Result<()> {
    let dir = sub.get_one::<String>("dir").expect("dir required");
    install_book_toml(PathBuf::from(dir).as_path(), HighlightConfig::all())
        .context("install preprocessor in book.toml")
}

fn handle_preprocessing() -> Result<()> {
    let (ctx, book) = mdbook_preprocessor::parse_input(io::stdin())?;
    let processed = ProtobufHighlight.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed)?;
    Ok(())
}
