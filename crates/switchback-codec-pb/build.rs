//! Build script: compile wire-schema `.proto` with buffa-build and vendored protoc.

fn main() {
    let protoc = protoc_bin_vendored::protoc_bin_path().expect("vendored protoc");
    std::env::set_var("PROTOC", protoc);

    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let proto_root = manifest_dir.join("proto");
    let proto_file = proto_root
        .join("canardleteer")
        .join("switchback")
        .join("v1alpha1")
        .join("switchback.proto");

    println!("cargo:rerun-if-changed={}", proto_file.display());
    println!(
        "cargo:rerun-if-changed={}",
        proto_root.join("buf.yaml").display()
    );

    buffa_build::Config::new()
        .files(&[proto_file])
        .includes(&[proto_root])
        .include_file("_include.rs")
        .compile()
        .expect("compile switchback.proto");
}
