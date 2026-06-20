//! Build script: compile wire-schema `.proto` with buffa-build and vendored protoc.

fn main() {
    let protoc = protoc_bin_vendored::protoc_bin_path().expect("vendored protoc");
    std::env::set_var("PROTOC", protoc);

    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let proto_root = manifest_dir.join("proto");
    let switchback_proto = proto_root
        .join("canardleteer")
        .join("switchback")
        .join("v1alpha1")
        .join("switchback.proto");
    let http_proto = proto_root
        .join("canardleteer")
        .join("switchback")
        .join("protocol")
        .join("http")
        .join("v1alpha1")
        .join("http.proto");
    let grpc_proto = proto_root
        .join("canardleteer")
        .join("switchback")
        .join("protocol")
        .join("grpc")
        .join("v1alpha1")
        .join("grpc.proto");
    let grpc_metadata_options_proto = proto_root
        .join("canardleteer")
        .join("switchback")
        .join("protocol")
        .join("grpc")
        .join("v1alpha1")
        .join("metadata_options.proto");

    for path in [
        &switchback_proto,
        &http_proto,
        &grpc_proto,
        &grpc_metadata_options_proto,
    ] {
        println!("cargo:rerun-if-changed={}", path.display());
    }
    println!(
        "cargo:rerun-if-changed={}",
        proto_root.join("buf.yaml").display()
    );

    buffa_build::Config::new()
        .files(&[
            switchback_proto,
            http_proto,
            grpc_proto,
            grpc_metadata_options_proto,
        ])
        .includes(&[proto_root])
        .include_file("_include.rs")
        .compile()
        .expect("compile switchback wire schemas");
}
