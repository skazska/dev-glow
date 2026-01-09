//! CLI command implementations

use std::path::Path;

use anyhow::{Context, Result};
use colored::Colorize;
use glow_core::engine::operations::{ProcessEngine, StatusTree};
use glow_core::model::{ParameterValue, StepStatus};

use crate::output::{print_error, print_info, print_success, print_tree, print_warning};

/// Initialize a new project
pub fn project_init(project_dir: &Path, name: Option<String>) -> Result<()> {
    print_info("Initializing glow project...");

    match ProcessEngine::init_project(project_dir.to_path_buf(), name) {
        Ok(_engine) => {
            print_success(&format!(
                "Project initialized in {}",
                project_dir.display()
            ));
            print_info("Created:");
            println!("  {} Configuration and schemas", ".glow/".cyan());
            println!("  {} Process data directory", "glow/".cyan());
            print_info("\nNext steps:");
            println!("  1. Edit {} to configure your process", ".glow/process_config.yaml".cyan());
            println!("  2. Run {} to start your process", "glow init ROOT".cyan());
            Ok(())
        }
        Err(e) => {
            print_error(&format!("Failed to initialize project: {}", e));
            Err(e.into())
        }
    }
}

/// Initialize a step
pub fn init_step(
    project_dir: &Path,
    fqid: &str,
    force_new: bool,
    params: Vec<String>,
) -> Result<()> {
    let mut engine = ProcessEngine::new(project_dir.to_path_buf())
        .context("Failed to load project")?;

    let scope_params = parse_params(&params)?;

    print_info(&format!("Initializing step {}...", fqid.cyan()));

    match engine.init_step(fqid, scope_params, force_new) {
        Ok(step) => {
            print_success(&format!("Step {} initialized", fqid.cyan()));
            println!("  Status: {}", format_status(step.status()));

            if !step.own_steps.is_empty() {
                print_info("Sub-steps created:");
                for sub in &step.own_steps {
                    println!("  {} {}", format_status(sub.status), sub.id.cyan());
                }
            }

            Ok(())
        }
        Err(e) => {
            print_error(&format!("Failed to initialize step: {}", e));
            Err(e.into())
        }
    }
}

/// Start a step
pub fn start_step(project_dir: &Path, fqid: &str) -> Result<()> {
    let mut engine = ProcessEngine::new(project_dir.to_path_buf())
        .context("Failed to load project")?;

    print_info(&format!("Starting step {}...", fqid.cyan()));

    match engine.start_step(fqid) {
        Ok(step) => {
            print_success(&format!("Step {} started", fqid.cyan()));

            if let Some(purpose) = &step.attr.purpose {
                println!("\n{}", "Purpose:".bold());
                println!("  {}", purpose);
            }

            if let Some(expectations) = &step.attr.expectations {
                println!("\n{}", "Expectations:".bold());
                println!("  {}", expectations);
            }

            print_info(&format!(
                "\nView details: {}",
                format!("glow show {}", fqid).cyan()
            ));
            Ok(())
        }
        Err(e) => {
            print_error(&format!("Failed to start step: {}", e));
            Err(e.into())
        }
    }
}

/// Finish a step
pub fn finish_step(
    project_dir: &Path,
    fqid: &str,
    summary: Option<String>,
    params: Vec<String>,
) -> Result<()> {
    let mut engine = ProcessEngine::new(project_dir.to_path_buf())
        .context("Failed to load project")?;

    let output_params = parse_params(&params)?;

    print_info(&format!("Finishing step {}...", fqid.cyan()));

    match engine.finish_step(fqid, output_params, summary) {
        Ok(step) => {
            print_success(&format!("Step {} completed", fqid.cyan()));

            // Show next available actions
            if let Ok(next_actions) = engine.get_next_actions() {
                if !next_actions.is_empty() {
                    print_info("\nNext available actions:");
                    for action in next_actions.iter().take(3) {
                        println!("  {} {}", "→".green(), action.description);
                    }
                }
            }

            Ok(())
        }
        Err(e) => {
            print_error(&format!("Failed to finish step: {}", e));
            Err(e.into())
        }
    }
}

