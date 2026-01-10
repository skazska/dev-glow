//! MCP Resources
//!
//! Resources exposed by the glow-mcp server:
//! - glow://project/status - Overall project status
//! - glow://step/{fqid} - Specific step context
//! - glow://next - Recommended next actions
//! - glow://progress - Progress metrics

use std::sync::Arc;

use glow_core::engine::operations::{ActionType, ProcessEngine};
use serde_json::{json, Value};

use crate::protocol::{Resource, ResourceContent, RpcError};
use crate::ServerState;

/// List available resources
pub async fn list_resources(state: &Arc<ServerState>) -> Result<Value, RpcError> {
    let mut resources = vec![
        Resource {
            uri: "glow://project/status".to_string(),
            name: "Project Status".to_string(),
            description: "Current status of all steps in the development process".to_string(),
            mime_type: "application/json".to_string(),
        },
        Resource {
            uri: "glow://next".to_string(),
            name: "Next Actions".to_string(),
            description: "Recommended next actions based on current state".to_string(),
            mime_type: "application/json".to_string(),
        },
        Resource {
            uri: "glow://progress".to_string(),
            name: "Progress Metrics".to_string(),
            description: "Overall progress metrics".to_string(),
            mime_type: "application/json".to_string(),
        },
    ];

    // Add dynamic step resources if engine is loaded
    if let Err(_) = state.ensure_engine().await {
        return Ok(json!({ "resources": resources }));
    }

    let engine = state.engine.read().await;
    if let Some(ref engine) = *engine {
        if let Ok(status) = engine.get_status_tree() {
            add_step_resources(&mut resources, &status, "".to_string());
        }
    }

    Ok(json!({ "resources": resources }))
}

fn add_step_resources(
    resources: &mut Vec<Resource>,
    tree: &glow_core::engine::operations::StatusTree,
    prefix: String,
) {
    let fqid = if prefix.is_empty() {
        tree.id.clone()
    } else {
        format!("{}.{}", prefix, tree.id)
    };

    resources.push(Resource {
        uri: format!("glow://step/{}", fqid),
        name: format!("Step: {}", tree.id),
        description: tree.purpose.clone().unwrap_or_default(),
        mime_type: "application/json".to_string(),
    });

    for child in &tree.children {
        add_step_resources(resources, child, fqid.clone());
    }
}

/// Read a resource
pub async fn read_resource(state: &Arc<ServerState>, params: &Value) -> Result<Value, RpcError> {
    let uri = params
        .get("uri")
        .and_then(|v| v.as_str())
        .ok_or_else(|| RpcError {
            code: -32602,
            message: "Missing 'uri' parameter".to_string(),
            data: None,
        })?;

    // Ensure engine is loaded
    state.ensure_engine().await.map_err(|e| RpcError {
        code: -32603,
        message: format!("Failed to load project: {}", e),
        data: None,
    })?;

    let engine = state.engine.read().await;
    let engine = engine.as_ref().ok_or_else(|| RpcError {
        code: -32603,
        message: "Engine not initialized".to_string(),
        data: None,
    })?;

    let content = if uri == "glow://project/status" {
        read_project_status(engine)?
    } else if uri == "glow://next" {
        read_next_actions(engine)?
    } else if uri == "glow://progress" {
        read_progress(engine)?
    } else if uri.starts_with("glow://step/") {
        let fqid = uri.strip_prefix("glow://step/").unwrap();
        read_step(engine, fqid)?
    } else {
        return Err(RpcError {
            code: -32602,
            message: format!("Unknown resource: {}", uri),
            data: None,
        });
    };

    Ok(json!({ "contents": [content] }))
}

fn read_project_status(engine: &ProcessEngine) -> Result<ResourceContent, RpcError> {
    let status = engine.get_status_tree().map_err(|e| RpcError {
        code: -32603,
        message: e.to_string(),
        data: None,
    })?;

    let json = serde_json::to_string_pretty(&status_tree_to_json(&status)).map_err(|e| RpcError {
        code: -32603,
        message: e.to_string(),
        data: None,
    })?;

    Ok(ResourceContent {
        uri: "glow://project/status".to_string(),
        mime_type: "application/json".to_string(),
        text: json,
    })
}

fn status_tree_to_json(tree: &glow_core::engine::operations::StatusTree) -> Value {
    json!({
        "id": tree.id,
        "status": format!("{:?}", tree.status),
        "purpose": tree.purpose,
        "children": tree.children.iter().map(status_tree_to_json).collect::<Vec<_>>()
    })
}

fn read_next_actions(engine: &ProcessEngine) -> Result<ResourceContent, RpcError> {
    let actions = engine.get_next_actions().map_err(|e| RpcError {
        code: -32603,
        message: e.to_string(),
        data: None,
    })?;

    let actions_json: Vec<Value> = actions
        .iter()
        .map(|a| {
            json!({
                "fqid": a.fqid,
                "action": match a.action_type {
                    ActionType::Init => "init",
                    ActionType::Start => "start",
                    ActionType::Review => "review",
                    ActionType::Finish => "finish",
                },
                "description": a.description
            })
        })
        .collect();

    let json = serde_json::to_string_pretty(&actions_json).map_err(|e| RpcError {
        code: -32603,
        message: e.to_string(),
        data: None,
    })?;

    Ok(ResourceContent {
        uri: "glow://next".to_string(),
        mime_type: "application/json".to_string(),
        text: json,
    })
}

fn read_progress(engine: &ProcessEngine) -> Result<ResourceContent, RpcError> {
    let progress = engine.get_progress().map_err(|e| RpcError {
        code: -32603,
        message: e.to_string(),
        data: None,
    })?;

    let json = serde_json::to_string_pretty(&json!({
        "total": progress.total,
        "wait": progress.wait,
        "todo": progress.todo,
        "in_progress": progress.in_progress,
        "done": progress.done,
        "completion_percentage": progress.completion_percentage()
    }))
    .map_err(|e| RpcError {
        code: -32603,
        message: e.to_string(),
        data: None,
    })?;

    Ok(ResourceContent {
        uri: "glow://progress".to_string(),
        mime_type: "application/json".to_string(),
        text: json,
    })
}

fn read_step(engine: &ProcessEngine, fqid: &str) -> Result<ResourceContent, RpcError> {
    let step = engine.show_step(fqid).map_err(|e| RpcError {
        code: -32603,
        message: e.to_string(),
        data: None,
    })?;

    let json = serde_json::to_string_pretty(&json!({
        "id": step.attr.id,
        "fqid": step.fqid(),
        "status": format!("{:?}", step.status()),
        "classification": step.attr.classification,
        "purpose": step.attr.purpose,
        "expectations": step.attr.expectations,
        "input": step.input.iter().map(|p| {
            json!({ "id": p.id, "value": p.value })
        }).collect::<Vec<_>>(),
        "scope": step.scope.iter().map(|p| {
            json!({ "id": p.id, "value": p.value })
        }).collect::<Vec<_>>(),
        "output": step.output.iter().map(|p| {
            json!({ "id": p.id, "value": p.value })
        }).collect::<Vec<_>>(),
        "sub_steps": step.own_steps.iter().map(|s| {
            json!({ "id": s.id, "status": format!("{:?}", s.status) })
        }).collect::<Vec<_>>()
    }))
    .map_err(|e| RpcError {
        code: -32603,
        message: e.to_string(),
        data: None,
    })?;

    Ok(ResourceContent {
        uri: format!("glow://step/{}", fqid),
        mime_type: "application/json".to_string(),
        text: json,
    })
}
