//! Parameter data model
//!
//! Parameters represent typed values used as inputs, outputs, or scope in steps.

use serde::{Deserialize, Serialize};

/// Data type for parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataType {
    /// String value
    Str,
    /// Integer number
    Int,
    /// Decimal number
    Dec,
    /// Boolean value
    Bool,
    /// ISO date
    Date,
    /// Integer range [min, max]
    RangeInt,
    /// Decimal range [min, max]
    RangeDate,
    /// Date range [start, end]
    RangeDec,
    /// Hierarchical coded set (tree structure)
    Set,
    /// Link to artifact file with mime type
    Content,
    /// Templated string with parameter references
    Template,
}

impl Default for DataType {
    fn default() -> Self {
        DataType::Str
    }
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Str => write!(f, "STR"),
            DataType::Int => write!(f, "INT"),
            DataType::Dec => write!(f, "DEC"),
            DataType::Bool => write!(f, "BOOL"),
            DataType::Date => write!(f, "DATE"),
            DataType::RangeInt => write!(f, "RANGE_INT"),
            DataType::RangeDec => write!(f, "RANGE_DEC"),
            DataType::RangeDate => write!(f, "RANGE_DATE"),
            DataType::Set => write!(f, "SET"),
            DataType::Content => write!(f, "CONTENT"),
            DataType::Template => write!(f, "TEMPLATE"),
        }
    }
}

/// Parameter type definition
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ParameterType {
    /// Unique identifier for this parameter type
    pub id: String,
    /// Description of what this parameter represents
    #[serde(default)]
    pub purpose: Option<String>,
    /// Data type of the parameter value
    pub data_type: DataType,
    /// Optional prefix to prepend when rendering
    #[serde(default)]
    pub prefix: Option<String>,
    /// Whether this parameter must have a value
    #[serde(default)]
    pub is_required: bool,
    /// Default value if not provided
    #[serde(default)]
    pub default_value: Option<serde_json::Value>,
    /// Validation rules
    #[serde(default)]
    pub validation: Option<ParameterValidation>,
}

/// Validation rules for parameter values
#[derive(Debug, Clone, Default, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ParameterValidation {
    /// Regex pattern for STR type
    #[serde(default)]
    pub pattern: Option<String>,
    /// Minimum value for numeric types
    #[serde(default)]
    pub min: Option<f64>,
    /// Maximum value for numeric types
    #[serde(default)]
    pub max: Option<f64>,
    /// Allowed values (enum)
    #[serde(default)]
    pub r#enum: Option<Vec<serde_json::Value>>,
    /// Allowed MIME types for CONTENT type
    #[serde(default)]
    pub mime_types: Option<Vec<String>>,
}

/// Reference to a parameter type with optional overrides
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ParameterRef {
    /// Parameter identifier in step scope
    pub id: String,
    /// Reference to parameter_types definition
    #[serde(default)]
    pub type_ref: Option<String>,
    /// Override purpose for this usage
    #[serde(default)]
    pub purpose: Option<String>,
    /// Override required flag
    #[serde(default)]
    pub is_required: Option<bool>,
    /// Override default value
    #[serde(default)]
    pub default_value: Option<serde_json::Value>,
    /// Template expression for deriving value from context
    #[serde(default)]
    pub mapping: Option<String>,
}

/// Runtime parameter with value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    /// Parameter definition
    pub definition: ParameterType,
    /// Current value
    pub value: Option<ParameterValue>,
}

impl Parameter {
    /// Check if this parameter has a value
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    /// Check if this parameter is required and missing
    pub fn is_missing_required(&self) -> bool {
        self.definition.is_required && self.value.is_none()
    }
}

/// Parameter value in step data
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ParameterValue {
    /// Parameter identifier
    pub id: String,
    /// Parameter value (type depends on parameter data_type)
    #[serde(default)]
    pub value: Option<serde_json::Value>,
}

impl ParameterValue {
    /// Create a new parameter value
    pub fn new(id: impl Into<String>, value: serde_json::Value) -> Self {
        Self {
            id: id.into(),
            value: Some(value),
        }
    }

