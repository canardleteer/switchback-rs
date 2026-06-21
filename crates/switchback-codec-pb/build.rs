//! Build script: compile wire-schema `.proto` with buffa-build and vendored protoc.

fn main() {
    let protoc = protoc_bin_vendored::protoc_bin_path().expect("vendored protoc");
    std::env::set_var("PROTOC", protoc);

    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let proto_root = manifest_dir.join("proto");

    // Use module-root-relative paths with `/` separators so descriptor names
    // match on Windows (buffa matches `FileDescriptorProto.name` literally).
    let switchback_proto = "canardleteer/switchback/v1alpha1/switchback.proto";
    let http_proto = "canardleteer/switchback/protocol/http/v1alpha1/http.proto";
    let grpc_proto = "canardleteer/switchback/protocol/grpc/v1alpha1/grpc.proto";
    let grpc_metadata_options_proto =
        "canardleteer/switchback/protocol/grpc/v1alpha1/metadata_options.proto";

    for path in [
        switchback_proto,
        http_proto,
        grpc_proto,
        grpc_metadata_options_proto,
    ] {
        println!("cargo:rerun-if-changed={}", proto_root.join(path).display());
    }
    println!(
        "cargo:rerun-if-changed={}",
        proto_root.join("buf.yaml").display()
    );

    std::env::set_current_dir(&proto_root).expect("chdir into proto module root");

    buffa_build::Config::new()
        .files(&[
            switchback_proto,
            http_proto,
            grpc_proto,
            grpc_metadata_options_proto,
        ])
        .includes(&["."])
        .include_file("_include.rs")
        .compile()
        .expect("compile switchback wire schemas");
}
