//! Error types for glow-core

use std::path::PathBuf;
use thiserror::Error;

/// Result type alias for glow operations
pub type Result<T> = std::result::Result<T, GlowError>;

/// Main error type for glow-core
#[derive(Error, Debug)]
pub enum GlowError {
    // Configuration errors
    #[error("Configuration error: {message}")]
    ConfigError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Configuration file not found: {path}")]
    ConfigNotFound { path: PathBuf },

    #[error("Invalid configuration at {path}: {message}")]
    InvalidConfig { path: String, message: String },

    // Validation errors
    #[error("Validation error: {message}")]
    ValidationError { message: String },

    #[error("Schema validation failed at {path}: {errors:?}")]
    SchemaValidationError { path: String, errors: Vec<String> },

    // State errors
    #[error("Invalid state transition: cannot move step '{step_id}' from '{current}' to '{target}'")]
    InvalidStateTransition {
        step_id: String,
        current: String,
        target: String,
    },

    #[error("Step '{step_id}' is blocked by dependencies: {dependencies:?}")]
    BlockedByDependencies {
        step_id: String,
        dependencies: Vec<String>,
    },

    #[error("Cannot start new iteration: current iteration not complete")]
    IterationNotComplete { step_id: String },

    // Not found errors
    #[error("Step not found: {fqid}")]
    StepNotFound { fqid: String },

    #[error("Parameter not found: {param_id} in step {step_id}")]
    ParameterNotFound { step_id: String, param_id: String },

    #[error("Template not found: {template}")]
    TemplateNotFound { template: String },

    #[error("Link target not found: {target_id} in step {step_id}")]
    LinkTargetNotFound { step_id: String, target_id: String },

    // IO errors
    #[error("IO error: {message}")]
    IoError {
        message: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to read file: {path}")]
    FileReadError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write file: {path}")]
    FileWriteError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    // Template errors
    #[error("Template rendering error: {message}")]
    TemplateError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    // Parse errors
    #[error("YAML parse error: {message}")]
    YamlParseError {
        message: String,
        #[source]
        source: serde_yaml::Error,
    },

    #[error("JSON parse error: {message}")]
    JsonParseError {
        message: String,
        #[source]
        source: serde_json::Error,
    },

    #[error("Invalid frontmatter in file: {path}")]
    InvalidFrontmatter { path: PathBuf },

    // Process errors
    #[error("Missing required parameter: {param_id}")]
    MissingRequiredParameter { param_id: String },

    #[error("Missing required output: {param_id}")]
    MissingRequiredOutput { param_id: String },

    #[error("Circular dependency detected: {cycle:?}")]
    CircularDependency { cycle: Vec<String> },

    #[error("Project already initialized in {path}")]
    ProjectAlreadyExists { path: PathBuf },

    #[error("Project not initialized. Run 'glow project init' first.")]
    ProjectNotInitialized,

    // Quality errors
    #[error("Context quality issue: {message}")]
    QualityError { message: String },
}

impl GlowError {
    /// Get the error category for exit codes
    pub fn category(&self) -> ErrorCategory {
        match self {
            GlowError::ConfigError { .. }
            | GlowError::ConfigNotFound { .. }
            | GlowError::InvalidConfig { .. } => ErrorCategory::Config,

            GlowError::ValidationError { .. }
            | GlowError::SchemaValidationError { .. } => ErrorCategory::Validation,

            GlowError::InvalidStateTransition { .. }
            | GlowError::BlockedByDependencies { .. }
            | GlowError::IterationNotComplete { .. } => ErrorCategory::State,

            GlowError::StepNotFound { .. }
            | GlowError::ParameterNotFound { .. }
            | GlowError::TemplateNotFound { .. }
            | GlowError::LinkTargetNotFound { .. } => ErrorCategory::NotFound,

            GlowError::IoError { .. }
            | GlowError::FileReadError { .. }
            | GlowError::FileWriteError { .. } => ErrorCategory::Io,

            GlowError::TemplateError { .. }
            | GlowError::YamlParseError { .. }
            | GlowError::JsonParseError { .. }
            | GlowError::InvalidFrontmatter { .. } => ErrorCategory::Parse,

            GlowError::MissingRequiredParameter { .. }
            | GlowError::MissingRequiredOutput { .. }
            | GlowError::CircularDependency { .. }
            | GlowError::ProjectAlreadyExists { .. }
            | GlowError::ProjectNotInitialized => ErrorCategory::Process,

            GlowError::QualityError { .. } => ErrorCategory::Quality,
        }
    }

    /// Get the exit code for this error
    pub fn exit_code(&self) -> i32 {
        self.category().exit_code()
    }
}

/// Error categories for grouping errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    Config,
    Validation,
    State,
    NotFound,
    Io,
    Parse,
    Process,
    Quality,
}

impl ErrorCategory {
    /// Get the exit code for this category
    pub fn exit_code(&self) -> i32 {
        match self {
            ErrorCategory::Config => 1,
            ErrorCategory::Validation => 2,
            ErrorCategory::State => 3,
            ErrorCategory::NotFound => 4,
            ErrorCategory::Io => 5,
            ErrorCategory::Parse => 6,
            ErrorCategory::Process => 7,
            ErrorCategory::Quality => 8,
        }
    }

    /// Get the category name
    pub fn name(&self) -> &'static str {
        match self {
            ErrorCategory::Config => "CONFIG_ERROR",
            ErrorCategory::Validation => "VALIDATION_ERROR",
            ErrorCategory::State => "STATE_ERROR",
            ErrorCategory::NotFound => "NOT_FOUND",
            ErrorCategory::Io => "IO_ERROR",
            ErrorCategory::Parse => "PARSE_ERROR",
            ErrorCategory::Process => "PROCESS_ERROR",
            ErrorCategory::Quality => "QUALITY_ERROR",
        }
    }
}

impl std::fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

// Convenience conversions
impl From<std::io::Error> for GlowError {
    fn from(err: std::io::Error) -> Self {
        GlowError::IoError {
            message: err.to_string(),
            source: err,
        }
    }
}

impl From<serde_yaml::Error> for GlowError {
    fn from(err: serde_yaml::Error) -> Self {
        GlowError::YamlParseError {
            message: err.to_string(),
            source: err,
        }
    }
}

impl From<serde_json::Error> for GlowError {
    fn from(err: serde_json::Error) -> Self {
        GlowError::JsonParseError {
            message: err.to_string(),
            source: err,
        }
    }
}