    /// Create an empty parameter value
    pub fn empty(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            value: None,
        }
    }

    /// Get value as string
    pub fn as_str(&self) -> Option<&str> {
        self.value.as_ref().and_then(|v| v.as_str())
    }

    /// Get value as i64
    pub fn as_i64(&self) -> Option<i64> {
        self.value.as_ref().and_then(|v| v.as_i64())
    }

    /// Get value as f64
    pub fn as_f64(&self) -> Option<f64> {
        self.value.as_ref().and_then(|v| v.as_f64())
    }

    /// Get value as bool
    pub fn as_bool(&self) -> Option<bool> {
        self.value.as_ref().and_then(|v| v.as_bool())
    }
}

/// CONTENT type parameter value
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ContentValue {
    /// Relative path or absolute URL to artifact
    pub uri: String,
    /// MIME type of the content
    #[serde(default)]
    pub mime: Option<String>,
    /// Display title for links
    #[serde(default)]
    pub title: Option<String>,
}

impl ContentValue {
    /// Create a new content value
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            mime: None,
            title: None,
        }
    }

    /// Create with MIME type
    pub fn with_mime(mut self, mime: impl Into<String>) -> Self {
        self.mime = Some(mime.into());
        self
    }

    /// Create with title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Render as markdown link
    pub fn to_markdown_link(&self) -> String {
        let title = self.title.as_deref().unwrap_or(&self.uri);
        format!("[{}]({})", title, self.uri)
    }
}

/// SET type parameter value (hierarchical coded list)
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SetValue {
    /// Hierarchical code (dot-separated)
    pub code: String,
    /// Human-readable value
    pub value: String,
}

impl SetValue {
    /// Create a new set value
    pub fn new(code: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            value: value.into(),
        }
    }

    /// Get the depth level of this code (1-based)
    pub fn depth(&self) -> usize {
        self.code.matches('.').count() + 1
    }

    /// Check if this is a parent of another code
    pub fn is_parent_of(&self, other: &str) -> bool {
        other.starts_with(&self.code) && other.len() > self.code.len()
    }

    /// Check if this is a child of another code
    pub fn is_child_of(&self, other: &str) -> bool {
        self.code.starts_with(other) && self.code.len() > other.len()
    }
}

/// Range type parameter value [min, max]
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct RangeValue<T> {
    /// Minimum value
    pub min: T,
    /// Maximum value
    pub max: T,
}

impl<T: PartialOrd> RangeValue<T> {
    /// Create a new range value
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }

    /// Check if a value is within this range
    pub fn contains(&self, value: &T) -> bool {
        value >= &self.min && value <= &self.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_type_display() {
        assert_eq!(format!("{}", DataType::Str), "STR");
        assert_eq!(format!("{}", DataType::Content), "CONTENT");
    }

    #[test]
    fn test_content_value_markdown() {
        let content = ContentValue::new("./docs/spec.md")
            .with_title("Specification");
        assert_eq!(content.to_markdown_link(), "[Specification](./docs/spec.md)");
    }

    #[test]
    fn test_set_value_depth() {
        assert_eq!(SetValue::new("1", "Value 1").depth(), 1);
        assert_eq!(SetValue::new("1.1", "Value 1.1").depth(), 2);
        assert_eq!(SetValue::new("1.2.3", "Value 1.2.3").depth(), 3);
    }

    #[test]
    fn test_set_value_hierarchy() {
        let parent = SetValue::new("1", "Parent");
        let child = SetValue::new("1.1", "Child");

        assert!(parent.is_parent_of("1.1"));
        assert!(parent.is_parent_of("1.2.3"));
        assert!(!parent.is_parent_of("2"));

        assert!(child.is_child_of("1"));
        assert!(!child.is_child_of("2"));
    }

    #[test]
    fn test_range_contains() {
        let range = RangeValue::new(1, 10);
        assert!(range.contains(&5));
        assert!(range.contains(&1));
        assert!(range.contains(&10));
        assert!(!range.contains(&0));
        assert!(!range.contains(&11));
    }

    #[test]
    fn test_parameter_value() {
        let pv = ParameterValue::new("TEST", serde_json::json!("hello"));
        assert_eq!(pv.as_str(), Some("hello"));

        let pv = ParameterValue::new("NUM", serde_json::json!(42));
        assert_eq!(pv.as_i64(), Some(42));
    }
}
