//! mdBook plugin option parsing (comma-separated protoc tokens).

use anyhow::{bail, Result};
use std::path::PathBuf;
use switchback_traits::{EscapeTags, Layout, Options};

use crate::paths::normalize_rel_path;

fn default_markdown_root() -> &'static str {
    "src/packages"
}

fn default_summary_path() -> &'static str {
    "src/SUMMARY.md"
}

fn normalize_book_root(root: &str) -> String {
    normalize_rel_path(root, ".").expect("book_root validated")
}

enum ParsedToken {
    Init,
    Summary,
    NoProtoHighlight,
    NoCelHighlight,
    NoProtoMarkdown,
    MarkdownOnly,
    EscapeTags(EscapeTags),
    Layout(Layout),
    BookRoot(String),
    MarkdownRoot(String),
    SummaryPath(String),
    Book(String),
    MdbookOut(String),
    Title(String),
    IgnoreGit(bool),
    LinkFormat(String),
    SearchPath(Vec<PathBuf>),
    AlphabetizeServices,
    AlphabetizeMessages,
}

fn parse_token(token: &str) -> Result<ParsedToken> {
    if let Some(v) = token.strip_prefix("book_root=") {
        return Ok(ParsedToken::BookRoot(normalize_book_root(v)));
    }
    if let Some(v) = token.strip_prefix("markdown_root=") {
        return Ok(ParsedToken::MarkdownRoot(normalize_rel_path(
            v,
            default_markdown_root(),
        )?));
    }
    if let Some(v) = token.strip_prefix("summary_path=") {
        return Ok(ParsedToken::SummaryPath(normalize_rel_path(
            v,
            default_summary_path(),
        )?));
    }
    if let Some(v) = token.strip_prefix("book=") {
        return Ok(ParsedToken::Book(v.to_string()));
    }
    if let Some(v) = token.strip_prefix("mdbook_out=") {
        return Ok(ParsedToken::MdbookOut(v.to_string()));
    }
    if let Some(v) = token.strip_prefix("title=") {
        return Ok(ParsedToken::Title(v.to_string()));
    }
    if let Some(v) = token.strip_prefix("link_format=") {
        return Ok(ParsedToken::LinkFormat(v.to_string()));
    }
    if let Some(v) = token.strip_prefix("ignore=") {
        return Ok(ParsedToken::IgnoreGit(match v {
            "git" => true,
            "none" => false,
            other => bail!("unknown ignore value {other:?}; use git or none"),
        }));
    }
    if let Some(v) = token.strip_prefix("proto_path=") {
        return Ok(ParsedToken::SearchPath(
            v.split(':')
                .filter(|s| !s.is_empty())
                .map(PathBuf::from)
                .collect(),
        ));
    }
    if let Some(v) = token.strip_prefix("layout=") {
        return Ok(ParsedToken::Layout(match v {
            "package" => Layout::Package,
            "entity" => Layout::Entity,
            "split" => Layout::Split,
            other => bail!("unknown layout {other:?}; use package, entity, or split"),
        }));
    }
    if token == "escape_tags" {
        return Ok(ParsedToken::EscapeTags(EscapeTags::Backticks));
    }
    if let Some(v) = token.strip_prefix("escape_tags=") {
        return Ok(ParsedToken::EscapeTags(match v {
            "backticks" => EscapeTags::Backticks,
            "entities" => EscapeTags::Entities,
            other => bail!("unknown escape_tags value {other:?}; use backticks or entities"),
        }));
    }

    match token {
        "init" => Ok(ParsedToken::Init),
        "summary" => Ok(ParsedToken::Summary),
        "no_proto_highlight" => Ok(ParsedToken::NoProtoHighlight),
        "no_cel_highlight" => Ok(ParsedToken::NoCelHighlight),
        "no_proto_markdown" => Ok(ParsedToken::NoProtoMarkdown),
        "markdown_only" => Ok(ParsedToken::MarkdownOnly),
        "alphabetize_services" => Ok(ParsedToken::AlphabetizeServices),
        "alphabetize_messages" => Ok(ParsedToken::AlphabetizeMessages),
        other => bail!("unknown plugin option: {other:?}"),
    }
}

