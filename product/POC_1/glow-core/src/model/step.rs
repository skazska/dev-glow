//! Step data model
//!
//! Steps represent tasks or sub-processes in the development workflow.

use serde::{Deserialize, Serialize};
use std::fmt;

use super::{LinkDefinition, ParameterRef};

/// Step status in the workflow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum StepStatus {
    /// Waiting for dependencies to complete
    Wait,
    /// Ready to start (all dependencies done)
    Todo,
    /// Currently being worked on
    InProgress,
    /// Completed successfully
    Done,
}

impl StepStatus {
    /// Check if a transition to the target status is valid
    pub fn can_transition_to(&self, target: StepStatus) -> bool {
        matches!(
            (self, target),
            (StepStatus::Wait, StepStatus::Todo)
                | (StepStatus::Todo, StepStatus::InProgress)
                | (StepStatus::InProgress, StepStatus::Done)
                | (StepStatus::InProgress, StepStatus::Todo) // Rollback
        )
    }

    /// Get all valid transitions from this status
    pub fn valid_transitions(&self) -> Vec<StepStatus> {
        match self {
            StepStatus::Wait => vec![StepStatus::Todo],
            StepStatus::Todo => vec![StepStatus::InProgress],
            StepStatus::InProgress => vec![StepStatus::Done, StepStatus::Todo],
            StepStatus::Done => vec![],
        }
    }

    /// Check if this status represents a completed state
    pub fn is_complete(&self) -> bool {
        matches!(self, StepStatus::Done)
    }

    /// Check if this status represents an active state
    pub fn is_active(&self) -> bool {
        matches!(self, StepStatus::InProgress)
    }

    /// Check if this status represents a blocked state
    pub fn is_blocked(&self) -> bool {
        matches!(self, StepStatus::Wait)
    }
}

impl Default for StepStatus {
    fn default() -> Self {
        StepStatus::Wait
    }
}

impl fmt::Display for StepStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StepStatus::Wait => write!(f, "wait"),
            StepStatus::Todo => write!(f, "todo"),
            StepStatus::InProgress => write!(f, "in-progress"),
            StepStatus::Done => write!(f, "done"),
        }
    }
}

impl std::str::FromStr for StepStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "wait" => Ok(StepStatus::Wait),
            "todo" => Ok(StepStatus::Todo),
            "in-progress" | "inprogress" | "in_progress" => Ok(StepStatus::InProgress),
            "done" => Ok(StepStatus::Done),
            _ => Err(format!("Invalid step status: {}", s)),
        }
    }
}

/// Step definition in process configuration
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct StepDefinition {
    /// Step identifier (ROOT for root process)
    pub id: String,
    /// Description of step's purpose or destiny
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    /// Classification expression (e.g., "Feature,Backend")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    /// Criteria to consider step done
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expectations: Option<String>,
    /// Template file name override
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    /// Input parameters for this step
    #[serde(default)]
    pub inputs: Vec<ParameterRef>,
    /// Output parameters produced by this step
    #[serde(default)]
    pub outputs: Vec<ParameterRef>,
    /// Scope parameters for this step
    #[serde(default)]
    pub scope: Vec<ParameterRef>,
    /// Sub-steps for step-as-process
    #[serde(default)]
    pub steps: Vec<StepDefinition>,
    /// Links between sub-steps
    #[serde(default)]
    pub links: Vec<LinkDefinition>,
    /// Whether multiple iterations are allowed
    #[serde(default = "default_allow_iterations")]
    pub allow_iterations: bool,
}

fn default_allow_iterations() -> bool {
    true
}

impl StepDefinition {
    /// Create a new root step definition
    pub fn new_root() -> Self {
        Self {
            id: "ROOT".to_string(),
            purpose: Some("Development Process".to_string()),
            classification: None,
            expectations: None,
            template: None,
            inputs: Vec::new(),
            outputs: Vec::new(),
            scope: Vec::new(),
            steps: Vec::new(),
            links: Vec::new(),
            allow_iterations: true,
        }
    }

    /// Check if this step definition is a process (has sub-steps)
    pub fn is_process(&self) -> bool {
        !self.steps.is_empty()
    }

    /// Check if this step definition is a task (no sub-steps)
    pub fn is_task(&self) -> bool {
        self.steps.is_empty()
    }

    /// Find a sub-step by ID
    pub fn find_step(&self, id: &str) -> Option<&StepDefinition> {
        self.steps.iter().find(|s| s.id == id)
    }

    /// Get all step IDs in this definition
    pub fn all_step_ids(&self) -> Vec<&str> {
        self.steps.iter().map(|s| s.id.as_str()).collect()
    }
}

/// Step attributes stored in data files
#[derive(Debug, Clone, Default, Serialize, Deserialize, schemars::JsonSchema)]
pub struct StepAttributes {
    /// Step identifier within parent scope
    pub id: String,
    /// Fully qualified identifier (dot-separated path from root)
    #[serde(default)]
    pub fqid: Option<String>,
    /// Classification expression
    #[serde(default)]
    pub classification: Option<String>,
    /// Step purpose or destiny
    #[serde(default)]
    pub purpose: Option<String>,
    /// Criteria to consider step done
    #[serde(default)]
    pub expectations: Option<String>,
    /// Current step status
    #[serde(default)]
    pub status: StepStatus,
}

