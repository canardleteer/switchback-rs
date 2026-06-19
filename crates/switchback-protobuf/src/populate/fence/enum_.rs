//! Enum definition synthesis.

use super::push_inline_comment_lines;
use crate::populate::comments::CommentIndex;
use buffa_descriptor::generated::descriptor::EnumDescriptorProto;

pub fn synthesize_enum_body(
    _file_name: &str,
    idx: &CommentIndex<'_>,
    ei: usize,
    en: &EnumDescriptorProto,
) -> String {
    let name = en.name.as_deref().unwrap_or("Enum");
    let mut body = format!("enum {name} {{\n");
    for (vi, val) in en.value.iter().enumerate() {
        if let Some(c) = idx.leading_enum_value(ei, vi) {
            push_inline_comment_lines(&mut body, c);
        }
        body.push_str(&format!(
            "  {} = {};\n",
            val.name.as_deref().unwrap_or("UNKNOWN"),
            val.number.unwrap_or(0)
        ));
    }
    body.push_str("}\n");
    body
}
