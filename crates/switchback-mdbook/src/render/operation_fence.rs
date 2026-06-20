//! Trim and render OpenAPI operation source fences.

use switchback_traits::OpenApiOperationSource;

use crate::render::fence::push_fence_body;

const TRIMMED_OPERATION_KEYS: &[&str] = &[
    "summary",
    "description",
    "parameters",
    "requestBody",
    "responses",
];

pub fn render_openapi_operation_fence(
    out: &mut String,
    fence_language: &str,
    fence_body: &str,
    source_mode: OpenApiOperationSource,
) {
    if fence_body.trim().is_empty() {
        return;
    }
    match source_mode {
        OpenApiOperationSource::Hidden => {}
        OpenApiOperationSource::Collapsed => {
            out.push_str("<details>\n<summary>Operation definition (YAML)</summary>\n\n");
            push_fence_body(out, fence_language, fence_body);
            out.push_str("</details>\n\n");
        }
        OpenApiOperationSource::Trimmed => {
            if let Some(trimmed) = trim_operation_fence(fence_body, fence_language) {
                push_fence_body(out, fence_language, &trimmed);
            }
        }
    }
}

fn trim_operation_fence(fence_body: &str, fence_language: &str) -> Option<String> {
    let mut value = parse_fence_value(fence_body, fence_language)?;
    let map = value.as_object_mut()?;
    for key in TRIMMED_OPERATION_KEYS {
        map.remove(*key);
    }
    if map.is_empty() {
        return None;
    }
    serialize_fence_value(&value, fence_language)
}

fn parse_fence_value(fence_body: &str, fence_language: &str) -> Option<serde_json::Value> {
    if fence_language == "yaml" || fence_language == "yml" {
        serde_saphyr::from_str(fence_body).ok()
    } else {
        serde_json::from_str(fence_body).ok()
    }
}

fn serialize_fence_value(value: &serde_json::Value, fence_language: &str) -> Option<String> {
    if fence_language == "yaml" || fence_language == "yml" {
        serde_saphyr::to_string(value).ok()
    } else {
        serde_json::to_string_pretty(value).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_removes_structured_fields() {
        let body = "summary: Example\nparameters: []\ncallbacks:\n  cb: {}\n";
        let trimmed = trim_operation_fence(body, "yaml").expect("trimmed");
        assert!(!trimmed.contains("summary:"));
        assert!(!trimmed.contains("parameters:"));
        assert!(trimmed.contains("callbacks:"));
    }
}
