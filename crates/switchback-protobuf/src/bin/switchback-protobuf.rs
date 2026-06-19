#![forbid(unsafe_code)]

//! `switchback-protobuf` CLI: parse .proto files into a switchback.
//!
//!   switchback-protobuf [--render mdbook] [--no-switchback] <input...>
//!
//! Always emits a switchback binary file unless `--no-switchback` is set. An optional
//! protoc-plugin stdin/stdout entry may be kept as a convenience. This binary
//! parses `.proto` files into a switchback.

fn main() {
    eprintln!("switchback-protobuf: unimplemented");
    std::process::exit(1);
}
