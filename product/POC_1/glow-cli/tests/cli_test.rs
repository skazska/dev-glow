//! CLI Integration tests

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn setup_test_project() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let path = temp_dir.path();

    // Create directory structure
    fs::create_dir_all(path.join(".glow/schemas")).expect("Failed to create .glow dir");
    fs::create_dir_all(path.join("glow")).expect("Failed to create glow dir");

    // Write config
    let config = r#"
version: "1.0"
project_id: "TEST"
project_name: "Test Project"
process: "simple"
glow_dir: "glow"
"#;
    fs::write(path.join(".glow/config.yaml"), config).expect("Failed to write config");

    // Write process config
    let process_config = r#"
version: "1.0"
process:
  id: "simple"
  name: "Simple Process"

steps:
  - id: "ROOT"
    classification: "Process"
    purpose: "Main process"
    own_steps:
      - id: "TASK"
        classification: "Task"
        purpose: "A simple task"
"#;
    fs::write(path.join(".glow/process_config.yaml"), process_config)
        .expect("Failed to write process config");

    temp_dir
}

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("glow").expect("Failed to find binary");
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Development Process Management"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("glow").expect("Failed to find binary");
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_cli_init_step() {
    let project = setup_test_project();

    let mut cmd = Command::cargo_bin("glow").expect("Failed to find binary");
    cmd.current_dir(project.path())
        .args(["init", "ROOT"])
        .assert()
        .success()
        .stdout(predicate::str::contains("initialized"));

    // Verify step file was created
    assert!(project.path().join("glow/ROOT.step.md").exists());
}

#[test]
fn test_cli_status() {
    let project = setup_test_project();

    // First init ROOT
    let mut init_cmd = Command::cargo_bin("glow").expect("Failed to find binary");
    init_cmd.current_dir(project.path())
        .args(["init", "ROOT"])
        .assert()
        .success();

    // Then check status
    let mut status_cmd = Command::cargo_bin("glow").expect("Failed to find binary");
    status_cmd.current_dir(project.path())
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("ROOT"));
}

#[test]
fn test_cli_next() {
    let project = setup_test_project();

    // Init ROOT
    let mut init_cmd = Command::cargo_bin("glow").expect("Failed to find binary");
    init_cmd.current_dir(project.path())
        .args(["init", "ROOT"])
        .assert()
        .success();

    // Check next actions
    let mut next_cmd = Command::cargo_bin("glow").expect("Failed to find binary");
    next_cmd.current_dir(project.path())
        .arg("next")
        .assert()
        .success()
        .stdout(predicate::str::contains("Recommended"));
}

#[test]
fn test_cli_progress() {
    let project = setup_test_project();

    // Init ROOT
    let mut init_cmd = Command::cargo_bin("glow").expect("Failed to find binary");
    init_cmd.current_dir(project.path())
        .args(["init", "ROOT"])
        .assert()
        .success();

    // Check progress
    let mut progress_cmd = Command::cargo_bin("glow").expect("Failed to find binary");
    progress_cmd.current_dir(project.path())
        .arg("progress")
        .assert()
        .success()
        .stdout(predicate::str::contains("Progress"));
}

#[test]
fn test_cli_workflow() {
    let project = setup_test_project();
    let project_path = project.path();

    // Init ROOT
    Command::cargo_bin("glow").unwrap()
        .current_dir(project_path)
        .args(["init", "ROOT"])
        .assert()
        .success();

    // Start ROOT
    Command::cargo_bin("glow").unwrap()
        .current_dir(project_path)
        .args(["start", "ROOT"])
        .assert()
        .success()
        .stdout(predicate::str::contains("started"));

    // Show ROOT
    Command::cargo_bin("glow").unwrap()
        .current_dir(project_path)
        .args(["show", "ROOT"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Step"));

    // Validate
    Command::cargo_bin("glow").unwrap()
        .current_dir(project_path)
        .arg("validate")
        .assert()
        .success();
}