/// Show project status
pub fn show_status(
    project_dir: &Path,
    as_list: bool,
    _attrs: &str,
    _filter: Option<&str>,
) -> Result<()> {
    let engine = ProcessEngine::new(project_dir.to_path_buf())
        .context("Failed to load project")?;

    let status_tree = engine.get_status_tree()
        .context("Failed to get status tree")?;

    if as_list {
        print_status_list(&status_tree, "");
    } else {
        print_tree(&status_tree);
    }

    Ok(())
}

/// Print status as flat list
fn print_status_list(tree: &StatusTree, prefix: &str) {
    let fqid = if prefix.is_empty() {
        tree.id.clone()
    } else {
        format!("{}.{}", prefix, tree.id)
    };

    println!(
        "{} {} {}",
        format_status(tree.status),
        fqid.cyan(),
        tree.purpose.as_deref().unwrap_or("").dimmed()
    );

    for child in &tree.children {
        print_status_list(child, &fqid);
    }
}

/// Show step details
pub fn show_step(project_dir: &Path, fqid: &str, _include_context: bool) -> Result<()> {
    let engine = ProcessEngine::new(project_dir.to_path_buf())
        .context("Failed to load project")?;

    let step = engine.show_step(fqid)
        .context("Failed to get step")?;

    println!("{}", "═".repeat(60).dimmed());
    println!("{} {}", "Step:".bold(), step.attr.id.cyan().bold());
    println!("{} {}", "FQID:".bold(), step.fqid().cyan());
    println!("{} {}", "Status:".bold(), format_status(step.status()));

    if let Some(classification) = &step.attr.classification {
        println!("{} {}", "Classification:".bold(), classification);
    }

    if let Some(purpose) = &step.attr.purpose {
        println!("\n{}", "Purpose:".bold());
        println!("  {}", purpose);
    }

    if let Some(expectations) = &step.attr.expectations {
        println!("\n{}", "Expectations:".bold());
        println!("  {}", expectations);
    }

    if !step.input.is_empty() {
        println!("\n{}", "Inputs:".bold());
        for param in &step.input {
            let value = param.value.as_ref()
                .map(|v| v.to_string())
                .unwrap_or_else(|| "(not set)".dimmed().to_string());
            println!("  {}: {}", param.id.cyan(), value);
        }
    }

    if !step.scope.is_empty() {
        println!("\n{}", "Scope:".bold());
        for param in &step.scope {
            let value = param.value.as_ref()
                .map(|v| v.to_string())
                .unwrap_or_else(|| "(not set)".dimmed().to_string());
            println!("  {}: {}", param.id.cyan(), value);
        }
    }

    if !step.output.is_empty() {
        println!("\n{}", "Outputs:".bold());
        for param in &step.output {
            let value = param.value.as_ref()
                .map(|v| v.to_string())
                .unwrap_or_else(|| "(not set)".dimmed().to_string());
            println!("  {}: {}", param.id.cyan(), value);
        }
    }

    if !step.own_steps.is_empty() {
        println!("\n{}", "Sub-steps:".bold());
        for sub in &step.own_steps {
            println!("  {} {}", format_status(sub.status), sub.id.cyan());
        }
    }

    println!("{}", "═".repeat(60).dimmed());

    Ok(())
}

/// Show next actions
pub fn show_next(project_dir: &Path) -> Result<()> {
    let engine = ProcessEngine::new(project_dir.to_path_buf())
        .context("Failed to load project")?;

    let next_actions = engine.get_next_actions()
        .context("Failed to get next actions")?;

    if next_actions.is_empty() {
        print_info("No pending actions. All steps are complete or waiting.");
    } else {
        println!("{}", "Recommended Actions:".bold());
        println!();

        for action in &next_actions {
            let cmd = match action.action_type {
                glow_core::engine::operations::ActionType::Start => {
                    format!("glow start {}", action.fqid)
                }
                glow_core::engine::operations::ActionType::Init => {
                    format!("glow init {}", action.fqid)
                }
                _ => action.fqid.clone(),
            };

            println!(
                "  {} {}",
                "→".green().bold(),
                action.description
            );
            println!("    {}", cmd.cyan());
            println!();
        }
    }

    Ok(())
}

