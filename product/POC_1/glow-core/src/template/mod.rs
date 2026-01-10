//! Templating engine
//!
//! Handles rendering of step data files and descriptions using Handlebars.

mod helpers;
pub mod renderer;
mod subset;

pub use renderer::{TemplateContext, TemplateEngine};
pub use subset::SubsetPicker;

/// Default template for any step
pub const DEFAULT_TEMPLATE: &str = r#"---
attr:
  id: {{attr.id}}
  fqid: {{attr.fqid}}
  classification: {{attr.classification}}
  purpose: {{attr.purpose}}
  expectations: {{attr.expectations}}
  status: {{attr.status}}
{{#if inputs}}
input:
{{#each inputs}}
  - id: {{this.id}}
    value: {{this.value}}
{{/each}}
{{/if}}
{{#if scope}}
scope:
{{#each scope}}
  - id: {{this.id}}
    value: {{this.value}}
{{/each}}
{{/if}}
{{#if outputs}}
output:
{{#each outputs}}
  - id: {{this.id}}
    value: {{this.value}}
{{/each}}
{{/if}}
{{#if parents}}
parent:
{{#each parents}}
  - id: {{this.id}}
{{#if this.steps}}
    steps:
{{#each this.steps}}
      - id: {{this.id}}
        status: {{this.status}}
{{/each}}
{{/if}}
{{/each}}
{{/if}}
{{#if own_steps}}
own_steps:
{{#each own_steps}}
  - id: {{this.id}}
    status: {{this.status}}
{{/each}}
{{/if}}
{{#if links}}
links:
{{#each links}}
  - step_id: {{this.step_id}}
    link_type: {{this.link_type}}
    step_status: {{this.step_status}}
{{/each}}
{{/if}}
---

# {{attr.id}}

{{#if attr.purpose}}
**Purpose:** {{attr.purpose}}

{{/if}}
{{#if attr.expectations}}
**Expectations:** {{attr.expectations}}

{{/if}}
## Description

TODO: Add step description.
"#;
