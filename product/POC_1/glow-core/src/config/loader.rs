//! Configuration loading
//!
//! Loads and parses YAML configuration files.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{GlowError, Result};
use crate::model::{ClassificationDictionary, LinkType, ParameterType, StepDefinition};
use crate::DEFAULT_DATA_DIR;

/// Project configuration (.glow/config.yaml)
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct Config {
    /// Schema version for migration support
    #[serde(default = "default_version")]
    pub version: String,
    /// Human-readable project name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    /// Folder name for process data storage
    #[serde(default = "default_data_folder")]
    pub data_folder: String,
    /// Process configuration file name
    #[serde(default = "default_process_config")]
    pub process_config: String,
    /// Folder name for step templates
    #[serde(default = "default_templates_folder")]
    pub templates_folder: String,
    /// Default template file name for steps
    #[serde(default = "default_template")]
    pub default_template: String,
}

fn default_version() -> String {
    "0.1.0".to_string()
}

fn default_data_folder() -> String {
    DEFAULT_DATA_DIR.to_string()
}

fn default_process_config() -> String {
    "process_config.yaml".to_string()
}

fn default_templates_folder() -> String {
    "templates/".to_string()
}

fn default_template() -> String {
    "any-step.md".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: default_version(),
            project_name: None,
            data_folder: default_data_folder(),
            process_config: default_process_config(),
            templates_folder: default_templates_folder(),
            default_template: default_template(),
        }
    }
}

impl Config {
    /// Create a new config with project name
    pub fn new(project_name: impl Into<String>) -> Self {
        Self {
            project_name: Some(project_name.into()),
            ..Default::default()
        }
    }

    /// Get the data folder path relative to project root
    pub fn data_dir(&self, project_root: &Path) -> PathBuf {
        project_root.join(&self.data_folder)
    }
}

/// Process configuration (.glow/process_config.yaml)
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ProcessConfig {
    /// Schema version for migration support
    #[serde(default = "default_version")]
    pub version: String,
    /// Classification dimensions
    #[serde(default)]
    pub classifications: Vec<ClassificationDictionary>,
    /// Parameter type definitions
    #[serde(default)]
    pub parameter_types: Vec<ParameterType>,
    /// Custom link type definitions
    #[serde(default)]
    pub link_types: Vec<LinkType>,
    /// Root process step definition
    pub root_process: StepDefinition,
}

impl Default for ProcessConfig {
    fn default() -> Self {
        Self {
            version: default_version(),
            classifications: Vec::new(),
            parameter_types: Vec::new(),
            link_types: Vec::new(),
            root_process: StepDefinition::new_root(),
        }
    }
}

impl ProcessConfig {
    /// Find a classification dictionary by ID
    pub fn find_classification(&self, id: &str) -> Option<&ClassificationDictionary> {
        self.classifications.iter().find(|c| c.id == id)
    }

    /// Find a parameter type by ID
    pub fn find_parameter_type(&self, id: &str) -> Option<&ParameterType> {
        self.parameter_types.iter().find(|p| p.id == id)
    }

    /// Find a link type by ID
    pub fn find_link_type(&self, id: &str) -> Option<&LinkType> {
        self.link_types.iter().find(|l| l.id == id)
    }

    /// Find a step definition by FQID
    pub fn find_step_definition(&self, fqid: &str) -> Option<&StepDefinition> {
        if fqid == "ROOT" {
            return Some(&self.root_process);
        }

        let parts: Vec<&str> = fqid.split('.').collect();
        let mut current = &self.root_process;

        for part in parts {
            match current.find_step(part) {
                Some(step) => current = step,
                None => return None,
            }
        }

        Some(current)
    }
}

/// Configuration loader
pub struct ConfigLoader {
    /// Config directory path
    config_dir: PathBuf,
    /// Project root path
    project_root: PathBuf,
}

impl ConfigLoader {
    /// Create a new config loader
    pub fn new(config_dir: PathBuf, project_root: PathBuf) -> Self {
        Self {
            config_dir,
            project_root,
        }
    }

    /// Load the main config file
    pub fn load_config(&self) -> Result<Config> {
        let config_path = self.config_dir.join(super::CONFIG_FILE);
        self.load_yaml(&config_path)
    }

