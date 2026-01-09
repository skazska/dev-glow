//! Data models for dev-glow
//!
//! This module contains all the core data structures representing
//! steps, processes, parameters, links, and classifications.

mod link;
mod parameter;
mod process;
mod step;

pub use link::{Link, LinkDefinition, LinkType};
pub use parameter::{
    ContentValue, DataType, Parameter, ParameterRef, ParameterType, ParameterValue, RangeValue,
    SetValue,
};
pub use process::{Process, ProcessDefinition};
pub use step::{Step, StepAttributes, StepDefinition, StepStatus};

/// Classification dimension for multi-dimensional grouping
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ClassificationDictionary {
    /// Unique identifier for this classification dimension
    pub id: String,
    /// Human-readable name
    #[serde(default)]
    pub name: Option<String>,
    /// Available classification values in this dimension
    pub values: Vec<ClassificationValue>,
}

/// A single classification value with optional defaults
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ClassificationValue {
    /// Classification key used in expressions
    pub key: String,
    /// Human-readable name
    #[serde(default)]
    pub name: Option<String>,
    /// Template file name to use for this classification
    #[serde(default)]
    pub template: Option<String>,
    /// Default attribute values for steps with this classification
    #[serde(default)]
    pub default_attributes: Option<serde_json::Value>,
    /// Default input parameters for this classification
    #[serde(default)]
    pub default_inputs: Vec<ParameterRef>,
    /// Default output parameters for this classification
    #[serde(default)]
    pub default_outputs: Vec<ParameterRef>,
    /// Default scope parameters for this classification
    #[serde(default)]
    pub default_scope: Vec<ParameterRef>,
}

/// Parse a classification string into components
pub fn parse_classification(classification: &str) -> Vec<String> {
    classification
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

/// Match a classification against a pattern expression
/// Pattern format: "Class1|Class2,*,Class3" where:
/// - Classes separated by commas correspond to dimensions
/// - | separates alternatives within a dimension
/// - * matches any value in a dimension
pub fn match_classification(classification: &str, pattern: &str) -> bool {
    let class_parts = parse_classification(classification);
    let pattern_parts = parse_classification(pattern);

    if class_parts.len() != pattern_parts.len() {
        return false;
    }

    class_parts
        .iter()
        .zip(pattern_parts.iter())
        .all(|(class, pattern)| {
            if pattern == "*" {
                return true;
            }
            let alternatives: Vec<&str> = pattern.split('|').map(|s| s.trim()).collect();
            alternatives.contains(&class.as_str())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_classification() {
        let result = parse_classification("Feature, Backend, Must");
        assert_eq!(result, vec!["Feature", "Backend", "Must"]);
    }

    #[test]
    fn test_match_classification_exact() {
        assert!(match_classification("Feature,Backend,Must", "Feature,Backend,Must"));
        assert!(!match_classification("Feature,Backend,Must", "Task,Backend,Must"));
    }

    #[test]
    fn test_match_classification_wildcard() {
        assert!(match_classification("Feature,Backend,Must", "*,Backend,Must"));
        assert!(match_classification("Feature,Backend,Must", "Feature,*,Must"));
        assert!(match_classification("Feature,Backend,Must", "*,*,*"));
    }

    #[test]
    fn test_match_classification_alternatives() {
        assert!(match_classification(
            "Feature,Backend,Must",
            "Feature|Task,Backend,Must"
        ));
        assert!(match_classification(
            "Task,Backend,Must",
            "Feature|Task,Backend,Must"
        ));
        assert!(!match_classification(
            "Requirement,Backend,Must",
            "Feature|Task,Backend,Must"
        ));
    }
}
