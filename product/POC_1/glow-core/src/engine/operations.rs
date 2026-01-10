//! Process operations
//!
//! Main process engine implementation.

use std::path::{Path, PathBuf};

use crate::config::{Config, ConfigLoader, ProcessConfig};
use crate::error::{GlowError, Result};
use crate::model::{
    LinkGraph, ParameterValue, Step, StepDefinition, StepRef, StepStatus,
};
use crate::storage::Storage;
use crate::template::TemplateEngine;

use super::state::StateManager;
use super::validation::Validator;

/// Main process engine
pub struct ProcessEngine {
    /// Project root path
    project_root: PathBuf,
    /// Configuration loader
    #[allow(dead_code)]
    config_loader: ConfigLoader,
    /// Project configuration
    config: Config,
    /// Process configuration
    process_config: ProcessConfig,
    /// Storage layer
    storage: Storage,
    /// Template engine
    template_engine: TemplateEngine,
    /// State manager
    #[allow(dead_code)]
    state_manager: StateManager,
    /// Validator
    validator: Validator,
}

impl ProcessEngine {
    /// Create a new process engine for an existing project
    pub fn new(project_root: PathBuf) -> Result<Self> {
        let config_dir = crate::config::find_config_dir(&project_root, None)?;
        let config_loader = ConfigLoader::new(config_dir.clone(), project_root.clone());

        let config = config_loader.load_config()?;
        let process_config = config_loader.load_process_config(&config)?;

        let storage = Storage::new(project_root.clone(), &config, config_dir.clone());
        let templates_dir = crate::config::templates_dir(&config_dir);
        let template_engine = TemplateEngine::new(templates_dir);

        let state_manager = StateManager::new();
        let validator = Validator::new();

        Ok(Self {
            project_root,
            config_loader,
            config,
            process_config,
            storage,
            template_engine,
            state_manager,
            validator,
        })
    }

    /// Initialize a new project
    pub fn init_project(project_root: PathBuf, project_name: Option<String>) -> Result<Self> {
        let config_dir = crate::config::default_config_dir(&project_root);

        // Check if already initialized
        if config_dir.exists() {
            return Err(GlowError::ProjectAlreadyExists {
                path: config_dir,
            });
        }

        // Create config directory
        std::fs::create_dir_all(&config_dir)?;

        // Create schemas directory and copy schemas
        let schemas_dir = crate::config::schemas_dir(&config_dir);
        std::fs::create_dir_all(&schemas_dir)?;
        Self::write_schema_files(&schemas_dir)?;

        // Create templates directory
        let templates_dir = crate::config::templates_dir(&config_dir);
        std::fs::create_dir_all(&templates_dir)?;
        Self::write_default_template(&templates_dir)?;

        // Create config file
        let config = if let Some(name) = project_name {
            Config::new(name)
        } else {
            Config::default()
        };

        let config_loader = ConfigLoader::new(config_dir.clone(), project_root.clone());
        config_loader.save_config(&config)?;

        // Create process config
        let process_config = ProcessConfig::default();
        config_loader.save_process_config(&config, &process_config)?;

        // Create storage
        let storage = Storage::new(project_root.clone(), &config, config_dir.clone());
        storage.init_data_dir()?;

        let template_engine = TemplateEngine::new(templates_dir);
        let state_manager = StateManager::new();
        let validator = Validator::new();

        Ok(Self {
            project_root,
            config_loader,
            config,
            process_config,
            storage,
            template_engine,
            state_manager,
            validator,
        })
    }

    /// Write schema files to schemas directory
    fn write_schema_files(schemas_dir: &Path) -> Result<()> {
        std::fs::write(
            schemas_dir.join("config.schema.json"),
            crate::config::schema::CONFIG_SCHEMA,
        )?;
        std::fs::write(
            schemas_dir.join("process_config.schema.json"),
            crate::config::schema::PROCESS_CONFIG_SCHEMA,
        )?;
        std::fs::write(
            schemas_dir.join("step_data.schema.json"),
            crate::config::schema::STEP_DATA_SCHEMA,
        )?;
        std::fs::write(
            schemas_dir.join("mcp_config.schema.json"),
            crate::config::schema::MCP_CONFIG_SCHEMA,
        )?;
        Ok(())
    }

    /// Write default template
    fn write_default_template(templates_dir: &Path) -> Result<()> {
        std::fs::write(
            templates_dir.join("any-step.md"),
            crate::template::DEFAULT_TEMPLATE,
        )?;
        Ok(())
    }

    /// Get project root
    pub fn project_root(&self) -> &Path {
        &self.project_root
    }

    /// Get config
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get process config
    pub fn process_config(&self) -> &ProcessConfig {
        &self.process_config
    }

    /// Get storage
    pub fn storage(&self) -> &Storage {
        &self.storage
    }

