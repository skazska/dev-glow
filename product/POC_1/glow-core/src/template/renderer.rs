//! Template renderer
//!
//! Renders step data files using Handlebars templates.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use handlebars::Handlebars;
use serde::Serialize;

use crate::error::{GlowError, Result};
use crate::model::{ParameterValue, ParentRef, Step, StepRef, LinkRef, StepStatus};

use super::helpers::register_helpers;

/// Template rendering context
#[derive(Debug, Clone, Serialize)]
pub struct TemplateContext {
    /// Step attributes
    pub attr: AttrContext,
    /// Input parameters
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<ParamContext>,
    /// Scope parameters
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub scope: Vec<ParamContext>,
    /// Output parameters
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub outputs: Vec<ParamContext>,
    /// Parent steps in stack
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parents: Vec<ParentContext>,
    /// Own sub-steps (for process)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub own_steps: Vec<StepRefContext>,
    /// Linked steps
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<LinkContext>,
}

/// Step attributes context
#[derive(Debug, Clone, Serialize)]
pub struct AttrContext {
    pub id: String,
    pub fqid: String,
    pub classification: String,
    pub purpose: String,
    pub expectations: String,
    pub status: String,
}

/// Parameter context
#[derive(Debug, Clone, Serialize)]
pub struct ParamContext {
    pub id: String,
    pub value: serde_json::Value,
}

/// Parent step context
#[derive(Debug, Clone, Serialize)]
pub struct ParentContext {
    pub id: String,
    pub steps: Vec<StepRefContext>,
}

/// Step reference context
#[derive(Debug, Clone, Serialize)]
pub struct StepRefContext {
    pub id: String,
    pub status: String,
}

/// Link context
#[derive(Debug, Clone, Serialize)]
pub struct LinkContext {
    pub step_id: String,
    pub link_type: String,
    pub step_status: String,
}

impl TemplateContext {
    /// Create context from a step
    pub fn from_step(step: &Step) -> Self {
        Self {
            attr: AttrContext {
                id: step.attr.id.clone(),
                fqid: step.attr.fqid.clone().unwrap_or_default(),
                classification: step.attr.classification.clone().unwrap_or_default(),
                purpose: step.attr.purpose.clone().unwrap_or_default(),
                expectations: step.attr.expectations.clone().unwrap_or_default(),
                status: step.attr.status.to_string(),
            },
            inputs: step
                .input
                .iter()
                .map(|p| ParamContext {
                    id: p.id.clone(),
                    value: p.value.clone().unwrap_or(serde_json::Value::Null),
                })
                .collect(),
            scope: step
                .scope
                .iter()
                .map(|p| ParamContext {
                    id: p.id.clone(),
                    value: p.value.clone().unwrap_or(serde_json::Value::Null),
                })
                .collect(),
            outputs: step
                .output
                .iter()
                .map(|p| ParamContext {
                    id: p.id.clone(),
                    value: p.value.clone().unwrap_or(serde_json::Value::Null),
                })
                .collect(),
            parents: step
                .parent
                .iter()
                .map(|p| ParentContext {
                    id: p.id.clone(),
                    steps: p
                        .steps
                        .iter()
                        .map(|s| StepRefContext {
                            id: s.id.clone(),
                            status: s.status.to_string(),
                        })
                        .collect(),
                })
                .collect(),
            own_steps: step
                .own_steps
                .iter()
                .map(|s| StepRefContext {
                    id: s.id.clone(),
                    status: s.status.to_string(),
                })
                .collect(),
            links: step
                .links
                .iter()
                .map(|l| LinkContext {
                    step_id: l.step_id.clone(),
                    link_type: l.link_type.clone(),
                    step_status: l.step_status.map(|s| s.to_string()).unwrap_or_default(),
                })
                .collect(),
        }
    }
}

/// Template engine using Handlebars
pub struct TemplateEngine {
    /// Handlebars instance
    handlebars: Handlebars<'static>,
    /// Templates directory
    templates_dir: PathBuf,
    /// Loaded templates cache
    loaded_templates: HashMap<String, bool>,
}

impl TemplateEngine {
    /// Create a new template engine
    pub fn new(templates_dir: PathBuf) -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(false); // Allow missing variables
        register_helpers(&mut handlebars);

        // Register default template
        handlebars
            .register_template_string("any-step.md", super::DEFAULT_TEMPLATE)
            .expect("Failed to register default template");

