#![forbid(unsafe_code)]

//! The mdBook renderer of the switchback-rs toolchain.
//!
//! `switchback-mdbook` turns a switchback binary file into an mdBook. It
//! implements the `Renderer` trait from `switchback-traits` as `MdBookRenderer`
//! and owns the mdBook-specific scaffolding lifted from `protobuf-mdbook`:
//! `book.toml` inference, `init` scaffolding, SUMMARY generation, and the
//! mdBook render driver. mdBook is one renderer impl, not the center of the
//! pipeline.
