//! Optional meta-schema validation hook (not wired to CLI in v1).

use serde_json::Value;

/// Validate a document value against a meta-schema when the `validate` feature is enabled.
pub fn validate_against_schema(instance: &Value, schema: &Value) -> switchback_traits::Result<()> {
    let validator = jsonschema::validator_for(schema)
        .map_err(|e| switchback_traits::SwitchbackError::load(e.to_string()))?;
    let messages: Vec<String> = validator
        .iter_errors(instance)
        .map(|e| e.to_string())
        .collect();
    if !messages.is_empty() {
        return Err(switchback_traits::SwitchbackError::load(
            messages.join("; "),
        ));
    }
    Ok(())
}
