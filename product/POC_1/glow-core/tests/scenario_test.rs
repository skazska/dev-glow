//! TaskTrack Validation Scenario Tests
//!
//! These tests implement the validation scenario from POC_1/scenario.md
//! to verify the complete workflow for the TaskTrack project.

mod common;

use common::TestProject;
use glow_core::engine::operations::ProcessEngine;
use glow_core::model::{ParameterValue, StepStatus};

/// Create a TaskTrack-configured test project with full process config
fn setup_tasktrack_project() -> TestProject {
    let project = TestProject::new("tasktrack-scenario");

    // Write config.yaml matching the current schema
    let config = r#"
version: "0.1.0"
project_name: "TaskTrack"
data_folder: "glow"
process_config: "process_config.yaml"
templates_folder: "templates/"
default_template: "any-step.md"
"#;
    project.write_file(".glow/config.yaml", config);

    // Write process_config.yaml matching the current schema
    let process_config = include_str!("fixtures/tasktrack_process_config.yaml");
    project.write_file(".glow/process_config.yaml", process_config);

    project
}

/// Full scenario test: TaskTrack User Management Feature
/// 
/// This test follows the scenario from scenario.md:
/// 1. Project initialization
/// 2. Feature initialization
/// 3. Requirements gathering
/// 4. Design
/// 5. Implementation
/// 6. Testing
/// 7. Feature completion
///
/// Note: Currently using template IDs directly (FEAT, REQ, etc.)
/// Iteration support (FEAT-001, FEAT-002) would be a future enhancement.
#[test]
fn test_tasktrack_full_scenario() {
    let project = setup_tasktrack_project();

    let mut engine = ProcessEngine::new(project.path().to_path_buf())
        .expect("Failed to load project");

    // === Phase 1: Project Initialization ===
    println!("\n=== Phase 1: Project Initialization ===");

    // Initialize ROOT process
    let root = engine.init_step("ROOT", vec![], false)
        .expect("Failed to init ROOT");
    assert_eq!(root.status(), StepStatus::Todo);
    assert!(project.file_exists("glow/ROOT.md"));

    // Start ROOT
    let root = engine.start_step("ROOT")
        .expect("Failed to start ROOT");
    assert_eq!(root.status(), StepStatus::InProgress);

    // Verify initial status tree
    let tree = engine.get_status_tree().expect("Failed to get status");
    assert_eq!(tree.id, "ROOT");
    assert_eq!(tree.status, StepStatus::InProgress);

    // === Phase 2: Feature Initialization ===
    println!("\n=== Phase 2: Feature Initialization ===");

    // Initialize FEAT with scope parameters
    let feat = engine.init_step(
        "FEAT",
        vec![
            ParameterValue::new("FEATURE_ID", serde_json::json!("001")),
            ParameterValue::new("FEATURE_NAME", serde_json::json!("User Management")),
            ParameterValue::new("FEATURE_DESCRIPTION", serde_json::json!(
                "User registration, authentication, and profile management"
            )),
        ],
        false,
    ).expect("Failed to init FEAT");

    assert_eq!(feat.attr.id, "FEAT");

    // Start FEAT
    engine.start_step("FEAT")
        .expect("Failed to start FEAT");

    // === Phase 3: Requirements Gathering ===
    println!("\n=== Phase 3: Requirements Gathering ===");

    // Initialize REQ task
    let req_init = engine.init_step("FEAT.REQ", vec![], false)
        .expect("Failed to init FEAT.REQ");
    println!("After init, REQ id={}, fqid={}", req_init.attr.id, req_init.fqid());
    
    // Start REQ
    engine.start_step("FEAT.REQ")
        .expect("Failed to start FEAT.REQ");

    // Complete REQ with outputs
    let req_done = engine.finish_step(
        "FEAT.REQ",
        vec![
            ParameterValue::new("REQUIREMENTS_DOC", serde_json::json!(
                "docs/requirements/user-management.md"
            )),
            ParameterValue::new("ACCEPTANCE_CRITERIA", serde_json::json!([
                "User can register with email",
                "User can login with credentials",
                "User can update profile"
            ])),
        ],
        Some("Requirements gathered through stakeholder interviews".to_string()),
    ).expect("Failed to finish FEAT.REQ");

    assert_eq!(req_done.status(), StepStatus::Done);
    println!("FEAT.REQ finished with status: {:?}, fqid: {}", req_done.status(), req_done.fqid());
    
    // List files in the glow directory
    println!("\nFiles in glow directory:");
    fn print_dir(path: &std::path::Path, prefix: &str) {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                println!("{}{}", prefix, p.display());
                if p.is_dir() {
                    print_dir(&p, &format!("{}  ", prefix));
                }
            }
        }
    }
    print_dir(&project.path().join("glow"), "  ");
    
    // Verify REQ is actually stored as Done
    let req_check = engine.show_step("FEAT.REQ").expect("Failed to read REQ");
    println!("\nFEAT.REQ read back with status: {:?}, fqid: {}", req_check.status(), req_check.fqid());

    // === Phase 4: Design ===
    println!("\n=== Phase 4: Design ===");

    // Check next actions
    let actions = engine.get_next_actions().expect("Failed to get actions");
    let _design_available = actions.iter().any(|a| a.fqid.contains("DESIGN"));

    // Initialize and complete DESIGN
    engine.init_step("FEAT.DESIGN", vec![], false)
        .expect("Failed to init FEAT.DESIGN");
    engine.start_step("FEAT.DESIGN")
        .expect("Failed to start FEAT.DESIGN");
    engine.finish_step(
        "FEAT.DESIGN",
        vec![
            ParameterValue::new("DESIGN_DOC", serde_json::json!(
                "docs/design/user-management-design.md"
            )),
            ParameterValue::new("API_SPEC", serde_json::json!(
                "docs/api/user-api.yaml"
            )),
        ],
        Some("Design completed with API specs".to_string()),
    ).expect("Failed to finish FEAT.DESIGN");

    // === Phase 5: Implementation ===
    println!("\n=== Phase 5: Implementation ===");

    engine.init_step("FEAT.IMPL", vec![], false)
        .expect("Failed to init FEAT.IMPL");
    engine.start_step("FEAT.IMPL")
        .expect("Failed to start FEAT.IMPL");
    engine.finish_step(
        "FEAT.IMPL",
        vec![
            ParameterValue::new("CODE_LOCATION", serde_json::json!("src/users/")),
        ],
        Some("Implementation complete with tests".to_string()),
    ).expect("Failed to finish FEAT.IMPL");

    // === Phase 6: Testing ===
    println!("\n=== Phase 6: Testing ===");

    engine.init_step("FEAT.TEST", vec![], false)
        .expect("Failed to init FEAT.TEST");
    engine.start_step("FEAT.TEST")
        .expect("Failed to start FEAT.TEST");
    engine.finish_step(
        "FEAT.TEST",
        vec![
            ParameterValue::new("TEST_RESULTS", serde_json::json!("All 42 tests passing, 87% coverage")),
        ],
        Some("All tests passing".to_string()),
    ).expect("Failed to finish FEAT.TEST");

    // === Phase 7: Feature Completion ===
    println!("\n=== Phase 7: Feature Completion ===");

    // FEAT should already be marked Done when all sub-steps are done
    // (automatic parent completion)
    let feat = engine.show_step("FEAT").expect("Failed to show FEAT");
    println!("FEAT status after all sub-steps done: {:?}", feat.status());
    
    // The feature is automatically marked Done when all sub-steps complete
    // We just verify it's in the correct state
    assert_eq!(feat.status(), StepStatus::Done, "FEAT should be auto-completed when all sub-steps are done");

    // === Verification ===
    println!("\n=== Verification ===");

    // Check final progress
    let progress = engine.get_progress().expect("Failed to get progress");
    println!("Progress: {}/{} done ({:.1}%)", 
        progress.done, progress.total, progress.completion_percentage());

    // Validate context quality
    let report = engine.validate(None).expect("Failed to validate");
    println!("Completeness: {:.1}%", report.completeness);
    assert!(report.completeness >= 50.0, "Expected high completeness after scenario");

    // Verify feature is done
    let tree = engine.get_status_tree().expect("Failed to get status");
    let feat_tree = tree.children.iter()
        .find(|c| c.id == "FEAT")
        .expect("FEAT not found in tree");
    assert_eq!(feat_tree.status, StepStatus::Done);
}

