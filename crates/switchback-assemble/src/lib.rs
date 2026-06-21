//! Merge family-specific [`ReferenceManual`] fragments into one module.
//!
//! See [ADR 0014](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0014-multi-contract-reference-manual-assembly.md).

use switchback_asyncapi::load::LoadArgs as AsyncApiLoadArgs;
use switchback_openapi::load::LoadArgs as OpenApiLoadArgs;
use switchback_openrpc::load::LoadArgs as OpenRpcLoadArgs;
use switchback_protobuf::load::LoadArgs as ProtobufLoadArgs;
use switchback_traits::{
    EntityBody, EntityRef, GroupId, IntraLink, LinkTarget, ManualContract, Module, ModuleId,
    Reference, ReferenceManual,
};

/// How to namespace group ids when merging contracts that share package names.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GroupPrefixPolicy {
    /// Keep group ids from each family load unchanged.
    #[default]
    None,
    /// Prefix each group as `{family}.{original_group_id}`.
    ContractFamily,
}

/// Arguments for [`assemble_module`].
#[derive(Clone, Debug)]
pub struct AssembleArgs {
    pub module_id: String,
    pub title: String,
    pub overview: String,
    pub group_prefix: GroupPrefixPolicy,
    pub openapi: Option<OpenApiLoadArgs>,
    pub protobuf: Option<ProtobufLoadArgs>,
    pub asyncapi: Option<AsyncApiLoadArgs>,
    pub openrpc: Option<OpenRpcLoadArgs>,
}

/// Load each configured family and merge into one reference manual.
pub fn assemble_module(args: &AssembleArgs) -> switchback_traits::Result<ReferenceManual> {
    let mut contracts = Vec::new();
    let mut sources = Vec::new();
    let mut switchback_version = String::new();

    if let Some(openapi) = &args.openapi {
        let ReferenceManual {
            switchback_version: sv,
            sources: manual_sources,
            modules,
            ..
        } = switchback_openapi::load(openapi)?;
        if switchback_version.is_empty() {
            switchback_version = sv;
        }
        sources.extend(manual_sources);
        contracts.extend(modules.into_iter().flat_map(|m| m.contracts));
    }

    if let Some(protobuf) = &args.protobuf {
        let ReferenceManual {
            switchback_version: sv,
            sources: manual_sources,
            modules,
            ..
        } = switchback_protobuf::load(protobuf)?;
        if switchback_version.is_empty() {
            switchback_version = sv;
        }
        sources.extend(manual_sources);
        contracts.extend(modules.into_iter().flat_map(|m| m.contracts));
    }

    if let Some(asyncapi) = &args.asyncapi {
        let ReferenceManual {
            switchback_version: sv,
            sources: manual_sources,
            modules,
            ..
        } = switchback_asyncapi::load(asyncapi)?;
        if switchback_version.is_empty() {
            switchback_version = sv;
        }
        sources.extend(manual_sources);
        contracts.extend(modules.into_iter().flat_map(|m| m.contracts));
    }

    if let Some(openrpc) = &args.openrpc {
        let ReferenceManual {
            switchback_version: sv,
            sources: manual_sources,
            modules,
            ..
        } = switchback_openrpc::load(openrpc)?;
        if switchback_version.is_empty() {
            switchback_version = sv;
        }
        sources.extend(manual_sources);
        contracts.extend(modules.into_iter().flat_map(|m| m.contracts));
    }

    if contracts.is_empty() {
        return Err(switchback_traits::SwitchbackError::load(
            "assemble_module: at least one family load args required",
        ));
    }

    let module_id = ModuleId::from(args.module_id.as_str());
    rewrite_module_id(&mut contracts, &module_id);
    for contract in &mut contracts {
        apply_group_prefix(contract, &module_id, args.group_prefix);
    }

    Ok(ReferenceManual {
        switchback_version,
        title: args.title.clone(),
        sources,
        modules: vec![Module {
            id: module_id,
            title: args.title.clone(),
            overview: args.overview.clone(),
            contracts,
        }],
    })
}

fn apply_group_prefix(
    contract: &mut ManualContract,
    module_id: &ModuleId,
    policy: GroupPrefixPolicy,
) {
    if !matches!(policy, GroupPrefixPolicy::ContractFamily) {
        return;
    }
    let family = contract.family.clone();
    let mappings: Vec<(String, String)> = contract
        .groups
        .iter()
        .map(|group| {
            let old_id = group.id.as_str().to_string();
            (old_id.clone(), format!("{family}.{old_id}"))
        })
        .collect();

    for group in &mut contract.groups {
        let old_id = group.id.as_str().to_string();
        let new_id = format!("{family}.{old_id}");
        group.id = GroupId::from(new_id.as_str());
    }

    let module = module_id.as_str();
    for (old_group, new_group) in mappings {
        for group in &mut contract.groups {
            for entity in &mut group.entities {
                rewrite_entity_refs(entity, module, &old_group, &new_group);
            }
        }
    }
}

