//! Schema validation
//!
//! Validates configuration files against JSON schemas.

use std::path::Path;

use crate::error::{GlowError, Result};

/// Embedded schemas
pub const CONFIG_SCHEMA: &str = include_str!("../../schemas/config.schema.json");
pub const PROCESS_CONFIG_SCHEMA: &str = include_str!("../../schemas/process_config.schema.json");
pub const STEP_DATA_SCHEMA: &str = include_str!("../../schemas/step_data.schema.json");
pub const MCP_CONFIG_SCHEMA: &str = include_str!("../../schemas/mcp_config.schema.json");

/// Validate a configuration value against its schema
pub fn validate_config<T: serde::Serialize>(value: &T, schema_json: &str) -> Result<()> {
    let schema: serde_json::Value =
        serde_json::from_str(schema_json).map_err(|e| GlowError::ConfigError {
            message: format!("Invalid schema: {}", e),
            source: Some(Box::new(e)),
        })?;

    let instance = serde_json::to_value(value).map_err(|e| GlowError::ConfigError {
        message: format!("Failed to serialize config: {}", e),
        source: Some(Box::new(e)),
    })?;

    let compiled =
        jsonschema::JSONSchema::compile(&schema).map_err(|e| GlowError::ConfigError {
            message: format!("Failed to compile schema: {}", e),
            source: None,
        })?;

    let result = compiled.validate(&instance);

    if let Err(errors) = result {
        let error_messages: Vec<String> = errors
            .map(|e| format!("{}: {}", e.instance_path, e))
            .collect();

        return Err(GlowError::SchemaValidationError {
            path: "config".to_string(),
            errors: error_messages,
        });
    }

    Ok(())
}

/// Validate a YAML file against a schema
pub fn validate_yaml_file(file_path: &Path, schema_json: &str) -> Result<()> {
    let content = std::fs::read_to_string(file_path).map_err(|e| GlowError::FileReadError {
        path: file_path.to_path_buf(),
        source: e,
    })?;

    let value: serde_json::Value = serde_yaml::from_str(&content)?;

    let schema: serde_json::Value =
        serde_json::from_str(schema_json).map_err(|e| GlowError::ConfigError {
            message: format!("Invalid schema: {}", e),
            source: Some(Box::new(e)),
        })?;

    let compiled =
        jsonschema::JSONSchema::compile(&schema).map_err(|e| GlowError::ConfigError {
            message: format!("Failed to compile schema: {}", e),
            source: None,
        })?;

    let result = compiled.validate(&value);

    if let Err(errors) = result {
        let error_messages: Vec<String> = errors
            .map(|e| format!("{}: {}", e.instance_path, e))
            .collect();

        return Err(GlowError::SchemaValidationError {
            path: file_path.display().to_string(),
            errors: error_messages,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, ProcessConfig};

    #[test]
    fn test_validate_default_config() {
        let config = Config::default();
        let result = validate_config(&config, CONFIG_SCHEMA);
        if let Err(e) = &result {
            eprintln!("Validation error: {:?}", e);
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_default_process_config() {
        let config = ProcessConfig::default();
        let result = validate_config(&config, PROCESS_CONFIG_SCHEMA);
        if let Err(e) = &result {
            eprintln!("Validation error: {:?}", e);
        }
        assert!(result.is_ok());
    }
}
