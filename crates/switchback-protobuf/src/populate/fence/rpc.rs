//! Service and RPC synthesis.

use super::{short_rpc_type, strip_leading_dot};
use crate::descriptor_util::rpc_kind;
use crate::populate::comments::CommentIndex;
use buffa_descriptor::generated::descriptor::method_options::IdempotencyLevel;
use buffa_descriptor::generated::descriptor::{
    MethodDescriptorProto, ServiceDescriptorProto, UninterpretedOption,
};

pub fn synthesize_service_body(
    _file_name: &str,
    _idx: &CommentIndex<'_>,
    _si: usize,
    svc: &ServiceDescriptorProto,
) -> String {
    let name = svc.name.as_deref().unwrap_or("Service");
    let mut body = format!("service {name} {{\n");
    for method in &svc.method {
        let rpc_name = method.name.as_deref().unwrap_or("Rpc");
        let input = method
            .input_type
            .as_deref()
            .map(strip_leading_dot)
            .unwrap_or("google.protobuf.Empty");
        let output = method
            .output_type
            .as_deref()
            .map(strip_leading_dot)
            .unwrap_or("google.protobuf.Empty");
        let kind = rpc_kind(method);
        let (in_kw, out_kw) = match kind {
            crate::descriptor_util::RpcKind::Unary => ("", ""),
            crate::descriptor_util::RpcKind::ClientStreaming => ("stream ", ""),
            crate::descriptor_util::RpcKind::ServerStreaming => ("", "stream "),
            crate::descriptor_util::RpcKind::BidiStreaming => ("stream ", "stream "),
        };
        body.push_str(&format!(
            "  rpc {rpc_name} ({in_kw}{input}) returns ({out_kw}{output});\n"
        ));
    }
    body.push_str("}\n");
    body
}

pub fn synthesize_rpc_body(method: &MethodDescriptorProto) -> String {
    let name = method.name.as_deref().unwrap_or("Rpc");
    let input = method
        .input_type
        .as_deref()
        .map(strip_leading_dot)
        .unwrap_or("google.protobuf.Empty");
    let output = method
        .output_type
        .as_deref()
        .map(strip_leading_dot)
        .unwrap_or("google.protobuf.Empty");
    let kind = rpc_kind(method);
    let (in_kw, out_kw) = match kind {
        crate::descriptor_util::RpcKind::Unary => ("", ""),
        crate::descriptor_util::RpcKind::ClientStreaming => ("stream ", ""),
        crate::descriptor_util::RpcKind::ServerStreaming => ("", "stream "),
        crate::descriptor_util::RpcKind::BidiStreaming => ("stream ", "stream "),
    };
    format!("rpc {name} ({in_kw}{input}) returns ({out_kw}{output});")
}

pub fn rpc_signature_plain(method: &MethodDescriptorProto) -> String {
    let name = method.name.as_deref().unwrap_or("Rpc");
    let input_fqn = method
        .input_type
        .as_deref()
        .unwrap_or(".google.protobuf.Empty");
    let output_fqn = method
        .output_type
        .as_deref()
        .unwrap_or(".google.protobuf.Empty");
    let kind = rpc_kind(method);
    let (in_kw, out_kw) = match kind {
        crate::descriptor_util::RpcKind::Unary => ("", ""),
        crate::descriptor_util::RpcKind::ClientStreaming => ("stream ", ""),
        crate::descriptor_util::RpcKind::ServerStreaming => ("", "stream "),
        crate::descriptor_util::RpcKind::BidiStreaming => ("stream ", "stream "),
    };
    let input = short_rpc_type(strip_leading_dot(input_fqn));
    let output = short_rpc_type(strip_leading_dot(output_fqn));
    format!("{name} ( {in_kw}{input} ) returns ( {out_kw}{output} )")
}

pub fn synthesize_method_options_body(method: &MethodDescriptorProto) -> Option<String> {
    let opts = method.options.as_option()?;
    let mut lines = Vec::new();
    if opts.deprecated == Some(true) {
        lines.push("option deprecated = true;".to_string());
    }
    if let Some(level) = opts.idempotency_level {
        if let Some(name) = idempotency_level_name(level) {
            lines.push(format!("option idempotency_level = {name};"));
        }
    }
    for uo in &opts.uninterpreted_option {
        if let Some(line) = format_uninterpreted_option(uo) {
            lines.push(line);
        }
    }
    if lines.is_empty() {
        None
    } else {
        Some(lines.join("\n"))
    }
}

fn idempotency_level_name(level: IdempotencyLevel) -> Option<&'static str> {
    match level {
        IdempotencyLevel::NO_SIDE_EFFECTS => Some("NO_SIDE_EFFECTS"),
        IdempotencyLevel::IDEMPOTENT => Some("IDEMPOTENT"),
        IdempotencyLevel::IDEMPOTENCY_UNKNOWN => None,
    }
}

fn format_uninterpreted_option(opt: &UninterpretedOption) -> Option<String> {
    let mut name = String::new();
    for part in &opt.name {
        let piece = &part.name_part;
        if part.is_extension {
            name.push('(');
            name.push_str(piece);
            name.push(')');
        } else if !name.is_empty() {
            name.push('.');
            name.push_str(piece);
        } else {
            name.push_str(piece);
        }
    }
    if name.is_empty() {
        return None;
    }
    let value = opt
        .identifier_value
        .as_deref()
        .map(|s| s.to_string())
        .or_else(|| opt.positive_int_value.map(|n| n.to_string()))
        .or_else(|| opt.negative_int_value.map(|n| n.to_string()))
        .or_else(|| opt.double_value.map(|n| n.to_string()))
        .or_else(|| {
            opt.string_value
                .as_ref()
                .and_then(|b| std::str::from_utf8(b).ok().map(|s| format!("\"{s}\"")))
        })
        .or_else(|| opt.aggregate_value.clone())?;
    Some(format!("option {name} = {value};"))
}
