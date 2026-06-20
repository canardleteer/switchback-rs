//! JSON Pointer (RFC 6901) navigation.

use serde_json::Value;

/// Resolve a JSON Pointer against a document value.
///
/// Returns `None` when the pointer is invalid or missing.
pub fn resolve_pointer<'a>(value: &'a Value, pointer: &str) -> Option<&'a Value> {
    if pointer.is_empty() {
        return Some(value);
    }
    let pointer = pointer.strip_prefix('#').unwrap_or(pointer);
    let pointer = pointer.strip_prefix('/').unwrap_or(pointer);
    if pointer.is_empty() {
        return Some(value);
    }
    let mut current = value;
    for token in pointer.split('/') {
        let key = unescape_token(token);
        current = match current {
            Value::Object(map) => map.get(&key)?,
            Value::Array(arr) => {
                let idx: usize = key.parse().ok()?;
                arr.get(idx)?
            }
            _ => return None,
        };
    }
    Some(current)
}

fn unescape_token(token: &str) -> String {
    token.replace("~1", "/").replace("~0", "~")
}

/// Encode a single pointer segment.
pub fn escape_token(token: &str) -> String {
    token.replace('~', "~0").replace('/', "~1")
}
