//! Context building
//!
//! Builds execution context for steps.

use crate::model::{ParameterValue, ParentRef, Step};

/// Context builder for step execution
#[derive(Debug, Default)]
pub struct ContextBuilder {
    inputs: Vec<ParameterValue>,
    scope: Vec<ParameterValue>,
    parents: Vec<ParentRef>,
    links: Vec<LinkContext>,
}

#[derive(Debug, Clone)]
pub struct LinkContext {
    pub step_id: String,
    pub inputs: Vec<ParameterValue>,
    pub outputs: Vec<ParameterValue>,
}

impl ContextBuilder {
    /// Create a new context builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Add input parameter
    pub fn with_input(mut self, param: ParameterValue) -> Self {
        self.inputs.push(param);
        self
    }

    /// Add scope parameter
    pub fn with_scope(mut self, param: ParameterValue) -> Self {
        self.scope.push(param);
        self
    }

    /// Add parent context
    pub fn with_parent(mut self, parent: ParentRef) -> Self {
        self.parents.push(parent);
        self
    }

    /// Add linked step context
    pub fn with_link(mut self, link: LinkContext) -> Self {
        self.links.push(link);
        self
    }

    /// Build context from parent step
    pub fn from_parent(parent: &Step) -> Self {
        let mut builder = Self::new();

        // Add parent's scope to context
        for param in &parent.scope {
            builder.scope.push(param.clone());
        }

        // Create parent reference
        let parent_ref = ParentRef {
            id: parent.attr.id.clone(),
            steps: parent.own_steps.clone(),
        };
        builder.parents.push(parent_ref);

        builder
    }

    /// Build context for a step
    pub fn build_for_step(self, step: &mut Step) {
        step.input = self.inputs;
        step.scope.extend(self.scope);
        step.parent = self.parents;

        // Add link refs
        for link in self.links {
            step.links.push(crate::model::LinkRef {
                step_id: link.step_id,
                link_type: "dependency".to_string(),
                step_status: None,
            });
        }
    }

    /// Resolve parameter mappings
    pub fn resolve_mappings(&self, mapping: &str) -> Option<serde_json::Value> {
        // Parse mapping expression like "links.REQ-001.output.ACCEPTANCE_CRITERIA"
        let parts: Vec<&str> = mapping.split('.').collect();
        if parts.is_empty() {
            return None;
        }

        match parts[0] {
            "input" if parts.len() >= 2 => {
                self.inputs.iter()
                    .find(|p| p.id == parts[1])
                    .and_then(|p| p.value.clone())
            }
            "scope" if parts.len() >= 2 => {
                self.scope.iter()
                    .find(|p| p.id == parts[1])
                    .and_then(|p| p.value.clone())
            }
            "links" if parts.len() >= 4 => {
                let step_id = parts[1];
                let param_type = parts[2];
                let param_id = parts[3];

                self.links.iter()
                    .find(|l| l.step_id == step_id)
                    .and_then(|l| {
                        let params = match param_type {
                            "input" => &l.inputs,
                            "output" => &l.outputs,
                            _ => return None,
                        };
                        params.iter()
                            .find(|p| p.id == param_id)
                            .and_then(|p| p.value.clone())
                    })
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_builder() {
        let builder = ContextBuilder::new()
            .with_input(ParameterValue::new("REQ", serde_json::json!("REQ-001")))
            .with_scope(ParameterValue::new("DESC", serde_json::json!("./desc.md")));

        assert_eq!(builder.inputs.len(), 1);
        assert_eq!(builder.scope.len(), 1);
    }

    #[test]
    fn test_resolve_mappings() {
        let builder = ContextBuilder::new()
            .with_input(ParameterValue::new("REQ", serde_json::json!("REQ-001")))
            .with_scope(ParameterValue::new("DESC", serde_json::json!("description")));

        assert_eq!(
            builder.resolve_mappings("input.REQ"),
            Some(serde_json::json!("REQ-001"))
        );

        assert_eq!(
            builder.resolve_mappings("scope.DESC"),
            Some(serde_json::json!("description"))
        );

        assert_eq!(builder.resolve_mappings("unknown.PARAM"), None);
    }
}
