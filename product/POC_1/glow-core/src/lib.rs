//! glow-core: Core library for dev-glow development process tooling
//!
//! This library provides the business logic, data access layer, and process engine
//! for the dev-glow development process management tool.
//!
//! # Architecture
//!
//! - `config`: Configuration loading and validation
//! - `model`: Data model structs (Step, Process, Parameter, Link)
//! - `engine`: Process engine (state machine, context management)
//! - `storage`: File system operations
//! - `template`: Templating engine wrapper
//! - `quality`: Context quality assessment

pub mod config;
pub mod engine;
pub mod error;
pub mod model;
pub mod quality;
pub mod storage;
pub mod template;

pub use config::{Config, ProcessConfig};
pub use engine::ProcessEngine;
pub use error::{GlowError, Result};
pub use model::{Link, Parameter, Process, Step, StepStatus};
pub use storage::Storage;
pub use template::TemplateEngine;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default configuration folder name
pub const DEFAULT_CONFIG_DIR: &str = ".glow";

/// Default data folder name
pub const DEFAULT_DATA_DIR: &str = "glow";

/// Environment variable for custom config directory
pub const CONFIG_DIR_ENV: &str = "DEV_GLOW_CONFIG_DIR";
