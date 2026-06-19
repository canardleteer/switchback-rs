//! Build script: compile `proto/switchback.proto` with buffa-build and vendored protoc.

fn main() {
    let protoc = protoc_bin_vendored::protoc_bin_path().expect("vendored protoc");
    std::env::set_var("PROTOC", protoc);

    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.join("../..");
    let proto_root = workspace_root.join("proto");
    let proto_file = proto_root.join("switchback.proto");

    println!("cargo:rerun-if-changed={}", proto_file.display());

    buffa_build::Config::new()
        .files(&[proto_file])
        .includes(&[proto_root])
        .include_file("_include.rs")
        .compile()
        .expect("compile switchback.proto");
}