fn rewrite_module_id(contracts: &mut [ManualContract], module_id: &ModuleId) {
    let module = module_id.as_str();
    for contract in contracts {
        for group in &mut contract.groups {
            for entity in &mut group.entities {
                rewrite_entity_module(entity, module);
            }
        }
    }
}

fn rewrite_entity_module(entity: &mut switchback_traits::StoredEntity, module: &str) {
    for reference in &mut entity.refs {
        reference.target.module = module.to_string();
    }
    rewrite_entity_body_module(entity, module);
    rewrite_intra_link_module(&mut entity.intra_links, module);
}

fn rewrite_entity_refs(
    entity: &mut switchback_traits::StoredEntity,
    module: &str,
    old_group: &str,
    new_group: &str,
) {
    for reference in &mut entity.refs {
        rewrite_reference_group(reference, module, old_group, new_group);
    }
    rewrite_entity_body_group(entity, module, old_group, new_group);
    for link in &mut entity.intra_links {
        rewrite_intra_link_group(link, module, old_group, new_group);
    }
}

fn rewrite_reference_group(
    reference: &mut Reference,
    module: &str,
    old_group: &str,
    new_group: &str,
) {
    if reference.target.module == module && reference.target.group == old_group {
        reference.target.group = new_group.to_string();
    }
}

fn rewrite_entity_body_module(entity: &mut switchback_traits::StoredEntity, module: &str) {
    rewrite_entity_body_references(entity, |reference| {
        reference.target.module = module.to_string();
    });
}

fn rewrite_entity_body_group(
    entity: &mut switchback_traits::StoredEntity,
    module: &str,
    old_group: &str,
    new_group: &str,
) {
    rewrite_entity_body_references(entity, |reference| {
        rewrite_reference_group(reference, module, old_group, new_group);
    });
}

fn rewrite_entity_body_references(
    entity: &mut switchback_traits::StoredEntity,
    mut rewrite: impl FnMut(&mut Reference),
) {
    match &mut entity.body {
        EntityBody::Operation(body) => {
            for param in &mut body.parameters {
                rewrite(&mut param.schema_ref);
            }
            for response in &mut body.responses {
                rewrite(&mut response.schema_ref);
            }
            if let Some(request_body) = &mut body.request_body {
                rewrite(&mut request_body.schema_ref);
            }
        }
        EntityBody::Schema(body) => {
            for property in &mut body.properties {
                rewrite(&mut property.schema_ref);
            }
        }
        _ => {}
    }
}

fn rewrite_intra_link_module(links: &mut [IntraLink], module: &str) {
    for link in links {
        match &mut link.target {
            LinkTarget::Entity(EntityRef { module: m, .. }) => *m = module.to_string(),
            LinkTarget::Group(group_ref) => group_ref.module = module.to_string(),
            LinkTarget::Contract(contract_ref) => contract_ref.module = module.to_string(),
            LinkTarget::Module(module_ref) => module_ref.module = module.to_string(),
            LinkTarget::Manual(_) | LinkTarget::External(_) | LinkTarget::Unresolved => {}
        }
    }
}

