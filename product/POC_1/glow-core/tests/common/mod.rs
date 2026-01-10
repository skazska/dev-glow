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
    /// Create a new empty test project (for init_project tests)
    pub fn empty() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let path = temp_dir.path().to_path_buf();

        Self {
            _temp_dir: temp_dir,
            path,
        }
    }

    /// Create a new test project with basic structure
    pub fn new(_name: &str) -> Self {
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

        // Write config.yaml (updated to match current schema)
        let config = r#"
version: "0.1.0"
project_name: "TaskTrack"
data_folder: "glow"
process_config: "process_config.yaml"
templates_folder: "templates/"
default_template: "any-step.md"
"#;
        fs::write(project.path.join(".glow/config.yaml"), config)
            .expect("Failed to write config");

        // Write process_config.yaml (updated to match current schema)
        let process_config = r#"
version: "0.1.0"

classifications:
  - id: type
    name: "Step Type"
    values:
      - key: Feature
        name: "Feature"
      - key: Task
        name: "Task"
      - key: Process
        name: "Process"

parameter_types:
  - id: FEATURE_ID
    purpose: "Feature identifier"
    data_type: STR
    is_required: true

  - id: FEATURE_NAME
    purpose: "Feature name"
    data_type: STR
    is_required: true

  - id: REQUIREMENTS
    purpose: "Requirements document"
    data_type: CONTENT

  - id: DESIGN_DOC
    purpose: "Design document"
    data_type: CONTENT

  - id: CODE_LOCATION
    purpose: "Code location"
    data_type: STR

  - id: TEST_RESULTS
    purpose: "Test results"
    data_type: STR

root_process:
  id: ROOT
  purpose: "TaskTrack Development Process"
  expectations: "All features implemented"
  steps:
    - id: FEAT
      purpose: "Feature Development"
      classification: "Feature"
      allow_iterations: true
      scope:
        - id: FEATURE_ID
          type_ref: FEATURE_ID
          is_required: true
        - id: FEATURE_NAME
          type_ref: FEATURE_NAME
          is_required: true
      steps:
        - id: REQ
          purpose: "Requirements Gathering"
          classification: "Task"
          allow_iterations: true
          outputs:
            - id: REQUIREMENTS
              type_ref: REQUIREMENTS
        - id: DESIGN
          purpose: "Solution Design"
          classification: "Task"
          outputs:
            - id: DESIGN_DOC
              type_ref: DESIGN_DOC
        - id: IMPL
          purpose: "Implementation"
          classification: "Task"
          outputs:
            - id: CODE_LOCATION
              type_ref: CODE_LOCATION
        - id: TEST
          purpose: "Testing"
          classification: "Task"
          outputs:
            - id: TEST_RESULTS
              type_ref: TEST_RESULTS
      links:
        - type: dependency
          from: DESIGN
          to: REQ
        - type: dependency
          from: IMPL
          to: DESIGN
        - type: dependency
          from: TEST
          to: IMPL
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
