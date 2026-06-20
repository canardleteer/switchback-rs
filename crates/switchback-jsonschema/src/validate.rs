//! Optional meta-schema validation hook (not wired to CLI in v1).

use serde_json::Value;

/// Validate a document value against a meta-schema when the `validate` feature is enabled.
pub fn validate_against_schema(instance: &Value, schema: &Value) -> switchback_traits::Result<()> {
    let validator = jsonschema::validator_for(schema)
        .map_err(|e| switchback_traits::SwitchbackError::load(e.to_string()))?;
    let output = validator.validate(instance);
    if let Err(errors) = output {
        let messages: Vec<String> = errors.map(|e| e.to_string()).collect();
        return Err(switchback_traits::SwitchbackError::load(
            messages.join("; "),
        ));
    }
    Ok(())
}
