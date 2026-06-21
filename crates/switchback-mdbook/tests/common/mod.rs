//! Shared helpers for switchback-mdbook integration tests.

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use switchback_mdbook::{MdBookRenderer, parse_parameter, write_output_files};
use switchback_protobuf::Compiler;
use switchback_protobuf::examples::{EXAMPLE_PROTO_INPUTS, fixtures_proto_dir};
use switchback_protobuf::load::{LoadArgs, ensure_test_proto_deps, load};
use switchback_traits::{Layout, Options, ReferenceManual, SyncRenderer};

pub fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn ensure_proto_deps_export() -> PathBuf {
    ensure_test_proto_deps(&fixtures_proto_dir(), None).expect("ensure proto deps export")
}

pub fn load_examples() -> ReferenceManual {
    let module_root = fixtures_proto_dir();
    let export = ensure_proto_deps_export();
    let args = LoadArgs {
        compiler: Compiler::Buf,
        module_root: module_root.clone(),
        inputs: EXAMPLE_PROTO_INPUTS
            .iter()
            .map(|p| PathBuf::from(*p))
            .collect(),
        proto_paths: vec![module_root.clone(), export.clone()],
        protoc_path: None,
        buf_path: None,
        proto_deps_export: Some(export),
        title: None,
    };
    load(&args).expect("load examples")
}

pub fn load_fixture(name: &str) -> ReferenceManual {
    let fixture_dir = fixtures_proto_dir()
        .parent()
        .expect("fixtures parent")
        .join("loose");
    let args = LoadArgs {
        compiler: Compiler::Protoc,
        module_root: fixture_dir.clone(),
        inputs: vec![PathBuf::from(name)],
        proto_paths: vec![fixture_dir.clone()],
        protoc_path: None,
        buf_path: None,
        proto_deps_export: None,
        title: None,
    };
    load(&args).expect("load fixture")
}

pub fn load_openapi_tictactoe() -> ReferenceManual {
    use switchback_openapi::examples::{UPSTREAM_LOW_3_1, example_fixture, load_example};

    load_example(example_fixture("tictactoe-3.1").expect("tictactoe fixture")).unwrap_or_else(
        |e| panic!("load {UPSTREAM_LOW_3_1}: {e} (run cargo xtask spec-vendor fetch-fixtures --family openapi)"),
    )
}

pub fn load_openapi_acme() -> ReferenceManual {
    switchback_openapi::load_acme_example().expect("load acme-api")
}

pub fn load_asyncapi_acme() -> ReferenceManual {
    switchback_asyncapi::load_acme_example().expect("load acme events")
}

pub fn load_asyncapi_streetlights() -> ReferenceManual {
    use switchback_asyncapi::examples::{example_fixture, load_example};

    let fixture = example_fixture("streetlights-kafka").unwrap_or_else(|| {
        panic!("streetlights-kafka fixture not registered in switchback-asyncapi")
    });
    load_example(fixture).unwrap_or_else(|e| {
        panic!(
            "load streetlights-kafka: {e} (run cargo xtask spec-vendor fetch-fixtures --family asyncapi)"
        )
    })
}

pub fn render_asyncapi(layout: Layout, extra: &str) -> tempfile::TempDir {
    let manual = load_asyncapi_streetlights();
    let mut param = format!("layout={}", layout_name(layout));
    if !extra.is_empty() {
        param.push(',');
        param.push_str(extra);
    }
    let opts = parse_parameter(&Some(param)).expect("parse options");
    render_to_tempdir(&manual, &opts)
}

pub fn render_asyncapi_acme(layout: Layout, extra: &str) -> tempfile::TempDir {
    let manual = load_asyncapi_acme();
    let mut param = format!("layout={}", layout_name(layout));
    if !extra.is_empty() {
        param.push(',');
        param.push_str(extra);
    }
    let opts = parse_parameter(&Some(param)).expect("parse options");
    render_to_tempdir(&manual, &opts)
}

