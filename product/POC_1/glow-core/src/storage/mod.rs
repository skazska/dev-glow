//! Storage layer
//!
//! Handles file system operations for step data files.

mod frontmatter;
mod step_files;

pub use frontmatter::{parse_frontmatter, render_frontmatter};
pub use step_files::StepFileManager;

use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::error::Result;
use crate::model::{Process, Step};

/// Main storage interface
pub struct Storage {
    /// Project root path
    project_root: PathBuf,
    /// Data directory path
    data_dir: PathBuf,
    /// Config directory path
    config_dir: PathBuf,
    /// Step file manager
    step_files: StepFileManager,
}

impl Storage {
    /// Create a new storage instance
    pub fn new(project_root: PathBuf, config: &Config, config_dir: PathBuf) -> Self {
        let data_dir = config.data_dir(&project_root);
        let step_files = StepFileManager::new(data_dir.clone());

        Self {
            project_root,
            data_dir,
            config_dir,
            step_files,
        }
    }

    /// Get the project root path
    pub fn project_root(&self) -> &Path {
        &self.project_root
    }

    /// Get the data directory path
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    /// Get the config directory path
    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }

    /// Initialize the data directory structure
    pub fn init_data_dir(&self) -> Result<()> {
        std::fs::create_dir_all(&self.data_dir)?;
        
        // Create description.md placeholder
        let description_path = self.data_dir.join("description.md");
        if !description_path.exists() {
            std::fs::write(&description_path, "# Project Description\n\nTODO: Add project description.\n")?;
        }

        // Create summary.md placeholder
        let summary_path = self.data_dir.join("summary.md");
        if !summary_path.exists() {
            std::fs::write(&summary_path, "# Process Summary\n\nNo iterations yet.\n")?;
        }

        Ok(())
    }

    /// Get step file manager
    pub fn step_files(&self) -> &StepFileManager {
        &self.step_files
    }

    /// Get mutable step file manager
    pub fn step_files_mut(&mut self) -> &mut StepFileManager {
        &mut self.step_files
    }

    /// Read a step from its data file
    pub fn read_step(&self, fqid: &str) -> Result<Step> {
        self.step_files.read_step(fqid)
    }

    /// Write a step to its data file
    pub fn write_step(&self, step: &Step) -> Result<()> {
        self.step_files.write_step(step)
    }

    /// Create iteration folder
    pub fn create_iteration_folder(&self, process_fqid: &str, iteration: u32) -> Result<PathBuf> {
        let iteration_folder = format!("iteration_{:06}", iteration);
        let path = if process_fqid == "ROOT" {
            self.data_dir.join(&iteration_folder)
        } else {
            let step_folder = self.step_files.step_folder_path(process_fqid);
            step_folder.join(&iteration_folder)
        };

        std::fs::create_dir_all(&path)?;

        // Create iteration summary
        let summary_path = path.join("summary.md");
        if !summary_path.exists() {
            std::fs::write(
                &summary_path,
                format!("# Iteration {} Summary\n\nIn progress.\n", iteration),
            )?;
        }

        Ok(path)
    }

    /// Get iteration folder path
    pub fn iteration_folder_path(&self, process_fqid: &str, iteration: u32) -> PathBuf {
        let iteration_folder = format!("iteration_{:06}", iteration);
        if process_fqid == "ROOT" {
            self.data_dir.join(&iteration_folder)
        } else {
            let step_folder = self.step_files.step_folder_path(process_fqid);
            step_folder.join(&iteration_folder)
        }
    }

    /// Load process state from files
    pub fn load_process(&self, _fqid: &str) -> Result<Option<Process>> {
        // This will be implemented with full process loading logic
        todo!("Implement process loading")
    }

    /// Save process state to files
    pub fn save_process(&self, process: &Process) -> Result<()> {
        // Save main step file
        self.write_step(&process.step)?;

        // Save iteration data
        if let Some(iteration) = process.current_iteration_data() {
            for step in &iteration.steps {
                self.write_step(step)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_storage_init() {
        let temp = tempdir().unwrap();
        let config = Config::default();
        let config_dir = temp.path().join(".glow");

        let storage = Storage::new(temp.path().to_path_buf(), &config, config_dir);
        storage.init_data_dir().unwrap();

        assert!(storage.data_dir().exists());
        assert!(storage.data_dir().join("description.md").exists());
        assert!(storage.data_dir().join("summary.md").exists());
    }
}
