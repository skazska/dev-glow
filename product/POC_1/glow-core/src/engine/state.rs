//! State management
//!
//! Handles step state transitions and validation.

use crate::error::{GlowError, Result};
use crate::model::StepStatus;

/// State manager for step transitions
#[derive(Debug, Default)]
pub struct StateManager {
    /// Track active steps
    active_steps: Vec<String>,
}

impl StateManager {
    /// Create a new state manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate a state transition
    pub fn validate_transition(&self, from: StepStatus, to: StepStatus) -> Result<()> {
        if from.can_transition_to(to) {
            Ok(())
        } else {
            Err(GlowError::InvalidStateTransition {
                step_id: String::new(),
                current: from.to_string(),
                target: to.to_string(),
            })
        }
    }

    /// Record step becoming active
    pub fn mark_active(&mut self, fqid: &str) {
        if !self.active_steps.contains(&fqid.to_string()) {
            self.active_steps.push(fqid.to_string());
        }
    }

    /// Record step becoming inactive
    pub fn mark_inactive(&mut self, fqid: &str) {
        self.active_steps.retain(|s| s != fqid);
    }

    /// Get all active steps
    pub fn active_steps(&self) -> &[String] {
        &self.active_steps
    }

    /// Check if a step is active
    pub fn is_active(&self, fqid: &str) -> bool {
        self.active_steps.contains(&fqid.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        let sm = StateManager::new();

        assert!(sm.validate_transition(StepStatus::Wait, StepStatus::Todo).is_ok());
        assert!(sm.validate_transition(StepStatus::Todo, StepStatus::InProgress).is_ok());
        assert!(sm.validate_transition(StepStatus::InProgress, StepStatus::Done).is_ok());
    }

    #[test]
    fn test_invalid_transitions() {
        let sm = StateManager::new();

        assert!(sm.validate_transition(StepStatus::Wait, StepStatus::Done).is_err());
        assert!(sm.validate_transition(StepStatus::Done, StepStatus::Wait).is_err());
    }

    #[test]
    fn test_active_tracking() {
        let mut sm = StateManager::new();

        sm.mark_active("FEAT-001");
        assert!(sm.is_active("FEAT-001"));

        sm.mark_inactive("FEAT-001");
        assert!(!sm.is_active("FEAT-001"));
    }
}
