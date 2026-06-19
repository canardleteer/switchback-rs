#![forbid(unsafe_code)]

//! `switchback-mdbook` CLI: read a switchback and render an mdBook.
//!
//! Standalone renderer entry point:
//!   `switchback-mdbook switchback.binpb`
//!
//! A parser may also call this renderer in-process
//! (`switchback-openapi --render mdbook openapi.yaml`).

fn main() {
    eprintln!("switchback-mdbook: unimplemented");
    std::process::exit(1);
}
