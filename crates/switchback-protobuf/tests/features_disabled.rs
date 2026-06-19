//! Feature-gated compiler availability checks.
//!
//! Run with:
//! `cargo test -p switchback-protobuf --no-default-features --features protoc`
//! `cargo test -p switchback-protobuf --no-default-features --features buf`

#[cfg(all(feature = "protoc", not(feature = "buf")))]
mod protoc_only {
    use switchback_protobuf::examples::fixtures_proto_dir;
    use switchback_protobuf::input::Compiler;
    use switchback_protobuf::LoadArgs;

    #[test]
    fn buf_compiler_disabled_returns_clear_error() {
        let args = LoadArgs::examples(
            fixtures_proto_dir(),
            &["acme/example/v1/echo.proto"],
            Compiler::Buf,
        );
        let err = switchback_protobuf::load(&args).expect_err("buf should be disabled");
        assert!(
            err.to_string().contains("buf"),
            "expected buf-related error, got: {err}"
        );
    }
}

#[cfg(all(feature = "buf", not(feature = "protoc")))]
mod buf_only {
    use switchback_protobuf::examples::fixtures_proto_dir;
    use switchback_protobuf::input::Compiler;
    use switchback_protobuf::LoadArgs;

    #[test]
    fn protoc_compiler_disabled_returns_clear_error() {
        let args = LoadArgs::examples(
            fixtures_proto_dir(),
            &["acme/example/v1/echo.proto"],
            Compiler::Protoc,
        );
        let err = switchback_protobuf::load(&args).expect_err("protoc should be disabled");
        assert!(
            err.to_string().contains("protoc"),
            "expected protoc-related error, got: {err}"
        );
    }
}
