//! Process data model
//!
//! Processes are collections of steps with iterations.

use serde::{Deserialize, Serialize};

use super::{Step, StepDefinition, StepStatus};

/// Process definition (step-as-process)
pub type ProcessDefinition = StepDefinition;

/// Runtime process with iterations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Process {
    /// Process step data
    pub step: Step,
    /// Current iteration number (1-based)
    pub current_iteration: u32,
    /// Iterations history
    pub iterations: Vec<Iteration>,
}

impl Process {
    /// Create a new process from definition
    pub fn from_definition(def: &ProcessDefinition, parent_fqid: Option<&str>) -> Self {
        Self {
            step: Step::from_definition(def, parent_fqid),
            current_iteration: 0,
            iterations: Vec::new(),
        }
    }

    /// Get the FQID
    pub fn fqid(&self) -> &str {
        self.step.fqid()
    }

    /// Get current status
    pub fn status(&self) -> StepStatus {
        self.step.status()
    }

    /// Check if this process has started
    pub fn has_started(&self) -> bool {
        self.current_iteration > 0
    }

    /// Get the current iteration if any
    pub fn current_iteration_data(&self) -> Option<&Iteration> {
        if self.current_iteration == 0 {
            None
        } else {
            self.iterations.get(self.current_iteration as usize - 1)
        }
    }

    /// Get mutable reference to current iteration
    pub fn current_iteration_data_mut(&mut self) -> Option<&mut Iteration> {
        if self.current_iteration == 0 {
            None
        } else {
            self.iterations.get_mut(self.current_iteration as usize - 1)
        }
    }

    /// Start a new iteration
    pub fn start_new_iteration(&mut self) -> &mut Iteration {
        self.current_iteration += 1;
        let iteration = Iteration {
            number: self.current_iteration,
            steps: Vec::new(),
            summary: None,
            is_complete: false,
        };
        self.iterations.push(iteration);
        self.iterations.last_mut().unwrap()
    }
}

/// A single iteration of a process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iteration {
    /// Iteration number (1-based)
    pub number: u32,
    /// Steps in this iteration
    pub steps: Vec<Step>,
    /// Iteration summary
    pub summary: Option<String>,
    /// Whether this iteration is complete
    pub is_complete: bool,
}

impl Iteration {
    /// Get the folder name for this iteration
    pub fn folder_name(&self) -> String {
        format!("iteration_{:06}", self.number)
    }

    /// Find a step by ID
    pub fn find_step(&self, id: &str) -> Option<&Step> {
        self.steps.iter().find(|s| s.attr.id == id)
    }

    /// Find a step by ID mutably
    pub fn find_step_mut(&mut self, id: &str) -> Option<&mut Step> {
        self.steps.iter_mut().find(|s| s.attr.id == id)
    }

    /// Check if all steps are complete
    pub fn all_steps_complete(&self) -> bool {
        self.steps.iter().all(|s| s.status().is_complete())
    }

    /// Count steps by status
    pub fn count_by_status(&self) -> StatusCounts {
        let mut counts = StatusCounts::default();
        for step in &self.steps {
            match step.status() {
                StepStatus::Wait => counts.wait += 1,
                StepStatus::Todo => counts.todo += 1,
                StepStatus::InProgress => counts.in_progress += 1,
                StepStatus::Done => counts.done += 1,
            }
        }
        counts
    }

    /// Get steps that are ready to start (todo status)
    pub fn ready_steps(&self) -> Vec<&Step> {
        self.steps
            .iter()
            .filter(|s| s.status() == StepStatus::Todo)
            .collect()
    }

    /// Get steps that are currently in progress
    pub fn active_steps(&self) -> Vec<&Step> {
        self.steps
            .iter()
            .filter(|s| s.status() == StepStatus::InProgress)
            .collect()
    }

    /// Get steps that are blocked
    pub fn blocked_steps(&self) -> Vec<&Step> {
        self.steps
            .iter()
            .filter(|s| s.status() == StepStatus::Wait)
            .collect()
    }
}

/// Step counts by status
#[derive(Debug, Clone, Default)]
pub struct StatusCounts {
    pub wait: usize,
    pub todo: usize,
    pub in_progress: usize,
    pub done: usize,
}

impl StatusCounts {
    /// Get total count
    pub fn total(&self) -> usize {
        self.wait + self.todo + self.in_progress + self.done
    }

    /// Get completion percentage
    pub fn completion_percentage(&self) -> f64 {
        if self.total() == 0 {
            0.0
        } else {
            (self.done as f64 / self.total() as f64) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iteration_folder_name() {
        let iter = Iteration {
            number: 1,
            steps: Vec::new(),
            summary: None,
            is_complete: false,
        };
        assert_eq!(iter.folder_name(), "iteration_000001");

        let iter = Iteration {
            number: 42,
            steps: Vec::new(),
            summary: None,
            is_complete: false,
        };
        assert_eq!(iter.folder_name(), "iteration_000042");
    }

    #[test]
    fn test_status_counts() {
        let mut counts = StatusCounts::default();
        counts.done = 3;
        counts.wait = 2;

        assert_eq!(counts.total(), 5);
        assert_eq!(counts.completion_percentage(), 60.0);
    }

    #[test]
    fn test_process_from_definition() {
        let def = ProcessDefinition::new_root();
        let process = Process::from_definition(&def, None);

        assert_eq!(process.fqid(), "ROOT");
        assert_eq!(process.current_iteration, 0);
        assert!(!process.has_started());
    }

    #[test]
    fn test_process_start_iteration() {
        let def = ProcessDefinition::new_root();
        let mut process = Process::from_definition(&def, None);

        process.start_new_iteration();
        assert!(process.has_started());
        assert_eq!(process.current_iteration, 1);
        assert!(process.current_iteration_data().is_some());
    }
}