impl StepAttributes {
    /// Create new step attributes from a definition
    pub fn from_definition(def: &StepDefinition, parent_fqid: Option<&str>) -> Self {
        let fqid = match parent_fqid {
            Some(parent) if parent != "ROOT" => Some(format!("{}.{}", parent, def.id)),
            Some(_) => Some(def.id.clone()), // Parent is ROOT, don't include in FQID
            None => {
                if def.id == "ROOT" {
                    Some("ROOT".to_string())
                } else {
                    Some(def.id.clone())
                }
            }
        };

        Self {
            id: def.id.clone(),
            fqid,
            classification: def.classification.clone(),
            purpose: def.purpose.clone(),
            expectations: def.expectations.clone(),
            status: StepStatus::Wait,
        }
    }
}

/// Runtime step instance with data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    /// Step attributes
    pub attr: StepAttributes,
    /// Input parameter values
    #[serde(default)]
    pub input: Vec<super::ParameterValue>,
    /// Scope parameter values
    #[serde(default)]
    pub scope: Vec<super::ParameterValue>,
    /// Output parameter values
    #[serde(default)]
    pub output: Vec<super::ParameterValue>,
    /// Parent step references (execution stack)
    #[serde(default)]
    pub parent: Vec<ParentRef>,
    /// Sub-steps for step-as-process
    #[serde(default)]
    pub own_steps: Vec<StepRef>,
    /// Links to other steps
    #[serde(default)]
    pub links: Vec<LinkRef>,
}

impl Step {
    /// Create a new step from definition
    pub fn from_definition(def: &StepDefinition, parent_fqid: Option<&str>) -> Self {
        Self {
            attr: StepAttributes::from_definition(def, parent_fqid),
            input: Vec::new(),
            scope: Vec::new(),
            output: Vec::new(),
            parent: Vec::new(),
            own_steps: Vec::new(),
            links: Vec::new(),
        }
    }

    /// Get the fully qualified ID
    pub fn fqid(&self) -> &str {
        self.attr.fqid.as_deref().unwrap_or(&self.attr.id)
    }

    /// Get the current status
    pub fn status(&self) -> StepStatus {
        self.attr.status
    }

    /// Check if this step is a process (has sub-steps)
    pub fn is_process(&self) -> bool {
        !self.own_steps.is_empty()
    }

    /// Get input parameter value by ID
    pub fn get_input(&self, id: &str) -> Option<&super::ParameterValue> {
        self.input.iter().find(|p| p.id == id)
    }

    /// Get scope parameter value by ID
    pub fn get_scope(&self, id: &str) -> Option<&super::ParameterValue> {
        self.scope.iter().find(|p| p.id == id)
    }

    /// Get output parameter value by ID
    pub fn get_output(&self, id: &str) -> Option<&super::ParameterValue> {
        self.output.iter().find(|p| p.id == id)
    }
}

/// Reference to parent step in execution stack
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ParentRef {
    /// Parent step ID
    pub id: String,
    /// Sibling steps in parent's iteration
    #[serde(default)]
    pub steps: Vec<StepRef>,
}

/// Reference to a step with status
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct StepRef {
    /// Step ID
    pub id: String,
    /// Step status
    pub status: StepStatus,
}

/// Reference to a linked step
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct LinkRef {
    /// Linked step ID
    pub step_id: String,
    /// Type of link (dependency, predecessor, or custom)
    pub link_type: String,
    /// Current status of linked step
    #[serde(default)]
    pub step_status: Option<StepStatus>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_status_transitions() {
        assert!(StepStatus::Wait.can_transition_to(StepStatus::Todo));
        assert!(!StepStatus::Wait.can_transition_to(StepStatus::Done));
        assert!(StepStatus::Todo.can_transition_to(StepStatus::InProgress));
        assert!(StepStatus::InProgress.can_transition_to(StepStatus::Done));
        assert!(StepStatus::InProgress.can_transition_to(StepStatus::Todo)); // Rollback
        assert!(!StepStatus::Done.can_transition_to(StepStatus::InProgress));
    }

    #[test]
    fn test_step_status_display() {
        assert_eq!(format!("{}", StepStatus::Wait), "wait");
        assert_eq!(format!("{}", StepStatus::InProgress), "in-progress");
    }

    #[test]
    fn test_step_status_parse() {
        assert_eq!("wait".parse::<StepStatus>().unwrap(), StepStatus::Wait);
        assert_eq!(
            "in-progress".parse::<StepStatus>().unwrap(),
            StepStatus::InProgress
        );
    }

    #[test]
    fn test_step_definition_is_process() {
        let mut def = StepDefinition::new_root();
        assert!(!def.is_process());

        def.steps.push(StepDefinition {
            id: "CHILD".to_string(),
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
        });
        assert!(def.is_process());
    }

    #[test]
    fn test_step_attributes_fqid() {
        let def = StepDefinition {
            id: "FEAT-001".to_string(),
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
        };

        // Child of ROOT
        let attrs = StepAttributes::from_definition(&def, Some("ROOT"));
        assert_eq!(attrs.fqid, Some("FEAT-001".to_string()));

        // Nested child
        let attrs = StepAttributes::from_definition(&def, Some("FEAT-001"));
        assert_eq!(attrs.fqid, Some("FEAT-001.FEAT-001".to_string()));
    }
}