fn apply_parsed(opts: &mut Options, token: ParsedToken) {
    match token {
        ParsedToken::Init => opts.init = true,
        ParsedToken::Summary => opts.summary = true,
        ParsedToken::NoProtoHighlight => opts.no_proto_highlight = true,
        ParsedToken::NoCelHighlight => opts.no_cel_highlight = true,
        ParsedToken::NoProtoMarkdown => opts.no_proto_markdown = true,
        ParsedToken::MarkdownOnly => {}
        ParsedToken::EscapeTags(mode) => opts.escape_tags = mode,
        ParsedToken::Layout(layout) => opts.layout = layout,
        ParsedToken::BookRoot(v) => {
            opts.book_root = v;
            opts.explicit_book_root = true;
        }
        ParsedToken::MarkdownRoot(v) => {
            opts.markdown_root = v;
            opts.explicit_markdown_root = true;
        }
        ParsedToken::SummaryPath(v) => {
            opts.summary_path = v;
            opts.explicit_summary_path = true;
        }
        ParsedToken::Book(v) => opts.book = Some(v),
        ParsedToken::MdbookOut(v) => opts.mdbook_out = Some(v),
        ParsedToken::Title(v) => opts.title = Some(v),
        ParsedToken::IgnoreGit(v) => opts.ignore_git = v,
        ParsedToken::LinkFormat(v) => opts.link_format = Some(v),
        ParsedToken::SearchPath(paths) => opts.search_paths = paths,
        ParsedToken::AlphabetizeServices => opts.alphabetize_services = true,
        ParsedToken::AlphabetizeMessages => opts.alphabetize_messages = true,
    }
}

fn validate_options(opts: &Options, saw_markdown_only: bool) -> Result<()> {
    if saw_markdown_only {
        eprintln!(
            "switchback-mdbook: `markdown_only` is deprecated; use `init` for a full mdBook project"
        );
    }

    if opts.no_proto_highlight && !opts.init {
        bail!("`no_proto_highlight` is only valid with `init`");
    }

    if opts.no_cel_highlight && !opts.init {
        bail!("`no_cel_highlight` is only valid with `init`");
    }

    if !opts.init {
        if opts.title.is_some() {
            bail!("`title` is only valid with `init`");
        }
        if !opts.ignore_git {
            bail!("`ignore=none` is only valid with `init`");
        }
    }

    Ok(())
}

/// Parse comma-separated mdBook plugin options from a protoc parameter string.
pub fn parse_parameter(parameter: &Option<String>) -> Result<Options> {
    let mut opts = Options::default();
    let mut saw_markdown_only = false;

    let Some(param) = parameter else {
        return Ok(opts);
    };

    for token in param.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let parsed = parse_token(token)?;
        if matches!(parsed, ParsedToken::MarkdownOnly) {
            saw_markdown_only = true;
        }
        apply_parsed(&mut opts, parsed);
    }

    validate_options(&opts, saw_markdown_only)?;
    Ok(opts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alphabetize_defaults_off() {
        let opts = parse_parameter(&None).unwrap();
        assert!(!opts.alphabetize_services);
        assert!(!opts.alphabetize_messages);
    }

    #[test]
    fn alphabetize_services_flag() {
        let opts = parse_parameter(&Some("alphabetize_services".into())).unwrap();
        assert!(opts.alphabetize_services);
        assert!(!opts.alphabetize_messages);
    }

    #[test]
    fn alphabetize_messages_flag() {
        let opts = parse_parameter(&Some("alphabetize_messages".into())).unwrap();
        assert!(!opts.alphabetize_services);
        assert!(opts.alphabetize_messages);
    }
}