    /// Initialize a process step
    pub fn init_step(
        &mut self,
        fqid: &str,
        scope_params: Vec<ParameterValue>,
        force_new_iteration: bool,
    ) -> Result<Step> {
        let step_def = self.process_config.find_step_definition(fqid)
            .ok_or_else(|| GlowError::StepNotFound { fqid: fqid.to_string() })?
            .clone();

        // Check if step already exists and is in progress
        if self.storage.step_files().step_exists(fqid) {
            let existing = self.storage.read_step(fqid)?;
            if existing.status() == StepStatus::InProgress && !force_new_iteration {
                // Return existing step
                return Ok(existing);
            }
        }

        // Validate dependencies are satisfied
        if fqid != "ROOT" {
            self.validate_dependencies_for_step(fqid)?;
        }

        // Create or update step
        let parent_fqid = self.get_parent_fqid(fqid);
        let mut step = Step::from_definition(&step_def, parent_fqid.as_deref());

        // Set scope parameters
        for param in scope_params {
            step.scope.push(param);
        }

        // Determine initial status
        if self.can_step_start(&step, &step_def)? {
            step.attr.status = StepStatus::Todo;
        } else {
            step.attr.status = StepStatus::Wait;
        }

        // For process steps, create iteration
        if step_def.is_process() {
            self.init_process_iteration(&mut step, &step_def)?;
        }

        // Write step file
        self.storage.write_step(&step)?;

        // Render description file if todo
        if step.status() == StepStatus::Todo {
            self.render_description_file(&step)?;
        }

        Ok(step)
    }

    /// Get parent FQID from a FQID
    fn get_parent_fqid(&self, fqid: &str) -> Option<String> {
        if fqid == "ROOT" {
            return None;
        }

        let parts: Vec<&str> = fqid.split('.').collect();
        if parts.len() <= 1 {
            Some("ROOT".to_string())
        } else {
            Some(parts[..parts.len() - 1].join("."))
        }
    }

    /// Validate that dependencies are satisfied
    fn validate_dependencies_for_step(&self, fqid: &str) -> Result<()> {
        let parent_fqid = self.get_parent_fqid(fqid);
        
        if let Some(parent_fqid) = parent_fqid {
            let parent_def = self.process_config.find_step_definition(&parent_fqid)
                .ok_or_else(|| GlowError::StepNotFound { fqid: parent_fqid.clone() })?;

            let step_id = fqid.split('.').last().unwrap_or(fqid);
            let blocking_deps: Vec<String> = parent_def.links.iter()
                .filter(|l| l.from == step_id && l.is_blocking())
                .filter_map(|l| {
                    let dep_fqid = if parent_fqid == "ROOT" {
                        l.to.clone()
                    } else {
                        format!("{}.{}", parent_fqid, l.to)
                    };
                    
                    // Check if dependency is done
                    if let Ok(dep_step) = self.storage.read_step(&dep_fqid) {
                        if dep_step.status() != StepStatus::Done {
                            return Some(l.to.clone());
                        }
                    } else {
                        return Some(l.to.clone());
                    }
                    None
                })
                .collect();

            if !blocking_deps.is_empty() {
                return Err(GlowError::BlockedByDependencies {
                    step_id: fqid.to_string(),
                    dependencies: blocking_deps,
                });
            }
        }

        Ok(())
    }

