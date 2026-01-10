//! MCP Tools
//!
//! Tools exposed by the glow-mcp server:
//! - glow_status - Get current project status
//! - glow_next - Get recommended next actions
//! - glow_show_step - Show step details
//! - glow_start_step - Start a step
//! - glow_finish_step - Finish a step
//! - glow_progress - Get progress metrics
//! - glow_validate - Validate context quality

use std::sync::Arc;

use glow_core::engine::operations::ActionType;
use glow_core::model::ParameterValue;
use serde_json::{json, Value};

use crate::protocol::{RpcError, Tool, ToolResult};
use crate::ServerState;

/// List available tools
pub async fn list_tools() -> Result<Value, RpcError> {
    let tools = vec![
        Tool {
            name: "glow_status".to_string(),
            description: "Get current project status tree showing all steps and their states"
                .to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "glow_next".to_string(),
            description: "Get recommended next actions based on current project state".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "glow_show_step".to_string(),
            description: "Show detailed information about a specific step including context"
                .to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "fqid": {
                        "type": "string",
                        "description": "Fully qualified step ID (e.g., ROOT, FEAT-001, FEAT-001.REQ-001)"
                    }
                },
                "required": ["fqid"]
            }),
        },
        Tool {
            name: "glow_start_step".to_string(),
            description: "Start working on a step, transitioning it to in-progress state"
                .to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "fqid": {
                        "type": "string",
                        "description": "Step FQID to start"
                    }
                },
                "required": ["fqid"]
            }),
        },
        Tool {
            name: "glow_finish_step".to_string(),
            description: "Complete a step, providing output and summary".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "fqid": {
                        "type": "string",
                        "description": "Step FQID to finish"
                    },
                    "summary": {
                        "type": "string",
                        "description": "Summary of work completed"
                    },
                    "outputs": {
                        "type": "object",
                        "description": "Output parameter values as key-value pairs"
                    }
                },
                "required": ["fqid"]
            }),
        },
        Tool {
            name: "glow_progress".to_string(),
            description: "Get progress metrics for the project".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "glow_validate".to_string(),
            description: "Validate context quality and check for issues".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "fqid": {
                        "type": "string",
                        "description": "Optional step FQID to validate (validates all if omitted)"
                    }
                }
            }),
        },
    ];

    Ok(json!({ "tools": tools }))
}

/// Call a tool
pub async fn call_tool(state: &Arc<ServerState>, params: &Value) -> Result<Value, RpcError> {
    let name = params
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| RpcError {
            code: -32602,
            message: "Missing 'name' parameter".to_string(),
            data: None,
        })?;

    let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

    // Ensure engine is loaded
    state.ensure_engine().await.map_err(|e| RpcError {
        code: -32603,
        message: format!("Failed to load project: {}", e),
        data: None,
    })?;

    let result = match name {
        "glow_status" => tool_status(state).await,
        "glow_next" => tool_next(state).await,
        "glow_show_step" => tool_show_step(state, &arguments).await,
        "glow_start_step" => tool_start_step(state, &arguments).await,
        "glow_finish_step" => tool_finish_step(state, &arguments).await,
        "glow_progress" => tool_progress(state).await,
        "glow_validate" => tool_validate(state, &arguments).await,
        _ => ToolResult::error(format!("Unknown tool: {}", name)),
    };

    serde_json::to_value(result).map_err(|e| RpcError {
        code: -32603,
        message: e.to_string(),
        data: None,
    })
}

async fn tool_status(state: &Arc<ServerState>) -> ToolResult {
    let engine = state.engine.read().await;
    let engine = match engine.as_ref() {
        Some(e) => e,
        None => return ToolResult::error("Engine not initialized".to_string()),
    };

    match engine.get_status_tree() {
        Ok(status) => {
            let text = format_status_tree(&status, "");
            ToolResult::text(text)
        }
        Err(e) => ToolResult::error(e.to_string()),
    }
}

