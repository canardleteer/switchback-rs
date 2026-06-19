//! Protobuf fence body synthesis (proto source only, no markdown).

mod enum_;
mod message;
mod rpc;

pub use enum_::synthesize_enum_body;
pub use message::synthesize_message_body;
pub use rpc::{
    rpc_signature_plain, synthesize_method_options_body, synthesize_rpc_body,
    synthesize_service_body,
};

pub fn strip_leading_dot(s: &str) -> &str {
    s.strip_prefix('.').unwrap_or(s)
}

pub fn short_rpc_type(fqn: &str) -> String {
    fqn.rsplit('.').next().unwrap_or(fqn).to_string()
}

pub(crate) fn push_inline_comment_lines(out: &mut String, comment: &str) {
    for line in comment.lines() {
        out.push_str("// ");
        out.push_str(line);
        out.push('\n');
    }
}
