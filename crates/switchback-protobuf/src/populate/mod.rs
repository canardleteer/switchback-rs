//! Populate switchback entities from protobuf descriptors.

pub mod comments;
pub mod fence;
pub mod source;

use std::collections::BTreeMap;
use std::path::PathBuf;

use buffa_descriptor::generated::descriptor::MethodDescriptorProto;
use buffa_descriptor::generated::descriptor::{DescriptorProto, ServiceDescriptorProto};
use switchback_traits::{
    CompanionFile, Entity, EntityBody, EntityCategory, EntityId, Group, GroupId, OperationBody,
    RefKind, Reference, SchemaBody, ServiceBody, Source, SpecVersion,
};

use crate::category::ProtobufCategory;
use crate::companion::discover_companions;
use crate::descriptor::FileDescriptorProto;
use crate::descriptor_util::split_proto_type_name;
use crate::input::ResolvedInput;
use crate::populate::comments::{dedent_comment, package_overview, CommentIndex};
use crate::populate::fence::{
    rpc_signature_plain, synthesize_enum_body, synthesize_message_body,
    synthesize_method_options_body, synthesize_service_body,
};
use crate::populate::source::SourceCache;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum EntityKind {
    Message,
    Enum,
    Service,
    Operation,
}

pub struct PopulatedEntity {
    pub entity: Entity<ProtobufCategory>,
    pub refs: Vec<Reference>,
    /// Protobuf source file path for this entity (e.g. `acme/example/v1/echo.proto`).
    pub source_file: String,
}

pub struct PopulatedContract {
    pub version: SpecVersion,
    pub module_id: String,
    pub groups: Vec<Group>,
    pub entities_by_group: BTreeMap<GroupId, Vec<PopulatedEntity>>,
    pub companions: Vec<CompanionFile>,
    pub module_root: PathBuf,
}

