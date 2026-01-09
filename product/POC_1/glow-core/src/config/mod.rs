//! Configuration module
//!
//! Handles loading and validation of configuration files.

mod loader;
mod schema;

pub use loader::{Config, ConfigLoader, ProcessConfig};
pub use schema::validate_config;

use std::path::{Path, PathBuf};

use crate::error::{GlowError, Result};
use crate::{CONFIG_DIR_ENV, DEFAULT_CONFIG_DIR};

/// Default configuration file names
pub const CONFIG_FILE: &str = "config.yaml";
pub const PROCESS_CONFIG_FILE: &str = "process_config.yaml";
pub const TEMPLATES_FOLDER: &str = "templates";
pub const SCHEMAS_FOLDER: &str = "schemas";
pub const DEFAULT_TEMPLATE: &str = "any-step.md";

/// Find the configuration directory
pub fn find_config_dir(project_root: &Path, override_path: Option<&Path>) -> Result<PathBuf> {
    // 1. Check override path
    if let Some(path) = override_path {
        if path.exists() && path.is_dir() {
            return Ok(path.to_path_buf());
        }
        return Err(GlowError::ConfigNotFound {
            path: path.to_path_buf(),
        });
    }

    // 2. Check environment variable
    if let Ok(env_path) = std::env::var(CONFIG_DIR_ENV) {
        let path = PathBuf::from(env_path);
        if path.exists() && path.is_dir() {
            return Ok(path);
        }
    }

    // 3. Check default location
    let default_path = project_root.join(DEFAULT_CONFIG_DIR);
    if default_path.exists() && default_path.is_dir() {
        return Ok(default_path);
    }

    Err(GlowError::ConfigNotFound {
        path: default_path,
    })
}

/// Get the default config directory path (may not exist)
pub fn default_config_dir(project_root: &Path) -> PathBuf {
    project_root.join(DEFAULT_CONFIG_DIR)
}

/// Get the schemas directory
pub fn schemas_dir(config_dir: &Path) -> PathBuf {
    config_dir.join(SCHEMAS_FOLDER)
}

/// Get the templates directory
pub fn templates_dir(config_dir: &Path) -> PathBuf {
    config_dir.join(TEMPLATES_FOLDER)
}