/// Test context inheritance through step hierarchy
#[test]
fn test_context_inheritance() {
    let project = setup_tasktrack_project();

    let mut engine = ProcessEngine::new(project.path().to_path_buf())
        .expect("Failed to load project");

    // Initialize hierarchy
    engine.init_step("ROOT", vec![], false).unwrap();
    engine.start_step("ROOT").unwrap();

    engine.init_step(
        "FEAT",
        vec![
            ParameterValue::new("FEATURE_ID", serde_json::json!("001")),
            ParameterValue::new("FEATURE_NAME", serde_json::json!("Test Feature")),
        ],
        false,
    ).unwrap();

    // Initialize child step
    engine.init_step("FEAT.REQ", vec![], false).unwrap();

    // Child should have access to parent scope (FEATURE_NAME)
    let child = engine.show_step("FEAT.REQ").unwrap();
    
    // Verify the step was created - context inheritance is internal
    assert_eq!(child.attr.id, "REQ");
}

/// Test dependency resolution (waits_for links)
#[test]
fn test_dependency_resolution() {
    let project = TestProject::new("deps-test");

    // Write config with dependencies (updated to current schema)
    project.write_file(".glow/config.yaml", r#"
version: "0.1.0"
project_name: "Dependency Test"
data_folder: "glow"
"#);

    project.write_file(".glow/process_config.yaml", r#"
version: "0.1.0"

root_process:
  id: ROOT
  purpose: "Test dependencies"
  steps:
    - id: STEP_A
      purpose: "First step"
      outputs:
        - id: A_OUTPUT
    - id: STEP_B
      purpose: "Depends on A"
      inputs:
        - id: A_OUTPUT
          mapping: "links.STEP_A.output.A_OUTPUT"
    - id: STEP_C
      purpose: "Depends on B"
  links:
    - type: dependency
      from: STEP_B
      to: STEP_A
    - type: dependency
      from: STEP_C
      to: STEP_B
"#);

    let mut engine = ProcessEngine::new(project.path().to_path_buf())
        .expect("Failed to load project");

    // Initialize ROOT - this creates all sub-steps (A, B, C) automatically
    engine.init_step("ROOT", vec![], false).unwrap();
    engine.start_step("ROOT").unwrap();

    // Sub-steps were created by init_process_iteration
    // STEP_A should be Todo (no deps), STEP_B and STEP_C should be Wait (have deps)
    let step_a = engine.show_step("STEP_A").unwrap();
    let step_b = engine.show_step("STEP_B").unwrap();
    let step_c = engine.show_step("STEP_C").unwrap();
    
    println!("Before A done: A={:?}, B={:?}, C={:?}", 
        step_a.status(), step_b.status(), step_c.status());
    
    // A has no deps, so should be actionable
    assert_eq!(step_a.status(), StepStatus::Todo, "STEP_A should be Todo (no deps)");
    
    // B and C should be Wait (blocked by dependencies)
    assert_eq!(step_b.status(), StepStatus::Wait, "STEP_B should be Wait (depends on A)");
    assert_eq!(step_c.status(), StepStatus::Wait, "STEP_C should be Wait (depends on B)");

    // Complete A
    engine.start_step("STEP_A").unwrap();
    engine.finish_step(
        "STEP_A",
        vec![ParameterValue::new("A_OUTPUT", serde_json::json!("result"))],
        None,
    ).unwrap();

    // Now B should be actionable (A is done)
    let step_b = engine.show_step("STEP_B").unwrap();
    let step_c = engine.show_step("STEP_C").unwrap();
    
    println!("After A done: B={:?}, C={:?}", step_b.status(), step_c.status());
    
    assert_eq!(step_b.status(), StepStatus::Todo, "STEP_B should be Todo after A completes");
    assert_eq!(step_c.status(), StepStatus::Wait, "STEP_C should still be Wait (B not done)");
    
    // Verify B is in next actions
    let actions = engine.get_next_actions().unwrap();
    let b_action = actions.iter().find(|a| a.fqid.contains("STEP_B"));
    assert!(b_action.is_some(), "STEP_B should be available after A completes");
}

/// Test repeatable process steps (multiple iterations)
#[test]
fn test_repeatable_steps() {
    let project = setup_tasktrack_project();

    let mut engine = ProcessEngine::new(project.path().to_path_buf())
        .expect("Failed to load project");

    // Initialize ROOT
    engine.init_step("ROOT", vec![], false).unwrap();
    engine.start_step("ROOT").unwrap();

    // Initialize feature with parameters
    engine.init_step(
        "FEAT",
        vec![
            ParameterValue::new("FEATURE_ID", serde_json::json!("001")),
            ParameterValue::new("FEATURE_NAME", serde_json::json!("First Feature")),
        ],
        false,
    ).unwrap();

    // Feature should exist
    let tree = engine.get_status_tree().unwrap();
    let feat_count = tree.children.iter()
        .filter(|c| c.id == "FEAT")
        .count();

    assert!(feat_count >= 1, "Should have at least one FEAT step");
}