pub fn populate(resolved: &ResolvedInput) -> switchback_traits::Result<PopulatedContract> {
    let by_package = packages_map(&resolved.proto_file, &resolved.file_to_generate);
    let mut source = SourceCache::new(resolved.proto_search_paths.clone());
    let companions = discover_companions(
        &resolved.proto_file,
        &resolved.file_to_generate,
        &resolved.proto_search_paths,
        &resolved.module_root,
    )?;

    let module_id = default_module_id(&by_package);
    let mut groups = Vec::new();
    let mut entities_by_group = BTreeMap::new();

    for (package, files) in &by_package {
        let dir = package_dir(package);
        let overview = package_overview(files);
        let first_proto = files.first().map(|(n, _)| *n).unwrap_or("");
        groups.push(Group {
            id: GroupId::from(package.as_str()),
            dir,
            title: package.clone(),
            overview,
            source: Some(Source {
                file: first_proto.to_string(),
                span: None,
            }),
            entities: Vec::new(),
            source_path: resolved.module_root.join(first_proto),
        });

        let mut entities = Vec::new();
        for_each_entity_in_files(files, |kind, name, proto_name, file, mi, si, method_idx| {
            let idx = CommentIndex::from_file(file);
            match kind {
                EntityKind::Message => {
                    let msg = &file.message_type[mi];
                    let doc = idx
                        .leading_message(mi)
                        .map(dedent_comment)
                        .filter(|s| !s.is_empty());
                    let fence_body =
                        synthesize_message_body(proto_name, &idx, mi, msg, Some(&mut source));
                    entities.push(PopulatedEntity {
                        entity: Entity {
                            id: EntityId::new(
                                package.as_str(),
                                ProtobufCategory::Schema.as_str(),
                                name,
                            ),
                            category: ProtobufCategory::Schema,
                            title: name.to_string(),
                            doc,
                            source_span: None,
                            body: EntityBody::Schema(SchemaBody {
                                fence_language: "protobuf".into(),
                                fence_body,
                                payload_format: String::new(),
                                properties: Vec::new(),
                            }),
                        },
                        refs: message_field_refs(&module_id, msg),
                        source_file: proto_name.to_string(),
                    });
                }
                EntityKind::Enum => {
                    let en = &file.enum_type[mi];
                    let doc = idx
                        .leading_enum(mi)
                        .map(dedent_comment)
                        .filter(|s| !s.is_empty());
                    let fence_body = synthesize_enum_body(proto_name, &idx, mi, en);
                    entities.push(PopulatedEntity {
                        entity: Entity {
                            id: EntityId::new(
                                package.as_str(),
                                ProtobufCategory::Schema.as_str(),
                                name,
                            ),
                            category: ProtobufCategory::Schema,
                            title: name.to_string(),
                            doc,
                            source_span: None,
                            body: EntityBody::Schema(SchemaBody {
                                fence_language: "protobuf".into(),
                                fence_body,
                                payload_format: String::new(),
                                properties: Vec::new(),
                            }),
                        },
                        refs: Vec::new(),
                        source_file: proto_name.to_string(),
                    });
                }
                EntityKind::Service => {
                    let svc = &file.service[si];
                    let doc = idx
                        .leading_service(si)
                        .map(dedent_comment)
                        .filter(|s| !s.is_empty());
                    let fence_body = synthesize_service_body(proto_name, &idx, si, svc);
                    entities.push(PopulatedEntity {
                        entity: Entity {
                            id: EntityId::new(
                                package.as_str(),
                                ProtobufCategory::Service.as_str(),
                                name,
                            ),
                            category: ProtobufCategory::Service,
                            title: name.to_string(),
                            doc,
                            source_span: None,
                            body: EntityBody::Service(ServiceBody {
                                signature: name.to_string(),
                                fence_language: "protobuf".into(),
                                fence_body,
                            }),
                        },
                        refs: service_method_refs(&module_id, svc),
                        source_file: proto_name.to_string(),
                    });
                }
                EntityKind::Operation => {
                    let svc = &file.service[si];
                    let method = &svc.method[method_idx];
                    let service_name = svc.name.as_deref().unwrap_or("Service");
                    let rpc_name = method.name.as_deref().unwrap_or("Rpc");
                    let op_name = format!("{service_name}.{rpc_name}");
                    let doc = idx
                        .leading_method(si, method_idx)
                        .map(dedent_comment)
                        .filter(|s| !s.is_empty());
                    let signature = rpc_signature_plain(method);
                    let fence_body = synthesize_method_options_body(method).unwrap_or_default();
                    entities.push(PopulatedEntity {
                        entity: Entity {
                            id: EntityId::new(
                                package.as_str(),
                                ProtobufCategory::Operation.as_str(),
                                &op_name,
                            ),
                            category: ProtobufCategory::Operation,
                            title: rpc_name.to_string(),
                            doc,
                            source_span: None,
                            body: EntityBody::Operation(OperationBody {
                                signature,
                                fence_language: "protobuf".into(),
                                fence_body,
                                parameters: Vec::new(),
                                responses: Vec::new(),
                            }),
                        },
                        refs: operation_refs(&module_id, method),
                        source_file: proto_name.to_string(),
                    });
                }
            }
        });
        entities_by_group.insert(GroupId::from(package.as_str()), entities);
    }

    let version = detect_spec_version(&resolved.proto_file, &resolved.file_to_generate);

    Ok(PopulatedContract {
        version,
        module_id,
        groups,
        entities_by_group,
        companions,
        module_root: resolved.module_root.clone(),
    })
}

fn operation_refs(module_id: &str, method: &MethodDescriptorProto) -> Vec<Reference> {
    let mut refs = Vec::new();
    for fqn in [method.input_type.as_deref(), method.output_type.as_deref()] {
        if let Some(reference) = fqn_type_ref(module_id, fqn) {
            refs.push(reference);
        }
    }
    refs
}

fn message_field_refs(module_id: &str, msg: &DescriptorProto) -> Vec<Reference> {
    let mut refs = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for field in &msg.field {
        let Some(type_name) = field.type_name.as_deref() else {
            continue;
        };
        if let Some(reference) = fqn_type_ref(module_id, Some(type_name)) {
            let key = (
                reference.target.group.clone(),
                reference.target.name.clone(),
            );
            if seen.insert(key) {
                refs.push(reference);
            }
        }
    }
    refs
}

fn service_method_refs(module_id: &str, svc: &ServiceDescriptorProto) -> Vec<Reference> {
    let mut refs = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for method in &svc.method {
        for reference in operation_refs(module_id, method) {
            let key = (
                reference.target.group.clone(),
                reference.target.name.clone(),
            );
            if seen.insert(key) {
                refs.push(reference);
            }
        }
    }
    refs
}

fn fqn_type_ref(module_id: &str, fqn: Option<&str>) -> Option<Reference> {
    let fqn = fqn?;
    let (pkg, msg) = split_proto_type_name(fqn)?;
    if pkg.starts_with("google.protobuf") {
        return None;
    }
    Some(Reference {
        target: switchback_traits::EntityRef {
            module: module_id.to_string(),
            group: pkg.to_string(),
            category: ProtobufCategory::Schema.as_str().to_string(),
            name: msg.to_string(),
        },
        kind: RefKind::Internal,
    })
}