pub fn load_reference_manual_acme_v1() -> ReferenceManual {
    use std::path::PathBuf;

    use switchback_assemble::{AssembleArgs, GroupPrefixPolicy, assemble_module};
    use switchback_openapi::examples::{EXAMPLE_ACME_INPUTS, MICRO_ACME_ROOT, fixtures_dir};
    use switchback_openapi::load::LoadArgs as OpenApiLoadArgs;
    use switchback_protobuf::examples::EXAMPLE_PROTO_INPUTS;
    use switchback_protobuf::load::LoadArgs as ProtobufLoadArgs;

    let openapi_root = fixtures_dir().join(MICRO_ACME_ROOT);
    let proto_root = fixtures_proto_dir();
    let export = ensure_proto_deps_export();
    assemble_module(&AssembleArgs {
        module_id: "acme".into(),
        title: "Acme APIs".into(),
        overview: "Acme HTTP + gRPC reference (v1, v2, v3alpha1).".into(),
        group_prefix: GroupPrefixPolicy::ContractFamily,
        openapi: Some(OpenApiLoadArgs {
            module_root: openapi_root.clone(),
            inputs: EXAMPLE_ACME_INPUTS
                .iter()
                .map(|p| PathBuf::from(*p))
                .collect(),
            search_roots: vec![openapi_root],
            title: None,
        }),
        protobuf: Some(ProtobufLoadArgs {
            compiler: Compiler::Buf,
            module_root: proto_root.clone(),
            inputs: EXAMPLE_PROTO_INPUTS
                .iter()
                .map(|p| PathBuf::from(*p))
                .collect(),
            proto_paths: vec![proto_root.clone(), export.clone()],
            protoc_path: None,
            buf_path: None,
            proto_deps_export: Some(export),
            title: None,
        }),
        asyncapi: None,
    })
    .expect("assemble acme reference manual")
}

pub fn render_openapi(layout: Layout, extra: &str) -> tempfile::TempDir {
    let manual = load_openapi_tictactoe();
    let mut param = format!("layout={}", layout_name(layout));
    if !extra.is_empty() {
        param.push(',');
        param.push_str(extra);
    }
    let opts = parse_parameter(&Some(param)).expect("parse options");
    render_to_tempdir(&manual, &opts)
}

pub fn render_openapi_acme(layout: Layout, extra: &str) -> tempfile::TempDir {
    let manual = load_openapi_acme();
    let mut param = format!("layout={}", layout_name(layout));
    if !extra.is_empty() {
        param.push(',');
        param.push_str(extra);
    }
    let opts = parse_parameter(&Some(param)).expect("parse options");
    render_to_tempdir(&manual, &opts)
}

pub fn options_for(layout: Layout, extra: &str) -> Options {
    let mut param = format!("layout={}", layout_name(layout));
    if !extra.is_empty() {
        param.push(',');
        param.push_str(extra);
    }
    parse_parameter(&Some(param)).expect("parse options")
}

fn layout_name(layout: Layout) -> &'static str {
    match layout {
        Layout::Package => "package",
        Layout::Entity => "entity",
        Layout::Split => "split",
    }
}

pub fn render_manual(manual: &ReferenceManual, opts: &Options) -> BTreeMap<String, String> {
    let dir = render_to_tempdir(manual, opts);
    collect_tree(dir.path())
}

pub fn render_to_tempdir(manual: &ReferenceManual, opts: &Options) -> tempfile::TempDir {
    let files = MdBookRenderer.render(manual, opts).expect("render");
    let dir = tempfile::tempdir().expect("tempdir");
    write_output_files(dir.path(), &files).expect("write");
    dir
}

pub fn render_examples(layout: Layout, extra: &str) -> tempfile::TempDir {
    let manual = load_examples();
    let opts = options_for(layout, extra);
    render_to_tempdir(&manual, &opts)
}

pub fn render_fixture(name: &str, extra: &str) -> tempfile::TempDir {
    let manual = load_fixture(name);
    let opts = parse_parameter(&Some(extra.into())).expect("parse options");
    render_to_tempdir(&manual, &opts)
}

pub fn render_doc_rich(extra: &str) -> tempfile::TempDir {
    render_fixture("doc_rich.proto", extra)
}

pub fn collect_tree(root: &Path) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    collect_tree_inner(root, root, &mut out);
    out
}

fn collect_tree_inner(root: &Path, dir: &Path, out: &mut BTreeMap<String, String>) {
    for entry in std::fs::read_dir(dir).expect("read_dir") {
        let entry = entry.expect("entry");
        let path = entry.path();
        if path.is_dir() {
            collect_tree_inner(root, &path, out);
        } else {
            let rel = path
                .strip_prefix(root)
                .expect("strip")
                .to_string_lossy()
                .replace('\\', "/");
            let content = std::fs::read_to_string(&path).expect("read");
            out.insert(rel, content.replace("\r\n", "\n").replace('\r', "\n"));
        }
    }
}