    /// Load the process config file
    pub fn load_process_config(&self, config: &Config) -> Result<ProcessConfig> {
        let process_config_path = self.config_dir.join(&config.process_config);
        self.load_yaml(&process_config_path)
    }

    /// Load a YAML file
    fn load_yaml<T: serde::de::DeserializeOwned>(&self, path: &Path) -> Result<T> {
        let content = std::fs::read_to_string(path).map_err(|e| GlowError::FileReadError {
            path: path.to_path_buf(),
            source: e,
        })?;

        serde_yaml::from_str(&content).map_err(|e| GlowError::YamlParseError {
            message: format!("Failed to parse {}: {}", path.display(), e),
            source: e,
        })
    }

    /// Save a config file
    pub fn save_config(&self, config: &Config) -> Result<()> {
        let config_path = self.config_dir.join(super::CONFIG_FILE);
        self.save_yaml(&config_path, config)
    }

    /// Save the process config file
    pub fn save_process_config(&self, config: &Config, process_config: &ProcessConfig) -> Result<()> {
        let process_config_path = self.config_dir.join(&config.process_config);
        self.save_yaml(&process_config_path, process_config)
    }

    /// Save a YAML file
    fn save_yaml<T: serde::Serialize>(&self, path: &Path, value: &T) -> Result<()> {
        let content = serde_yaml::to_string(value)?;

        // Add schema reference comment
        let schema_comment = self.get_schema_comment(path);
        let content = format!("{}{}", schema_comment, content);

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| GlowError::FileWriteError {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }

        std::fs::write(path, content).map_err(|e| GlowError::FileWriteError {
            path: path.to_path_buf(),
            source: e,
        })
    }

    /// Get schema comment for a config file
    fn get_schema_comment(&self, path: &Path) -> String {
        let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        let schema = match filename {
            "config.yaml" => "config.schema.json",
            "process_config.yaml" => "process_config.schema.json",
            "mcp_config.yaml" => "mcp_config.schema.json",
            _ => return String::new(),
        };

        format!("# yaml-language-server: $schema=./schemas/{}\n", schema)
    }

    /// Get the project root path
    pub fn project_root(&self) -> &Path {
        &self.project_root
    }

    /// Get the config directory path
    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.version, "0.1.0");
        assert_eq!(config.data_folder, "glow");
    }

    #[test]
    fn test_process_config_default() {
        let config = ProcessConfig::default();
        assert_eq!(config.root_process.id, "ROOT");
        assert!(config.classifications.is_empty());
    }

    #[test]
    fn test_config_loader_save_load() -> Result<()> {
        let temp = tempdir().unwrap();
        let config_dir = temp.path().join(".glow");
        std::fs::create_dir_all(&config_dir).unwrap();

        let loader = ConfigLoader::new(config_dir.clone(), temp.path().to_path_buf());

        let config = Config::new("Test Project");
        loader.save_config(&config)?;

        let loaded = loader.load_config()?;
        assert_eq!(loaded.project_name, Some("Test Project".to_string()));

        Ok(())
    }

    #[test]
    fn test_find_step_definition() {
        let mut process_config = ProcessConfig::default();
        process_config.root_process.steps.push(StepDefinition {
            id: "FEAT-001".to_string(),
            purpose: None,
            classification: None,
            expectations: None,
            template: None,
            inputs: Vec::new(),
            outputs: Vec::new(),
            scope: Vec::new(),
            steps: vec![StepDefinition {
                id: "REQ-001".to_string(),
                purpose: None,
                classification: None,
                expectations: None,
                template: None,
                inputs: Vec::new(),
                outputs: Vec::new(),
                scope: Vec::new(),
                steps: Vec::new(),
                links: Vec::new(),
                allow_iterations: true,
            }],
            links: Vec::new(),
            allow_iterations: true,
        });

        assert!(process_config.find_step_definition("ROOT").is_some());
        assert!(process_config.find_step_definition("FEAT-001").is_some());
        assert!(process_config.find_step_definition("FEAT-001.REQ-001").is_some());
        assert!(process_config.find_step_definition("UNKNOWN").is_none());
    }
}