        Self {
            handlebars,
            templates_dir,
            loaded_templates: HashMap::new(),
        }
    }

    /// Load a template from file
    pub fn load_template(&mut self, name: &str) -> Result<()> {
        if self.loaded_templates.contains_key(name) {
            return Ok(());
        }

        let template_path = self.templates_dir.join(name);
        if template_path.exists() {
            let content = std::fs::read_to_string(&template_path).map_err(|e| {
                GlowError::FileReadError {
                    path: template_path.clone(),
                    source: e,
                }
            })?;

            self.handlebars
                .register_template_string(name, &content)
                .map_err(|e| GlowError::TemplateError {
                    message: format!("Failed to parse template {}: {}", name, e),
                    source: Some(Box::new(e)),
                })?;

            self.loaded_templates.insert(name.to_string(), true);
        }

        Ok(())
    }

    /// Get the template name for a step based on classification
    pub fn get_template_for_step(&self, step: &Step, default: &str) -> String {
        // TODO: Implement classification-based template selection
        // For now, use default template
        default.to_string()
    }

    /// Render a step to a data file
    pub fn render_step(&mut self, step: &Step, template_name: Option<&str>) -> Result<String> {
        let template = template_name.unwrap_or("any-step.md");

        // Try to load template if not default
        if template != "any-step.md" {
            self.load_template(template)?;
        }

        let context = TemplateContext::from_step(step);

        // Check if template exists
        if !self.handlebars.has_template(template) {
            // Fall back to default
            return self.render_with_template("any-step.md", &context);
        }

        self.render_with_template(template, &context)
    }

    /// Render with a specific template
    fn render_with_template(&self, template: &str, context: &TemplateContext) -> Result<String> {
        self.handlebars
            .render(template, context)
            .map_err(|e| GlowError::TemplateError {
                message: format!("Failed to render template {}: {}", template, e),
                source: Some(Box::new(e)),
            })
    }

    /// Render content template (description file)
    /// Replaces parameter references in content with actual values
    pub fn render_content_template(&self, content: &str, context: &TemplateContext) -> Result<String> {
        // Create a temporary template
        let mut hb = Handlebars::new();
        register_helpers(&mut hb);

        // Convert parameter references to Handlebars format
        let template_content = self.convert_refs_to_handlebars(content);

        hb.register_template_string("content", &template_content)
            .map_err(|e| GlowError::TemplateError {
                message: format!("Failed to parse content template: {}", e),
                source: Some(Box::new(e)),
            })?;

        hb.render("content", context)
            .map_err(|e| GlowError::TemplateError {
                message: format!("Failed to render content template: {}", e),
                source: Some(Box::new(e)),
            })
    }

    /// Convert parameter references like `input.PARAM_NAME` to Handlebars format
    fn convert_refs_to_handlebars(&self, content: &str) -> String {
        // Simple regex-based conversion
        // Matches patterns like input.PARAM, scope.PARAM, parent.STEP.scope.PARAM, etc.
        let re = regex::Regex::new(
            r"(?P<prefix>input|scope|output|parent|own_steps|links)\.(?P<rest>[A-Za-z0-9_.\[\]]+)"
        ).unwrap();

        re.replace_all(content, |caps: &regex::Captures| {
            format!("{{{{{}_{}}}}}", &caps["prefix"], &caps["rest"].replace('.', "_"))
        }).to_string()
    }

    /// Get templates directory
    pub fn templates_dir(&self) -> &Path {
        &self.templates_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{StepAttributes, StepStatus};
    use tempfile::tempdir;

    fn create_test_step() -> Step {
        Step {
            attr: StepAttributes {
                id: "FEAT-001".to_string(),
                fqid: Some("FEAT-001".to_string()),
                classification: Some("Feature,Core,Must".to_string()),
                purpose: Some("Test feature".to_string()),
                expectations: Some("Feature works".to_string()),
                status: StepStatus::Todo,
            },
            input: vec![ParameterValue::new("REQUIREMENT", serde_json::json!("REQ-001"))],
            scope: vec![ParameterValue::new("DESCRIPTION", serde_json::json!("./desc.md"))],
            output: Vec::new(),
            parent: Vec::new(),
            own_steps: Vec::new(),
            links: Vec::new(),
        }
    }

    #[test]
    fn test_render_default_template() {
        let temp = tempdir().unwrap();
        let mut engine = TemplateEngine::new(temp.path().to_path_buf());
        let step = create_test_step();

        let result = engine.render_step(&step, None).unwrap();

        assert!(result.contains("id: FEAT-001"));
        assert!(result.contains("status: todo"));
        assert!(result.contains("Feature,Core,Must"));
    }

    #[test]
    fn test_template_context_from_step() {
        let step = create_test_step();
        let context = TemplateContext::from_step(&step);

        assert_eq!(context.attr.id, "FEAT-001");
        assert_eq!(context.attr.status, "todo");
        assert_eq!(context.inputs.len(), 1);
        assert_eq!(context.inputs[0].id, "REQUIREMENT");
    }
}
