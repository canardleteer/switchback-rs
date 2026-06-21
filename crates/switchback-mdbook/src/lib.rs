#![forbid(unsafe_code)]

//! The mdBook renderer of the switchback-rs toolchain.
//!
//! `switchback-mdbook` turns a [`ReferenceManual`] into an mdBook project tree.
//! It implements [`Renderer`] and [`SyncRenderer`] as [`MdBookRenderer`].

mod book_config;
mod companion;
mod formatter;
pub mod highlight;
mod init;
mod link_check;
mod options;
mod paths;
mod render;
mod renderer;
mod summary;

pub use formatter::MdBookRelativeFormatter;
pub use link_check::{LinkError, assert_tree, check_tree};
pub use options::parse_parameter;
pub use renderer::{MdBookRenderer, write_output_files};

pub fn mdbook_version() -> &'static str {
    mdbook_preprocessor::MDBOOK_VERSION
}