/// Show progress metrics
pub fn show_progress(project_dir: &Path, format: &str) -> Result<()> {
    let engine = ProcessEngine::new(project_dir.to_path_buf())
        .context("Failed to load project")?;

    let progress = engine.get_progress()
        .context("Failed to get progress")?;

    if format == "json" {
        let json = serde_json::json!({
            "total": progress.total,
            "wait": progress.wait,
            "todo": progress.todo,
            "in_progress": progress.in_progress,
            "done": progress.done,
            "completion_percentage": progress.completion_percentage()
        });
        println!("{}", serde_json::to_string_pretty(&json)?);
    } else {
        println!("{}", "Progress Metrics".bold());
        println!("{}", "─".repeat(40).dimmed());
        println!();

        // Progress bar
        let pct = progress.completion_percentage();
        let bar_width = 30;
        let filled = (pct / 100.0 * bar_width as f64) as usize;
        let bar = format!(
            "[{}{}] {:.1}%",
            "█".repeat(filled).green(),
            "░".repeat(bar_width - filled).dimmed(),
            pct
        );
        println!("  {}", bar);
        println!();

        // Counts
        println!("  {} {}", "Total Steps:".bold(), progress.total);
        println!(
            "    {} {} ({}%)",
            format_status(StepStatus::Done),
            progress.done,
            if progress.total > 0 { progress.done * 100 / progress.total } else { 0 }
        );
        println!(
            "    {} {} ({}%)",
            format_status(StepStatus::InProgress),
            progress.in_progress,
            if progress.total > 0 { progress.in_progress * 100 / progress.total } else { 0 }
        );
        println!(
            "    {} {} ({}%)",
            format_status(StepStatus::Todo),
            progress.todo,
            if progress.total > 0 { progress.todo * 100 / progress.total } else { 0 }
        );
        println!(
            "    {} {} ({}%)",
            format_status(StepStatus::Wait),
            progress.wait,
            if progress.total > 0 { progress.wait * 100 / progress.total } else { 0 }
        );
    }

    Ok(())
}

/// Validate context quality
pub fn validate(project_dir: &Path, fqid: Option<&str>, _fix: bool) -> Result<()> {
    let engine = ProcessEngine::new(project_dir.to_path_buf())
        .context("Failed to load project")?;

    let report = engine.validate(fqid)
        .context("Failed to validate")?;

    println!("{}", "Context Quality Report".bold());
    println!("{}", "─".repeat(40).dimmed());
    println!();

    if report.is_valid {
        print_success(&format!("✓ Completeness: {:.1}%", report.completeness));
        print_success("✓ Consistency: Valid");
        print_success("✓ Semantic Connection: Valid");
    } else {
        println!(
            "  {} Completeness: {:.1}%",
            if report.completeness >= 80.0 { "✓".green() } else { "✗".red() },
            report.completeness
        );
        println!(
            "  {} Consistency: {}",
            if report.issues.is_empty() { "✓".green() } else { "✗".red() },
            if report.issues.is_empty() { "Valid" } else { "Issues found" }
        );
    }

    if !report.issues.is_empty() {
        println!();
        println!("{}", "Issues:".bold().red());
        for issue in &report.issues {
            println!("  {} {}: {}", "✗".red(), issue.fqid.cyan(), issue.message);
        }
    }

    if !report.warnings.is_empty() {
        println!();
        println!("{}", "Warnings:".bold().yellow());
        for warning in &report.warnings {
            println!("  {} {}", "⚠".yellow(), warning);
        }
    }

    Ok(())
}

/// Parse parameter arguments (--PARAM=value format)
fn parse_params(params: &[String]) -> Result<Vec<ParameterValue>> {
    let mut result = Vec::new();

    for param in params {
        let param = param.trim_start_matches("--");
        if let Some((key, value)) = param.split_once('=') {
            result.push(ParameterValue::new(key, serde_json::json!(value)));
        } else {
            print_warning(&format!("Ignoring malformed parameter: {}", param));
        }
    }

    Ok(result)
}

/// Format status with colors
fn format_status(status: StepStatus) -> colored::ColoredString {
    match status {
        StepStatus::Wait => "○".dimmed(),
        StepStatus::Todo => "◐".yellow(),
        StepStatus::InProgress => "◑".blue().bold(),
        StepStatus::Done => "●".green(),
    }
}
