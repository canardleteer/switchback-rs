//! AsyncAPI version parsing and 2.x / 3.x detection.

use serde_json::Value;
use switchback_traits::SpecVersion;

/// Returns true when the AsyncAPI version string denotes 3.x.
pub fn is_asyncapi_3(version: &str) -> bool {
    version.starts_with('3')
}

pub fn parse_asyncapi_version(root: &Value) -> switchback_traits::Result<SpecVersion> {
    let version = root
        .get("asyncapi")
        .and_then(|v| v.as_str())
        .ok_or_else(|| switchback_traits::SwitchbackError::load("missing asyncapi version field"))?;

    if version.starts_with('1') {
        return Err(switchback_traits::SwitchbackError::load(
            "AsyncAPI 1.x is not supported; use AsyncAPI 2.x or 3.x",
        ));
    }

    if is_asyncapi_3(version) {
        Ok(SpecVersion::from("3.0.0"))
    } else if version.starts_with('2') {
        Ok(SpecVersion::from("2.6.0"))
    } else {
        Err(switchback_traits::SwitchbackError::load(format!(
            "unsupported AsyncAPI version {version}"
        )))
    }
}
