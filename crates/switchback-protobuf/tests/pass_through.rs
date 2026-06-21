mod common;

use common::{
    assert_sources_match_inputs, codec_roundtrip, load_examples, load_loose_fixture,
    mirrored_compilers, normalize, rebuild_buf_module, restore_sources_map, run_buf_lint_format,
};
use switchback_protobuf::examples::{EXAMPLE_PROTO_INPUTS, fixtures_proto_dir};
use switchback_protocols::{DecodedAttachment, GrpcPayloadKind, ProtocolRegistry};
use switchback_traits::EntityBody;
use tempfile::tempdir;

#[test]
fn mirrored_compile_parity_examples_corpus() {
    let compilers = mirrored_compilers();
    assert!(
        compilers.len() >= 2,
        "expected protoc and buf with default features"
    );

    let mut manuals = Vec::new();
    for compiler in compilers {
        manuals.push(normalize(load_examples(compiler)));
    }
    for manual in &manuals[1..] {
        assert_eq!(&manuals[0], manual, "compiler parity on examples corpus");
    }
}

#[test]
fn mirrored_compile_parity_doc_rich_fixture() {
    let compilers = mirrored_compilers();
    let mut manuals = Vec::new();
    for compiler in compilers {
        manuals.push(normalize(load_loose_fixture("doc_rich.proto", compiler)));
    }
    for manual in &manuals[1..] {
        assert_eq!(&manuals[0], manual, "compiler parity on doc_rich.proto");
    }
}

#[test]
fn codec_roundtrip_and_directory_faithful_proto_restore() {
    let fixture_proto_dir = fixtures_proto_dir();
    for compiler in mirrored_compilers() {
        let manual = normalize(load_examples(compiler));
        assert_sources_match_inputs(&manual, &fixture_proto_dir, EXAMPLE_PROTO_INPUTS);

        let restored = normalize(codec_roundtrip(&manual));
        assert_eq!(manual, restored, "codec round-trip ({compiler:?})");

        let temp = tempdir().expect("tempdir");
        let module_root = temp.path();
        restore_sources_map(&restored, module_root);
        for input in EXAMPLE_PROTO_INPUTS {
            let expected = std::fs::read(fixture_proto_dir.join(input))
                .unwrap_or_else(|e| panic!("read fixture {input}: {e}"));
            let got = std::fs::read(module_root.join(input))
                .unwrap_or_else(|e| panic!("read restored {input}: {e}"));
            assert_eq!(
                expected, got,
                "restored proto bytes for {input} ({compiler:?})"
            );
        }

        let rebuilt = tempdir().expect("rebuilt tempdir");
        rebuild_buf_module(&restored, &fixture_proto_dir, rebuilt.path())
            .expect("rebuild buf module");
        run_buf_lint_format(rebuilt.path());

        let reloaded_from_rebuilt = {
            let export = common::ensure_proto_deps_export();
            let args = switchback_protobuf::LoadArgs {
                compiler,
                module_root: rebuilt.path().to_path_buf(),
                inputs: EXAMPLE_PROTO_INPUTS
                    .iter()
                    .map(|p| std::path::PathBuf::from(*p))
                    .collect(),
                proto_paths: vec![rebuilt.path().to_path_buf(), export.clone()],
                protoc_path: None,
                buf_path: None,
                proto_deps_export: Some(export),
                title: None,
            };
            normalize(
                switchback_protobuf::load(&args)
                    .unwrap_or_else(|e| panic!("reload rebuilt module ({compiler:?}): {e}")),
            )
        };
        assert_eq!(
            manual, reloaded_from_rebuilt,
            "parse → wire → parse idempotency ({compiler:?})"
        );
    }
}

#[test]
fn structural_smoke_examples_corpus() {
    let manual = load_examples(mirrored_compilers()[0]);
    let contract = &manual.modules[0].contracts[0];

    assert!(!contract.groups.is_empty());
    assert_eq!(manual.sources.len(), EXAMPLE_PROTO_INPUTS.len());

    let mut categories = std::collections::BTreeSet::new();
    let mut entity_count = 0usize;
    for group in &contract.groups {
        for entity in &group.entities {
            categories.insert(entity.category.as_str());
            entity_count += 1;
        }
    }
    assert!(entity_count > 0);
    assert!(categories.contains("schema"));
    assert!(categories.contains("service"));
    assert!(categories.contains("operation"));
    assert!(
        !contract.companions.is_empty(),
        "expected README companions from examples tree"
    );

    for input in EXAMPLE_PROTO_INPUTS {
        let doc = manual
            .sources
            .iter()
            .find(|d| d.source_ref.uri == *input)
            .expect("source document");
        assert!(
            doc.source_ref.uri.contains('/'),
            "uri must preserve directories: {}",
            doc.source_ref.uri
        );
    }
}

#[test]
fn grpc_metadata_parameters_from_method_options() {
    let manual = load_examples(mirrored_compilers()[0]);
    let contract = &manual.modules[0].contracts[0];
    let registry = ProtocolRegistry::with_builtins();

    let echo_unary = contract
        .groups
        .iter()
        .find(|g| g.id.as_str() == "acme.example.v1")
        .and_then(|g| {
            g.entities
                .iter()
                .find(|e| e.name == "EchoService.EchoUnary")
        })
        .expect("EchoUnary operation");
    let EntityBody::Operation(body) = &echo_unary.body else {
        panic!("expected operation body");
    };
    let request_id = body
        .parameters
        .iter()
        .find(|p| p.name == "x-request-id")
        .expect("x-request-id metadata");
    assert_eq!(request_id.location, "metadata");
    assert!(request_id.required);
    assert_eq!(request_id.protocols.len(), 1);
    assert_eq!(request_id.protocols[0].protocol_id, "grpc");
    let decoded = registry
        .decode_attachment(&request_id.protocols[0])
        .expect("decode metadata attachment");
    let DecodedAttachment::Grpc(GrpcPayloadKind::Metadata(meta)) = decoded else {
        panic!("expected GrpcMetadataMeta payload");
    };
    assert_eq!(meta.key, "x-request-id");
    assert!(meta.required);

    let relay = contract
        .groups
        .iter()
        .find(|g| g.id.as_str() == "acme.example.v1")
        .and_then(|g| {
            g.entities
                .iter()
                .find(|e| e.name == "GatewayService.RelayConnect")
        })
        .expect("RelayConnect operation");
    let EntityBody::Operation(relay_body) = &relay.body else {
        panic!("expected operation body");
    };
    assert!(
        relay_body
            .parameters
            .iter()
            .any(|p| p.name == "x-session-token" && p.location == "metadata" && p.required)
    );
}

#[test]
fn wire_policy_serialize_succeeds_without_unresolved_links() {
    let manual = load_examples(mirrored_compilers()[0]);
    let _ = codec_roundtrip(&manual);
}