fn format_status_tree(tree: &glow_core::engine::operations::StatusTree, indent: &str) -> String {
    let status_icon = match tree.status {
        glow_core::model::StepStatus::Wait => "○",
        glow_core::model::StepStatus::Todo => "◐",
        glow_core::model::StepStatus::InProgress => "◑",
        glow_core::model::StepStatus::Done => "●",
    };

    let purpose = tree.purpose.as_deref().unwrap_or("");
    let mut result = format!(
        "{}{} {} - {}\n",
        indent, status_icon, tree.id, purpose
    );

    let child_indent = format!("{}  ", indent);
    for child in &tree.children {
        result.push_str(&format_status_tree(child, &child_indent));
    }

    result
}

async fn tool_next(state: &Arc<ServerState>) -> ToolResult {
    let engine = state.engine.read().await;
    let engine = match engine.as_ref() {
        Some(e) => e,
        None => return ToolResult::error("Engine not initialized".to_string()),
    };

    match engine.get_next_actions() {
        Ok(actions) => {
            if actions.is_empty() {
                return ToolResult::text("No pending actions. All steps are complete or waiting.".to_string());
            }

            let mut text = "Recommended Next Actions:\n\n".to_string();
            for action in &actions {
                let cmd = match action.action_type {
                    ActionType::Init => format!("glow init {}", action.fqid),
                    ActionType::Start => format!("glow start {}", action.fqid),
                    ActionType::Review => format!("glow show {}", action.fqid),
                    ActionType::Finish => format!("glow finish {}", action.fqid),
                };
                text.push_str(&format!("→ {}\n  Command: {}\n\n", action.description, cmd));
            }
            ToolResult::text(text)
        }
        Err(e) => ToolResult::error(e.to_string()),
    }
}

async fn tool_show_step(state: &Arc<ServerState>, args: &Value) -> ToolResult {
    let fqid = match args.get("fqid").and_then(|v| v.as_str()) {
        Some(f) => f,
        None => return ToolResult::error("Missing 'fqid' argument".to_string()),
    };

    let engine = state.engine.read().await;
    let engine = match engine.as_ref() {
        Some(e) => e,
        None => return ToolResult::error("Engine not initialized".to_string()),
    };

    match engine.show_step(fqid) {
        Ok(step) => {
            let mut text = format!("Step: {} ({})\n", step.attr.id, fqid);
            text.push_str(&format!("Status: {:?}\n", step.status()));

            if let Some(purpose) = &step.attr.purpose {
                text.push_str(&format!("\nPurpose:\n{}\n", purpose));
            }

            if let Some(expectations) = &step.attr.expectations {
                text.push_str(&format!("\nExpectations:\n{}\n", expectations));
            }

            if !step.input.is_empty() {
                text.push_str("\nInputs:\n");
                for param in &step.input {
                    let value = param
                        .value
                        .as_ref()
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "(not set)".to_string());
                    text.push_str(&format!("  {}: {}\n", param.id, value));
                }
            }

            if !step.scope.is_empty() {
                text.push_str("\nScope:\n");
                for param in &step.scope {
                    let value = param
                        .value
                        .as_ref()
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "(not set)".to_string());
                    text.push_str(&format!("  {}: {}\n", param.id, value));
                }
            }

            if !step.output.is_empty() {
                text.push_str("\nOutputs:\n");
                for param in &step.output {
                    let value = param
                        .value
                        .as_ref()
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "(not set)".to_string());
                    text.push_str(&format!("  {}: {}\n", param.id, value));
                }
            }

            if !step.own_steps.is_empty() {
                text.push_str("\nSub-steps:\n");
                for sub in &step.own_steps {
                    text.push_str(&format!("  {:?} {}\n", sub.status, sub.id));
                }
            }

            ToolResult::text(text)
        }
        Err(e) => ToolResult::error(e.to_string()),
    }
}

async fn tool_start_step(state: &Arc<ServerState>, args: &Value) -> ToolResult {
    let fqid = match args.get("fqid").and_then(|v| v.as_str()) {
        Some(f) => f,
        None => return ToolResult::error("Missing 'fqid' argument".to_string()),
    };

    let mut engine = state.engine.write().await;
    let engine = match engine.as_mut() {
        Some(e) => e,
        None => return ToolResult::error("Engine not initialized".to_string()),
    };

    match engine.start_step(fqid) {
        Ok(step) => {
            let mut text = format!("Started step: {}\n\n", fqid);

            if let Some(purpose) = &step.attr.purpose {
                text.push_str(&format!("Purpose:\n{}\n\n", purpose));
            }

            if let Some(expectations) = &step.attr.expectations {
                text.push_str(&format!("Expectations:\n{}\n", expectations));
            }

            ToolResult::text(text)
        }
        Err(e) => ToolResult::error(e.to_string()),
    }
}

