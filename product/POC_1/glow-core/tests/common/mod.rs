//! Common test utilities

use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// A test project with temporary directory
pub struct TestProject {
    _temp_dir: TempDir,
    path: PathBuf,
}

impl TestProject {
    /// Create a new test project
    pub fn new(name: &str) -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let path = temp_dir.path().to_path_buf();

        // Create basic directory structure
        fs::create_dir_all(path.join(".glow/schemas")).expect("Failed to create .glow dir");
        fs::create_dir_all(path.join("glow")).expect("Failed to create glow dir");

        Self {
            _temp_dir: temp_dir,
            path,
        }
    }

    /// Create a test project with TaskTrack-like configuration
    pub fn with_tasktrack_config() -> Self {
        let project = Self::new("tasktrack");

        // Write config.yaml
        let config = r#"
version: "1.0"
project_id: "TASK-TRACK"
project_name: "TaskTrack"
process: "agile-feature"
glow_dir: "glow"
"#;
        fs::write(project.path.join(".glow/config.yaml"), config)
            .expect("Failed to write config");

        // Write process_config.yaml
        let process_config = r#"
version: "1.0"
process:
  id: "agile-feature"
  name: "Agile Feature Development"
  description: "A structured process for developing features"

steps:
  - id: "ROOT"
    classification: "Process"
    purpose: "Main project process"
    own_steps:
      - id: "FEAT"
        repeatable: true
        classification: "Process"
        purpose: "Feature development iteration"
        scope:
          - id: "FEATURE_ID"
            data_type: "String"
            required: true
          - id: "FEATURE_NAME"
            data_type: "String"
            required: true
        own_steps:
          - id: "REQ"
            repeatable: true
            classification: "Task"
            purpose: "Requirements gathering"
            scope:
              - id: "REQ_ID"
                data_type: "String"
            output:
              - id: "REQUIREMENTS"
                data_type: "String"
          - id: "DESIGN"
            classification: "Task"
            purpose: "Design the solution"
            link:
              - source: "REQ"
                type: "waits_for"
            output:
              - id: "DESIGN_DOC"
                data_type: "String"
          - id: "IMPL"
            classification: "Task"
            purpose: "Implement the feature"
            link:
              - source: "DESIGN"
                type: "waits_for"
            output:
              - id: "CODE_LOCATION"
                data_type: "String"
          - id: "TEST"
            classification: "Task"
            purpose: "Test the implementation"
            link:
              - source: "IMPL"
                type: "waits_for"
            output:
              - id: "TEST_RESULTS"
                data_type: "String"
"#;
        fs::write(project.path.join(".glow/process_config.yaml"), process_config)
            .expect("Failed to write process config");

        // Copy schemas
        let schemas_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("schemas");
        if schemas_dir.exists() {
            for entry in fs::read_dir(&schemas_dir).expect("Failed to read schemas dir") {
                let entry = entry.expect("Failed to read entry");
                let dest = project.path.join(".glow/schemas").join(entry.file_name());
                fs::copy(entry.path(), dest).expect("Failed to copy schema");
            }
        }

        project
    }

    /// Get the project path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Create a file in the project
    pub fn write_file(&self, relative_path: &str, content: &str) {
        let path = self.path.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent dirs");
        }
        fs::write(path, content).expect("Failed to write file");
    }

    /// Read a file from the project
    pub fn read_file(&self, relative_path: &str) -> String {
        fs::read_to_string(self.path.join(relative_path))
            .expect("Failed to read file")
    }

    /// Check if a file exists
    pub fn file_exists(&self, relative_path: &str) -> bool {
        self.path.join(relative_path).exists()
    }
}