fn default_module_id(by_package: &BTreeMap<String, Vec<(&str, &FileDescriptorProto)>>) -> String {
    by_package
        .keys()
        .next()
        .cloned()
        .unwrap_or_else(|| "default".into())
}

fn package_dir(package: &str) -> String {
    if package.is_empty() {
        return "_root".into();
    }
    package.replace('.', "/")
}

fn detect_spec_version(
    proto_file: &[FileDescriptorProto],
    file_to_generate: &[String],
) -> SpecVersion {
    for name in file_to_generate {
        if let Some(file) = proto_file
            .iter()
            .find(|f| f.name.as_deref() == Some(name.as_str()))
        {
            if let Some(edition) = &file.edition {
                return SpecVersion::from(edition_to_spec_version(*edition));
            }
            if let Some(syntax) = &file.syntax {
                return SpecVersion::from(syntax.as_str());
            }
        }
    }
    SpecVersion::from("proto3")
}

fn edition_to_spec_version(
    edition: buffa_descriptor::generated::descriptor::Edition,
) -> &'static str {
    use buffa_descriptor::generated::descriptor::Edition;
    match edition {
        Edition::EDITION_PROTO2 => "proto2",
        Edition::EDITION_PROTO3 => "proto3",
        Edition::EDITION_2023 => "2023",
        Edition::EDITION_2024 => "2024",
        _ => "proto3",
    }
}

pub fn packages_map<'a>(
    proto_files: &'a [FileDescriptorProto],
    file_to_generate: &'a [String],
) -> BTreeMap<String, Vec<(&'a str, &'a FileDescriptorProto)>> {
    let mut by_package: BTreeMap<String, Vec<(&'a str, &'a FileDescriptorProto)>> = BTreeMap::new();
    for name in file_to_generate {
        let Some(file) = proto_files
            .iter()
            .find(|f| f.name.as_deref() == Some(name.as_str()))
        else {
            continue;
        };
        let pkg = file.package.clone().unwrap_or_default();
        by_package
            .entry(pkg)
            .or_default()
            .push((name.as_str(), file));
    }
    by_package
}

fn for_each_entity_in_files(
    files: &[(&str, &FileDescriptorProto)],
    mut f: impl FnMut(EntityKind, &str, &str, &FileDescriptorProto, usize, usize, usize),
) {
    for (proto_name, file) in files {
        for (i, msg) in file.message_type.iter().enumerate() {
            if let Some(name) = msg.name.as_deref() {
                f(EntityKind::Message, name, proto_name, file, i, 0, 0);
            }
        }
        for (i, en) in file.enum_type.iter().enumerate() {
            if let Some(name) = en.name.as_deref() {
                f(EntityKind::Enum, name, proto_name, file, i, 0, 0);
            }
        }
        for (si, svc) in file.service.iter().enumerate() {
            if let Some(name) = svc.name.as_deref() {
                f(EntityKind::Service, name, proto_name, file, 0, si, 0);
                for (mi, method) in svc.method.iter().enumerate() {
                    if method.name.is_some() {
                        f(EntityKind::Operation, name, proto_name, file, 0, si, mi);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descriptor::FileDescriptorProto;

    fn file(name: &str, package: &str) -> FileDescriptorProto {
        FileDescriptorProto {
            name: Some(name.into()),
            package: Some(package.into()),
            ..Default::default()
        }
    }

    #[test]
    fn packages_map_preserves_file_to_generate_order_within_package() {
        let a = file("pkg/a.proto", "acme.v1");
        let b = file("pkg/b.proto", "acme.v1");
        let proto_files = vec![a.clone(), b.clone()];

        let forward_inputs = ["pkg/a.proto".into(), "pkg/b.proto".into()];
        let forward = packages_map(&proto_files, &forward_inputs);
        let reverse_inputs = ["pkg/b.proto".into(), "pkg/a.proto".into()];
        let reverse = packages_map(&proto_files, &reverse_inputs);

        let forward_names: Vec<_> = forward["acme.v1"].iter().map(|(n, _)| *n).collect();
        let reverse_names: Vec<_> = reverse["acme.v1"].iter().map(|(n, _)| *n).collect();

        assert_eq!(forward_names, ["pkg/a.proto", "pkg/b.proto"]);
        assert_eq!(reverse_names, ["pkg/b.proto", "pkg/a.proto"]);
    }
}
