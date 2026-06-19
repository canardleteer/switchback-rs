//! Descriptor helpers for protobuf code generation.

use buffa_descriptor::generated::descriptor::MethodDescriptorProto;

/// gRPC / Connect streaming classification for a single `MethodDescriptorProto`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RpcKind {
    Unary,
    ClientStreaming,
    ServerStreaming,
    BidiStreaming,
}

/// Classifies RPC streaming shape from descriptor flags.
pub fn rpc_kind(method: &MethodDescriptorProto) -> RpcKind {
    let client = method.client_streaming.unwrap_or(false);
    let server = method.server_streaming.unwrap_or(false);
    match (client, server) {
        (false, false) => RpcKind::Unary,
        (true, false) => RpcKind::ClientStreaming,
        (false, true) => RpcKind::ServerStreaming,
        (true, true) => RpcKind::BidiStreaming,
    }
}

/// Split a fully-qualified protobuf type name (`.pkg.Type`) into package + message.
pub fn split_proto_type_name(fqn: &str) -> Option<(&str, &str)> {
    let s = fqn.strip_prefix('.')?;
    let (pkg, msg) = s.rsplit_once('.')?;
    if pkg.is_empty() || msg.is_empty() {
        return None;
    }
    Some((pkg, msg))
}