async fn tool_finish_step(state: &Arc<ServerState>, args: &Value) -> ToolResult {
    let fqid = match args.get("fqid").and_then(|v| v.as_str()) {
        Some(f) => f,
        None => return ToolResult::error("Missing 'fqid' argument".to_string()),
    };

    let summary = args.get("summary").and_then(|v| v.as_str()).map(String::from);

    // Parse output parameters
    let outputs = if let Some(obj) = args.get("outputs").and_then(|v| v.as_object()) {
        obj.iter()
            .map(|(k, v)| ParameterValue::new(k, v.clone()))
            .collect()
    } else {
        Vec::new()
    };

    let mut engine = state.engine.write().await;
    let engine = match engine.as_mut() {
        Some(e) => e,
        None => return ToolResult::error("Engine not initialized".to_string()),
    };

    match engine.finish_step(fqid, outputs, summary) {
        Ok(_step) => {
            let mut text = format!("Completed step: {}\n\n", fqid);

            // Show next actions
            if let Ok(actions) = engine.get_next_actions() {
                if !actions.is_empty() {
                    text.push_str("Next available actions:\n");
                    for action in actions.iter().take(3) {
                        text.push_str(&format!("  → {}\n", action.description));
                    }
                }
            }

            ToolResult::text(text)
        }
        Err(e) => ToolResult::error(e.to_string()),
    }
}

async fn tool_progress(state: &Arc<ServerState>) -> ToolResult {
    let engine = state.engine.read().await;
    let engine = match engine.as_ref() {
        Some(e) => e,
        None => return ToolResult::error("Engine not initialized".to_string()),
    };

    match engine.get_progress() {
        Ok(progress) => {
            let pct = progress.completion_percentage();
            let mut text = "Progress Report\n".to_string();
            text.push_str(&format!("═══════════════════════\n\n"));
            text.push_str(&format!("Completion: {:.1}%\n\n", pct));
            text.push_str(&format!("Total Steps: {}\n", progress.total));
            text.push_str(&format!("  ● Done: {}\n", progress.done));
            text.push_str(&format!("  ◑ In Progress: {}\n", progress.in_progress));
            text.push_str(&format!("  ◐ Todo: {}\n", progress.todo));
            text.push_str(&format!("  ○ Wait: {}\n", progress.wait));

            ToolResult::text(text)
        }
        Err(e) => ToolResult::error(e.to_string()),
    }
}

async fn tool_validate(state: &Arc<ServerState>, args: &Value) -> ToolResult {
    let fqid = args.get("fqid").and_then(|v| v.as_str());

    let engine = state.engine.read().await;
    let engine = match engine.as_ref() {
        Some(e) => e,
        None => return ToolResult::error("Engine not initialized".to_string()),
    };

    match engine.validate(fqid) {
        Ok(report) => {
            let mut text = "Context Quality Report\n".to_string();
            text.push_str(&format!("═══════════════════════\n\n"));

            if report.is_valid {
                text.push_str(&format!("✓ Completeness: {:.1}%\n", report.completeness));
                text.push_str("✓ Consistency: Valid\n");
                text.push_str("✓ All checks passed\n");
            } else {
                text.push_str(&format!("Completeness: {:.1}%\n", report.completeness));

                if !report.issues.is_empty() {
                    text.push_str("\nIssues:\n");
                    for issue in &report.issues {
                        text.push_str(&format!("  ✗ {}: {}\n", issue.fqid, issue.message));
                    }
                }

                if !report.warnings.is_empty() {
                    text.push_str("\nWarnings:\n");
                    for warning in &report.warnings {
                        text.push_str(&format!("  ⚠ {}\n", warning));
                    }
                }
            }

            ToolResult::text(text)
        }
        Err(e) => ToolResult::error(e.to_string()),
    }
}
