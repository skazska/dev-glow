//! Integration tests for glow-core
//!
//! These tests verify the full workflow from project initialization
//! through step completion, using the TaskTrack validation scenario.

mod common;

use common::TestProject;
use glow_core::engine::operations::ProcessEngine;
use glow_core::model::StepStatus;

#[test]
fn test_project_initialization() {
    let project = TestProject::empty();

    // Initialize project
    let _engine = ProcessEngine::init_project(project.path().to_path_buf(), Some("TaskTrack".to_string()))
        .expect("Failed to initialize project");

    // Verify configuration was created
    assert!(project.path().join(".glow/config.yaml").exists());
    assert!(project.path().join(".glow/process_config.yaml").exists());
    assert!(project.path().join("glow").exists());
}

#[test]
fn test_root_step_initialization() {
    let project = TestProject::with_tasktrack_config();

    let mut engine = ProcessEngine::new(project.path().to_path_buf())
        .expect("Failed to load project");

    // Initialize ROOT step
    let root = engine.init_step("ROOT", vec![], false)
        .expect("Failed to init ROOT");

    assert_eq!(root.attr.id, "ROOT");
    assert_eq!(root.status(), StepStatus::Todo);

    // Verify sub-steps were created
    assert!(!root.own_steps.is_empty());

    // Verify file was created (the step file is ROOT.md, not ROOT.step.md)
    let root_file = project.path().join("glow/ROOT.md");
    assert!(root_file.exists());
}

#[test]
fn test_step_status_transitions() {
    let project = TestProject::with_tasktrack_config();

    let mut engine = ProcessEngine::new(project.path().to_path_buf())
        .expect("Failed to load project");

    // Initialize ROOT
    engine.init_step("ROOT", vec![], false)
        .expect("Failed to init ROOT");

    // Start ROOT
    let root = engine.start_step("ROOT")
        .expect("Failed to start ROOT");
    assert_eq!(root.status(), StepStatus::InProgress);

    // Initialize the FEAT step (using the definition ID, not an instance ID)
    use glow_core::model::ParameterValue;
    let feat = engine.init_step(
        "FEAT",
        vec![
            ParameterValue::new("FEATURE_ID", serde_json::json!("001")),
            ParameterValue::new("FEATURE_NAME", serde_json::json!("User Management")),
        ],
        false,
    ).expect("Failed to init FEAT");

    assert_eq!(feat.status(), StepStatus::Todo);

    // Start FEAT
    engine.start_step("FEAT")
        .expect("Failed to start FEAT");

    // Initialize a child step (REQ)
    engine.init_step("FEAT.REQ", vec![], false)
        .expect("Failed to init FEAT.REQ");

    engine.start_step("FEAT.REQ")
        .expect("Failed to start FEAT.REQ");

    let completed = engine.finish_step("FEAT.REQ", vec![], Some("Requirements gathered".to_string()))
        .expect("Failed to finish FEAT.REQ");

    assert_eq!(completed.status(), StepStatus::Done);
}

#[test]
fn test_status_tree() {
    let project = TestProject::with_tasktrack_config();

    let mut engine = ProcessEngine::new(project.path().to_path_buf())
        .expect("Failed to load project");

    // Initialize and start ROOT
    engine.init_step("ROOT", vec![], false)
        .expect("Failed to init ROOT");
    engine.start_step("ROOT")
        .expect("Failed to start ROOT");

    // Get status tree
    let tree = engine.get_status_tree()
        .expect("Failed to get status tree");

    assert_eq!(tree.id, "ROOT");
    assert_eq!(tree.status, StepStatus::InProgress);
}

#[test]
fn test_progress_tracking() {
    let project = TestProject::with_tasktrack_config();

    let mut engine = ProcessEngine::new(project.path().to_path_buf())
        .expect("Failed to load project");

    // Initialize ROOT
    engine.init_step("ROOT", vec![], false)
        .expect("Failed to init ROOT");

    let progress = engine.get_progress()
        .expect("Failed to get progress");

    assert!(progress.total > 0);
    assert_eq!(progress.done, 0);
    assert!(progress.todo > 0 || progress.wait > 0);
}

#[test]
fn test_next_actions() {
    let project = TestProject::with_tasktrack_config();

    let mut engine = ProcessEngine::new(project.path().to_path_buf())
        .expect("Failed to load project");

    // Initialize ROOT
    engine.init_step("ROOT", vec![], false)
        .expect("Failed to init ROOT");

    let actions = engine.get_next_actions()
        .expect("Failed to get next actions");

    assert!(!actions.is_empty());
    // First action should be to start ROOT
    assert_eq!(actions[0].fqid, "ROOT");
}

#[test]
fn test_validation() {
    let project = TestProject::with_tasktrack_config();

    let mut engine = ProcessEngine::new(project.path().to_path_buf())
        .expect("Failed to load project");

    // Initialize ROOT
    engine.init_step("ROOT", vec![], false)
        .expect("Failed to init ROOT");

    let report = engine.validate(None)
        .expect("Failed to validate");

    // Initial state may have low completeness
    assert!(report.completeness >= 0.0);
}
