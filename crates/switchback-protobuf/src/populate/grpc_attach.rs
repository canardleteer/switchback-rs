//! gRPC protocol attachment helpers for protobuf populate.

use buffa::Message;
use buffa::unknown_fields::UnknownFieldData;
use buffa_descriptor::generated::descriptor::MethodDescriptorProto;
use switchback_codec_pb::canardleteer::switchback::protocol::grpc::v1alpha1::RpcMetadataKeys;
use switchback_protocols::{GrpcProtocol, ResponseProtocol};
use switchback_traits::{ParameterRef, ProtocolAttachment, RefKind, Reference, ResponseRef};

use crate::descriptor_util::{RpcKind, rpc_kind};
use crate::populate::fence::rpc_signature_plain;

const SWITCHBACK_RPC_METADATA_FIELD: u32 = 50100;

/// Build operation signature, protocols, responses, and metadata parameters for an RPC.
pub fn populate_rpc(
    method: &MethodDescriptorProto,
    module_id: &str,
    output_fqn: &str,
) -> (
    String,
    Vec<ProtocolAttachment>,
    Vec<ResponseRef>,
    Vec<ParameterRef>,
) {
    let grpc = GrpcProtocol;
    let rpc_name = method.name.as_deref().unwrap_or("Rpc");
    let kind = rpc_kind(method);
    let (client_streaming, server_streaming) = match kind {
        RpcKind::Unary => (false, false),
        RpcKind::ClientStreaming => (true, false),
        RpcKind::ServerStreaming => (false, true),
        RpcKind::BidiStreaming => (true, true),
    };
    let meta = GrpcProtocol::operation_meta(rpc_name, client_streaming, server_streaming);
    let signature = rpc_signature_plain(method);
    let protocols = vec![grpc.attach_operation(&meta)];
    let status_meta = GrpcProtocol::status_meta_ok();
    let responses = vec![ResponseRef {
        status: "OK".into(),
        severity: grpc.response_severity("OK"),
        schema_ref: Reference {
            target: switchback_traits::EntityRef {
                module: module_id.to_string(),
                group: String::new(),
                category: "schema".into(),
                name: output_fqn.rsplit('.').next().unwrap_or("Response").into(),
            },
            kind: RefKind::Internal,
        },
        media_type: String::new(),
        description: String::new(),
        protocols: vec![grpc.attach_status(&status_meta)],
    }];
    let parameters = metadata_parameters(method, module_id);
    (signature, protocols, responses, parameters)
}

fn metadata_parameters(method: &MethodDescriptorProto, module_id: &str) -> Vec<ParameterRef> {
    let Some(keys) = rpc_metadata_keys(method) else {
        return Vec::new();
    };
    let grpc = GrpcProtocol;
    keys.keys
        .iter()
        .filter_map(|key| {
            let name = key.name.trim();
            if name.is_empty() {
                return None;
            }
            Some(ParameterRef {
                name: name.to_string(),
                location: "metadata".to_string(),
                required: key.required,
                schema_ref: inline_schema_ref(module_id),
                type_label: "string".to_string(),
                description: String::new(),
                protocols: vec![
                    grpc.attach_metadata(&GrpcProtocol::metadata_meta(name, key.required)),
                ],
            })
        })
        .collect()
}

fn rpc_metadata_keys(method: &MethodDescriptorProto) -> Option<RpcMetadataKeys> {
    let opts = method.options.as_option()?;
    for field in opts.__buffa_unknown_fields.iter() {
        if field.number != SWITCHBACK_RPC_METADATA_FIELD {
            continue;
        }
        let UnknownFieldData::LengthDelimited(bytes) = &field.data else {
            continue;
        };
        return RpcMetadataKeys::decode_from_slice(bytes).ok();
    }
    None
}

fn inline_schema_ref(module_id: &str) -> Reference {
    Reference {
        target: switchback_traits::EntityRef {
            module: module_id.to_string(),
            group: String::new(),
            category: String::new(),
            name: String::new(),
        },
        kind: RefKind::Inline,
    }
}

/// Contract-level gRPC attachment for a protobuf package.
pub fn contract_attachment(package_name: &str) -> Vec<ProtocolAttachment> {
    let grpc = GrpcProtocol;
    vec![grpc.attach_contract(&GrpcProtocol::contract_meta(package_name))]
}
