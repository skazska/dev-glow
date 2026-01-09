//! Step file management
//!
//! Handles reading and writing step data files.

use std::path::{Path, PathBuf};

use crate::error::{GlowError, Result};
use crate::model::Step;

use super::frontmatter::{parse_step_frontmatter, render_step_frontmatter};

/// Manages step data files
pub struct StepFileManager {
    /// Data directory root
    data_dir: PathBuf,
}

impl StepFileManager {
    /// Create a new step file manager
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }

    /// Get the data directory
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    /// Convert FQID to file path
    pub fn step_file_path(&self, fqid: &str) -> PathBuf {
        let parts: Vec<&str> = fqid.split('.').collect();
        let mut path = self.data_dir.clone();

        // Build path from FQID parts
        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                // Last part is the file name
                path.push(format!("{}.md", part));
            } else {
                // Intermediate parts are folders
                path.push(part);
            }
        }

        path
    }

    /// Get the folder path for a step (for artifacts, sub-steps)
    pub fn step_folder_path(&self, fqid: &str) -> PathBuf {
        let file_path = self.step_file_path(fqid);
        file_path.with_extension("")
    }

    /// Get the artifacts folder path for a step
    pub fn artifacts_folder_path(&self, fqid: &str) -> PathBuf {
        self.step_folder_path(fqid).join("artifacts")
    }

    /// Get the description file path for a step
    pub fn description_file_path(&self, fqid: &str) -> PathBuf {
        self.step_folder_path(fqid).join("description.md")
    }

    /// Read a step from its data file
    pub fn read_step(&self, fqid: &str) -> Result<Step> {
        let path = self.step_file_path(fqid);
        self.read_step_from_path(&path, fqid)
    }

    /// Read a step from a specific path
    pub fn read_step_from_path(&self, path: &Path, fqid: &str) -> Result<Step> {
        if !path.exists() {
            return Err(GlowError::StepNotFound {
                fqid: fqid.to_string(),
            });
        }

        let content = std::fs::read_to_string(path).map_err(|e| GlowError::FileReadError {
            path: path.to_path_buf(),
            source: e,
        })?;

        let (step, _body) = parse_step_frontmatter(&content)?;
        Ok(step)
    }

    /// Write a step to its data file
    pub fn write_step(&self, step: &Step) -> Result<()> {
        let path = self.step_file_path(step.fqid());
        self.write_step_to_path(step, &path)
    }

    /// Write a step to a specific path
    pub fn write_step_to_path(&self, step: &Step, path: &Path) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| GlowError::FileWriteError {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }

        // Read existing content if file exists
        let existing_content = if path.exists() {
            let content = std::fs::read_to_string(path).ok();
            content.and_then(|c| {
                super::frontmatter::parse_frontmatter(&c)
                    .ok()
                    .map(|(_, body)| body)
            })
        } else {
            None
        };

        let body = existing_content.unwrap_or_else(|| {
            // Default content template
            format!(
                "# {}\n\n{}\n",
                step.attr.id,
                step.attr.purpose.as_deref().unwrap_or("TODO: Add description")
            )
        });

        let content = render_step_frontmatter(step, &body)?;

        std::fs::write(path, content).map_err(|e| GlowError::FileWriteError {
            path: path.to_path_buf(),
            source: e,
        })
    }

    /// Create step folder structure
    pub fn create_step_folder(&self, fqid: &str) -> Result<()> {
        let folder_path = self.step_folder_path(fqid);
        std::fs::create_dir_all(&folder_path)?;

        // Create artifacts folder
        let artifacts_path = folder_path.join("artifacts");
        std::fs::create_dir_all(&artifacts_path)?;

        Ok(())
    }

    /// Check if a step file exists
    pub fn step_exists(&self, fqid: &str) -> bool {
        self.step_file_path(fqid).exists()
    }

    /// List all step files in a directory
    pub fn list_steps_in_dir(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        if !dir.exists() {
            return Ok(Vec::new());
        }

        let mut steps = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |e| e == "md") {
                // Skip special files
                let name = path.file_stem().and_then(|n| n.to_str()).unwrap_or("");
                if !matches!(name, "description" | "summary") {
                    steps.push(path);
                }
            }
        }

        // Sort by file name for consistent ordering
        steps.sort();
        Ok(steps)
    }

    /// Delete a step file
    pub fn delete_step(&self, fqid: &str) -> Result<()> {
        let path = self.step_file_path(fqid);
        if path.exists() {
            std::fs::remove_file(&path)?;
        }

        // Also remove folder if empty
        let folder_path = self.step_folder_path(fqid);
        if folder_path.exists() {
            if let Ok(entries) = std::fs::read_dir(&folder_path) {
                if entries.count() == 0 {
                    std::fs::remove_dir(&folder_path)?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{StepAttributes, StepStatus};
    use tempfile::tempdir;

    fn create_test_step(id: &str, fqid: &str) -> Step {
        Step {
            attr: StepAttributes {
                id: id.to_string(),
                fqid: Some(fqid.to_string()),
                classification: None,
                purpose: Some("Test step".to_string()),
                expectations: None,
                status: StepStatus::Wait,
            },
            input: Vec::new(),
            scope: Vec::new(),
            output: Vec::new(),
            parent: Vec::new(),
            own_steps: Vec::new(),
            links: Vec::new(),
        }
    }

    #[test]
    fn test_step_file_path() {
        let temp = tempdir().unwrap();
        let manager = StepFileManager::new(temp.path().to_path_buf());

        assert_eq!(
            manager.step_file_path("FEAT-001"),
            temp.path().join("FEAT-001.md")
        );

        assert_eq!(
            manager.step_file_path("FEAT-001.REQ-001"),
            temp.path().join("FEAT-001").join("REQ-001.md")
        );

        assert_eq!(
            manager.step_file_path("FEAT-001.REQ-001.TASK-001"),
            temp.path()
                .join("FEAT-001")
                .join("REQ-001")
                .join("TASK-001.md")
        );
    }

    #[test]
    fn test_write_read_step() {
        let temp = tempdir().unwrap();
        let manager = StepFileManager::new(temp.path().to_path_buf());

        let step = create_test_step("FEAT-001", "FEAT-001");
        manager.write_step(&step).unwrap();

        assert!(manager.step_exists("FEAT-001"));

        let loaded = manager.read_step("FEAT-001").unwrap();
        assert_eq!(loaded.attr.id, "FEAT-001");
        assert_eq!(loaded.attr.status, StepStatus::Wait);
    }

    #[test]
    fn test_nested_step() {
        let temp = tempdir().unwrap();
        let manager = StepFileManager::new(temp.path().to_path_buf());

        // Create parent folder first
        std::fs::create_dir_all(temp.path().join("FEAT-001")).unwrap();

        let step = create_test_step("REQ-001", "FEAT-001.REQ-001");
        manager.write_step(&step).unwrap();

        let loaded = manager.read_step("FEAT-001.REQ-001").unwrap();
        assert_eq!(loaded.attr.id, "REQ-001");
    }
}
