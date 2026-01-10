//! Context quality validation
//!
//! Validates completeness, consistency, and semantic connection.

use crate::error::Result;
use crate::model::{LinkGraph, StepStatus};

use super::operations::{IssueType, ProcessEngine, ValidationIssue, ValidationReport};

/// Context quality validator
#[derive(Debug, Default)]
pub struct Validator {
    /// Validation rules
    #[allow(dead_code)]
    rules: Vec<ValidationRule>,
}

#[derive(Debug)]
enum ValidationRule {
    RequiredParameters,
    LinkValidity,
    CircularDependency,
    StateConsistency,
}

impl Validator {
    /// Create a new validator with default rules
    pub fn new() -> Self {
        Self {
            rules: vec![
                ValidationRule::RequiredParameters,
                ValidationRule::LinkValidity,
                ValidationRule::CircularDependency,
                ValidationRule::StateConsistency,
            ],
        }
    }

    /// Validate the entire project or a specific step
    pub fn validate_project(
        &self,
        engine: &ProcessEngine,
        fqid: Option<&str>,
    ) -> Result<ValidationReport> {
        let mut report = ValidationReport {
            is_valid: true,
            completeness: 0.0,
            issues: Vec::new(),
            warnings: Vec::new(),
        };

        // Get status tree
        let status_tree = engine.get_status_tree()?;

        // Validate root or specific step
        if let Some(target_fqid) = fqid {
            self.validate_step(engine, target_fqid, &mut report)?;
        } else {
            self.validate_tree(engine, &status_tree, &mut report)?;
        }

        // Calculate completeness
        report.completeness = self.calculate_completeness(engine)?;
        report.is_valid = report.issues.is_empty();

        Ok(report)
    }

    /// Validate a single step
    fn validate_step(
        &self,
        engine: &ProcessEngine,
        fqid: &str,
        report: &mut ValidationReport,
    ) -> Result<()> {
        let step = match engine.storage().read_step(fqid) {
            Ok(s) => s,
            Err(_) => {
                report.issues.push(ValidationIssue {
                    fqid: fqid.to_string(),
                    issue_type: IssueType::InconsistentState,
                    message: "Step file not found".to_string(),
                });
                return Ok(());
            }
        };

        let step_def = match engine.process_config().find_step_definition(fqid) {
            Some(d) => d,
            None => {
                report.issues.push(ValidationIssue {
                    fqid: fqid.to_string(),
                    issue_type: IssueType::BrokenLink,
                    message: "Step definition not found in process config".to_string(),
                });
                return Ok(());
            }
        };

        // Check required inputs for in-progress/done steps
        if step.status() == StepStatus::InProgress || step.status() == StepStatus::Done {
            for input_ref in &step_def.inputs {
                if input_ref.is_required.unwrap_or(false) {
                    let has_value = step.input.iter()
                        .any(|p| p.id == input_ref.id && p.value.is_some());

                    if !has_value {
                        report.issues.push(ValidationIssue {
                            fqid: fqid.to_string(),
                            issue_type: IssueType::MissingParameter,
                            message: format!("Missing required input: {}", input_ref.id),
                        });
                    }
                }
            }
        }

        // Check required outputs for done steps
        if step.status() == StepStatus::Done {
            for output_ref in &step_def.outputs {
                if output_ref.is_required.unwrap_or(false) {
                    let has_value = step.output.iter()
                        .any(|p| p.id == output_ref.id && p.value.is_some());

                    if !has_value {
                        report.issues.push(ValidationIssue {
                            fqid: fqid.to_string(),
                            issue_type: IssueType::MissingParameter,
                            message: format!("Missing required output: {}", output_ref.id),
                        });
                    }
                }
            }
        }

        // Check for circular dependencies
        if !step_def.links.is_empty() {
            let graph = LinkGraph::from_links(&step_def.links);
            if let Some(cycle) = graph.find_cycle() {
                report.issues.push(ValidationIssue {
                    fqid: fqid.to_string(),
                    issue_type: IssueType::CircularDependency,
                    message: format!("Circular dependency detected: {:?}", cycle),
                });
            }
        }

        // Check link targets exist
        for link in &step_def.links {
            let target_exists = step_def.steps.iter().any(|s| s.id == link.to);
            if !target_exists {
                report.issues.push(ValidationIssue {
                    fqid: fqid.to_string(),
                    issue_type: IssueType::BrokenLink,
                    message: format!("Link target not found: {}", link.to),
                });
            }
        }

        Ok(())
    }

    /// Validate status tree recursively
    fn validate_tree(
        &self,
        engine: &ProcessEngine,
        tree: &super::operations::StatusTree,
        report: &mut ValidationReport,
    ) -> Result<()> {
        // Validate this node
        self.validate_step(engine, &tree.fqid, report)?;

        // Add warnings for waiting steps
        if tree.status == StepStatus::Wait && !tree.children.is_empty() {
            report.warnings.push(format!(
                "{}: {} steps waiting, no iteration started",
                tree.fqid,
                tree.children.len()
            ));
        }

        // Validate children
        for child in &tree.children {
            self.validate_tree(engine, child, report)?;
        }

        Ok(())
    }

    /// Calculate completeness percentage
    fn calculate_completeness(&self, engine: &ProcessEngine) -> Result<f64> {
        let progress = engine.get_progress()?;
        Ok(progress.completion_percentage())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let validator = Validator::new();
        assert_eq!(validator.rules.len(), 4);
    }
}