    /// Check if a step can start
    fn can_step_start(&self, step: &Step, def: &StepDefinition) -> Result<bool> {
        // Check required scope parameters
        for scope_ref in &def.scope {
            if scope_ref.is_required.unwrap_or(false) {
                if !step.scope.iter().any(|p| p.id == scope_ref.id && p.value.is_some()) {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    /// Initialize process iteration
    fn init_process_iteration(&mut self, step: &mut Step, def: &StepDefinition) -> Result<()> {
        // Create iteration folder
        let iteration_num = 1; // TODO: Track iteration numbers
        let _iteration_path = self.storage.create_iteration_folder(step.fqid(), iteration_num)?;

        // Create sub-steps
        for sub_def in &def.steps {
            let _sub_fqid = format!("{}.{}", step.fqid(), sub_def.id);
            let sub_step = Step::from_definition(sub_def, Some(step.fqid()));
            
            // Add to own_steps
            step.own_steps.push(StepRef {
                id: sub_def.id.clone(),
                status: StepStatus::Wait,
            });

            // Write sub-step file
            self.storage.write_step(&sub_step)?;
        }

        // Update sub-step statuses based on dependencies
        self.update_sub_step_statuses(step, def)?;

        Ok(())
    }

    /// Update sub-step statuses based on dependencies
    fn update_sub_step_statuses(&mut self, parent: &mut Step, def: &StepDefinition) -> Result<()> {
        let graph = LinkGraph::from_links(&def.links);
        let parent_fqid = parent.fqid().to_string();

        // First pass: collect status information and determine updates
        let updates: Vec<(usize, String)> = parent.own_steps.iter().enumerate()
            .filter_map(|(idx, own_step)| {
                let deps = graph.get_dependencies(&own_step.id);
                let all_deps_done = deps.iter().all(|dep_id| {
                    parent.own_steps.iter()
                        .find(|s| s.id == *dep_id)
                        .map(|s| s.status == StepStatus::Done)
                        .unwrap_or(false)
                });

                if deps.is_empty() || all_deps_done {
                    Some((idx, format!("{}.{}", parent_fqid, own_step.id)))
                } else {
                    None
                }
            })
            .collect();

        // Second pass: apply updates
        for (idx, sub_fqid) in updates {
            parent.own_steps[idx].status = StepStatus::Todo;
            
            // Update the actual step file
            if let Ok(mut sub_step) = self.storage.read_step(&sub_fqid) {
                sub_step.attr.status = StepStatus::Todo;
                self.storage.write_step(&sub_step)?;
            }
        }

        Ok(())
    }

    /// Render description file for a step
    fn render_description_file(&mut self, step: &Step) -> Result<()> {
        let content = self.template_engine.render_step(step, None)?;
        
        let folder_path = self.storage.step_files().step_folder_path(step.fqid());
        std::fs::create_dir_all(&folder_path)?;
        
        let desc_path = folder_path.join("description.md");
        
        // Extract body from rendered content and resolve references
        let body = self.extract_body_from_rendered(&content);
        let context = crate::template::renderer::TemplateContext::from_step(step);
        let resolved = self.template_engine.render_content_template(&body, &context)?;
        
        std::fs::write(&desc_path, resolved)?;
        
        Ok(())
    }

    /// Extract body from rendered template
    fn extract_body_from_rendered(&self, content: &str) -> String {
        if let Ok((_, body)) = crate::storage::parse_frontmatter(content) {
            body
        } else {
            content.to_string()
        }
    }

    /// Start a task step
    pub fn start_step(&mut self, fqid: &str) -> Result<Step> {
        let mut step = self.storage.read_step(fqid)?;

        // Validate state transition
        if step.status() != StepStatus::Todo {
            return Err(GlowError::InvalidStateTransition {
                step_id: fqid.to_string(),
                current: step.status().to_string(),
                target: StepStatus::InProgress.to_string(),
            });
        }

        // Update status
        step.attr.status = StepStatus::InProgress;

        // Re-render step file
        self.storage.write_step(&step)?;
        
        // Render description file
        self.render_description_file(&step)?;

        Ok(step)
    }

    /// Finish a task step
    pub fn finish_step(
        &mut self,
        fqid: &str,
        outputs: Vec<ParameterValue>,
        summary: Option<String>,
    ) -> Result<Step> {
        let mut step = self.storage.read_step(fqid)?;

        // Validate state transition
        if step.status() != StepStatus::InProgress {
            return Err(GlowError::InvalidStateTransition {
                step_id: fqid.to_string(),
                current: step.status().to_string(),
                target: StepStatus::Done.to_string(),
            });
        }

        // Set outputs
        for output in outputs {
            step.output.push(output);
        }

        // Update status
        step.attr.status = StepStatus::Done;

        // Write step file
        self.storage.write_step(&step)?;

        // Update summary if provided
        if let Some(summary_text) = summary {
            let folder_path = self.storage.step_files().step_folder_path(fqid);
            let summary_path = folder_path.join("summary.md");
            std::fs::write(&summary_path, format!("# Summary\n\n{}\n", summary_text))?;
        }

        // Update parent process
        self.update_parent_after_step_done(&step)?;

        Ok(step)
    }

    /// Update parent process after a step is done
    fn update_parent_after_step_done(&mut self, step: &Step) -> Result<()> {
        if let Some(parent_fqid) = self.get_parent_fqid(step.fqid()) {
            if parent_fqid == "ROOT" {
                // Update root process sub-step statuses
                // TODO: Implement root-level tracking
            } else {
                let parent_def = self.process_config.find_step_definition(&parent_fqid)
                    .ok_or_else(|| GlowError::StepNotFound { fqid: parent_fqid.clone() })?
                    .clone();

                let mut parent_step = self.storage.read_step(&parent_fqid)?;

                // Update own_steps status
                for own_step in &mut parent_step.own_steps {
                    if own_step.id == step.attr.id {
                        own_step.status = StepStatus::Done;
                    }
                }

                // Re-evaluate waiting steps
                self.update_sub_step_statuses(&mut parent_step, &parent_def)?;

                // Check if all sub-steps are done
                if parent_step.own_steps.iter().all(|s| s.status == StepStatus::Done) {
                    parent_step.attr.status = StepStatus::Done;
                }

                self.storage.write_step(&parent_step)?;
            }
        }

        Ok(())
    }

    /// Show step details
    pub fn show_step(&self, fqid: &str) -> Result<Step> {
        self.storage.read_step(fqid)
    }

    /// Get current project status tree
    pub fn get_status_tree(&self) -> Result<StatusTree> {
        let root_def = &self.process_config.root_process;
        self.build_status_tree(root_def, None)
    }

    /// Build status tree recursively
    fn build_status_tree(
        &self,
        def: &StepDefinition,
        parent_fqid: Option<&str>,
    ) -> Result<StatusTree> {
        let fqid = match parent_fqid {
            Some(parent) if parent != "ROOT" => format!("{}.{}", parent, def.id),
            Some(_) => def.id.clone(),
            None => "ROOT".to_string(),
        };

        let status = if let Ok(step) = self.storage.read_step(&fqid) {
            step.status()
        } else {
            StepStatus::Wait
        };

        let children: Vec<StatusTree> = def.steps.iter()
            .filter_map(|sub_def| {
                self.build_status_tree(sub_def, Some(&fqid)).ok()
            })
            .collect();

        Ok(StatusTree {
            id: def.id.clone(),
            fqid,
            purpose: def.purpose.clone(),
            status,
            children,
        })
    }

    /// Get recommended next actions
    pub fn get_next_actions(&self) -> Result<Vec<NextAction>> {
        let mut actions = Vec::new();
        let status_tree = self.get_status_tree()?;
        self.collect_next_actions(&status_tree, &mut actions);
        Ok(actions)
    }

    /// Collect next actions recursively
    fn collect_next_actions(&self, tree: &StatusTree, actions: &mut Vec<NextAction>) {
        match tree.status {
            StepStatus::Todo => {
                actions.push(NextAction {
                    fqid: tree.fqid.clone(),
                    action_type: ActionType::Start,
                    description: format!("Start: {}", tree.purpose.as_deref().unwrap_or(&tree.id)),
                });
            }
            StepStatus::InProgress => {
                // Check children for available work
                for child in &tree.children {
                    self.collect_next_actions(child, actions);
                }
            }
            _ => {
                for child in &tree.children {
                    self.collect_next_actions(child, actions);
                }
            }
        }
    }

    /// Get progress metrics
    pub fn get_progress(&self) -> Result<ProgressMetrics> {
        let status_tree = self.get_status_tree()?;
        let mut metrics = ProgressMetrics::default();
        self.collect_progress_metrics(&status_tree, &mut metrics);
        Ok(metrics)
    }

    /// Collect progress metrics recursively
    fn collect_progress_metrics(&self, tree: &StatusTree, metrics: &mut ProgressMetrics) {
        metrics.total += 1;
        match tree.status {
            StepStatus::Wait => metrics.wait += 1,
            StepStatus::Todo => metrics.todo += 1,
            StepStatus::InProgress => metrics.in_progress += 1,
            StepStatus::Done => metrics.done += 1,
        }

        for child in &tree.children {
            self.collect_progress_metrics(child, metrics);
        }
    }

    /// Validate context quality
    pub fn validate(&self, fqid: Option<&str>) -> Result<ValidationReport> {
        self.validator.validate_project(self, fqid)
    }
}

/// Status tree for displaying project state
#[derive(Debug, Clone)]
pub struct StatusTree {
    pub id: String,
    pub fqid: String,
    pub purpose: Option<String>,
    pub status: StepStatus,
    pub children: Vec<StatusTree>,
}

/// Next action recommendation
#[derive(Debug, Clone)]
pub struct NextAction {
    pub fqid: String,
    pub action_type: ActionType,
    pub description: String,
}

/// Action type
#[derive(Debug, Clone, Copy)]
pub enum ActionType {
    Init,
    Start,
    Finish,
    Review,
}

/// Progress metrics
#[derive(Debug, Clone, Default)]
pub struct ProgressMetrics {
    pub total: usize,
    pub wait: usize,
    pub todo: usize,
    pub in_progress: usize,
    pub done: usize,
}

impl ProgressMetrics {
    pub fn completion_percentage(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.done as f64 / self.total as f64) * 100.0
        }
    }
}

/// Validation report
#[derive(Debug, Clone, Default)]
pub struct ValidationReport {
    pub is_valid: bool,
    pub completeness: f64,
    pub issues: Vec<ValidationIssue>,
    pub warnings: Vec<String>,
}

/// Validation issue
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub fqid: String,
    pub issue_type: IssueType,
    pub message: String,
}

/// Issue type
#[derive(Debug, Clone)]
pub enum IssueType {
    MissingParameter,
    BrokenLink,
    InconsistentState,
    CircularDependency,
}