fn rewrite_intra_link_group(link: &mut IntraLink, module: &str, old_group: &str, new_group: &str) {
    match &mut link.target {
        LinkTarget::Entity(EntityRef {
            module: m,
            group: g,
            ..
        }) if m == module && g == old_group => {
            *g = new_group.to_string();
        }
        LinkTarget::Group(group_ref)
            if group_ref.module == module && group_ref.group == old_group =>
        {
            group_ref.group = new_group.to_string();
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use switchback_asyncapi::examples::{
        EXAMPLE_ACME_INPUTS as ASYNCAPI_ACME_INPUTS, MICRO_ACME_ROOT as ASYNCAPI_ACME_ROOT,
        fixtures_dir as asyncapi_fixtures_dir,
    };
    use switchback_openapi::examples::{EXAMPLE_ACME_INPUTS, MICRO_ACME_ROOT, fixtures_dir};
    use switchback_openrpc::examples::{
        EXAMPLE_ACME_INPUTS as OPENRPC_ACME_INPUTS, MICRO_ACME_ROOT as OPENRPC_ACME_ROOT,
        fixtures_dir as openrpc_fixtures_dir,
    };
    use switchback_protobuf::Compiler;
    use switchback_protobuf::examples::EXAMPLE_PROTO_INPUTS;
    use switchback_protobuf::examples::fixtures_proto_dir;
    use switchback_protobuf::load::{LoadArgs as ProtobufLoadArgs, ensure_test_proto_deps};

    use super::*;

    #[test]
    fn assembles_acme_openapi_protobuf_asyncapi_and_openrpc() {
        let openapi_root = fixtures_dir().join(MICRO_ACME_ROOT);
        let asyncapi_root = asyncapi_fixtures_dir().join(ASYNCAPI_ACME_ROOT);
        let openrpc_root = openrpc_fixtures_dir().join(OPENRPC_ACME_ROOT);
        let proto_root = fixtures_proto_dir();
        let export = ensure_test_proto_deps(&proto_root, None).expect("proto deps");

        let manual = assemble_module(&AssembleArgs {
            module_id: "acme".into(),
            title: "Acme APIs".into(),
            overview: "Acme HTTP + gRPC + events + JSON-RPC".into(),
            group_prefix: GroupPrefixPolicy::ContractFamily,
            openapi: Some(OpenApiLoadArgs {
                module_root: openapi_root.clone(),
                inputs: EXAMPLE_ACME_INPUTS.iter().map(PathBuf::from).collect(),
                search_roots: vec![openapi_root],
                title: None,
            }),
            protobuf: Some(ProtobufLoadArgs {
                compiler: Compiler::Buf,
                module_root: proto_root.clone(),
                inputs: EXAMPLE_PROTO_INPUTS.iter().map(PathBuf::from).collect(),
                proto_paths: vec![proto_root.clone(), export.clone()],
                protoc_path: None,
                buf_path: None,
                proto_deps_export: Some(export),
                title: None,
            }),
            asyncapi: Some(AsyncApiLoadArgs {
                module_root: asyncapi_root.clone(),
                inputs: ASYNCAPI_ACME_INPUTS.iter().map(PathBuf::from).collect(),
                search_roots: vec![asyncapi_root],
                title: None,
            }),
            openrpc: Some(OpenRpcLoadArgs {
                module_root: openrpc_root.clone(),
                inputs: OPENRPC_ACME_INPUTS.iter().map(PathBuf::from).collect(),
                search_roots: vec![openrpc_root],
                title: None,
            }),
        })
        .expect("assemble");

        assert_eq!(manual.modules.len(), 1);
        assert_eq!(manual.modules[0].contracts.len(), 4);
        let families: Vec<_> = manual.modules[0]
            .contracts
            .iter()
            .map(|c| c.family.as_str())
            .collect();
        assert!(families.contains(&"openapi"));
        assert!(families.contains(&"protobuf"));
        assert!(families.contains(&"asyncapi"));
        assert!(families.contains(&"openrpc"));

        let group_ids: Vec<_> = manual.modules[0]
            .contracts
            .iter()
            .flat_map(|c| c.groups.iter().map(|g| g.id.as_str().to_string()))
            .collect();
        for suffix in ["v1", "v2", "v3alpha1"] {
            assert!(
                group_ids
                    .iter()
                    .any(|id| id == &format!("openapi.acme.example.{suffix}")),
                "missing openapi.acme.example.{suffix} in {group_ids:?}"
            );
            assert!(
                group_ids
                    .iter()
                    .any(|id| id == &format!("protobuf.acme.example.{suffix}")),
                "missing protobuf.acme.example.{suffix} in {group_ids:?}"
            );
            assert!(
                group_ids
                    .iter()
                    .any(|id| id == &format!("asyncapi.acme.example.{suffix}")),
                "missing asyncapi.acme.example.{suffix} in {group_ids:?}"
            );
            assert!(
                group_ids
                    .iter()
                    .any(|id| id == &format!("openrpc.acme.example.{suffix}")),
                "missing openrpc.acme.example.{suffix} in {group_ids:?}"
            );
        }

        let protobuf = manual.modules[0]
            .contracts
            .iter()
            .find(|c| c.family == "protobuf")
            .expect("protobuf contract");
        let v1 = protobuf
            .groups
            .iter()
            .find(|g| g.id.as_str() == "protobuf.acme.example.v1")
            .expect("v1 group");
        let echo_stream = v1
            .entities
            .iter()
            .find(|e| e.name == "EchoService.EchoServerStream")
            .expect("EchoService.EchoServerStream operation");
        assert!(
            echo_stream.refs.iter().any(|r| {
                r.target.module == "acme"
                    && r.target.group == "protobuf.acme.example.v1"
                    && r.target.name == "EchoServerStreamRequest"
            }),
            "expected prefixed structural ref on EchoServerStream: {:?}",
            echo_stream.refs
        );

        let openapi = manual.modules[0]
            .contracts
            .iter()
            .find(|c| c.family == "openapi")
            .expect("openapi contract");
        let v2 = openapi
            .groups
            .iter()
            .find(|g| g.id.as_str() == "openapi.acme.example.v2")
            .expect("v2 group");
        let list_products = v2
            .entities
            .iter()
            .find(|e| e.name == "GET /products")
            .expect("GET /products operation");
        let EntityBody::Operation(body) = &list_products.body else {
            panic!("expected operation body");
        };
        let response = body
            .responses
            .iter()
            .find(|r| r.status == "200")
            .expect("200 response");
        assert_eq!(
            response.schema_ref.target.group, "openapi.acme.example.v2",
            "expected prefixed group on response schema_ref: {:?}",
            response.schema_ref.target
        );

        let asyncapi = manual.modules[0]
            .contracts
            .iter()
            .find(|c| c.family == "asyncapi")
            .expect("asyncapi contract");
        let async_v1 = asyncapi
            .groups
            .iter()
            .find(|g| g.id.as_str() == "asyncapi.acme.example.v1")
            .expect("asyncapi v1 group");
        assert!(
            async_v1
                .entities
                .iter()
                .any(|e| e.name == "publishEchoUnary"),
            "expected publishEchoUnary operation in asyncapi v1"
        );
    }
}
