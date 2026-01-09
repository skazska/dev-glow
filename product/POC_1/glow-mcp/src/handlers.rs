//! Request handlers for MCP protocol

use std::sync::Arc;

use serde_json::{json, Value};

use crate::protocol::{
    InitializeResult, JsonRpcRequest, JsonRpcResponse, PromptCapabilities, ResourceCapabilities,
    RpcError, ServerCapabilities, ServerInfo, ToolCapabilities,
};
use crate::resources;
use crate::tools;
use crate::ServerState;
use crate::PROTOCOL_VERSION;

/// Handle incoming MCP request
pub async fn handle_request(state: &Arc<ServerState>, request: JsonRpcRequest) -> JsonRpcResponse {
    tracing::debug!("Handling method: {}", request.method);

    let result = match request.method.as_str() {
        "initialize" => handle_initialize(&request.params).await,
        "initialized" => Ok(json!({})),
        "ping" => Ok(json!({})),

        // Resources
        "resources/list" => resources::list_resources(state).await,
        "resources/read" => resources::read_resource(state, &request.params).await,

        // Tools
        "tools/list" => tools::list_tools().await,
        "tools/call" => tools::call_tool(state, &request.params).await,

        // Prompts
        "prompts/list" => handle_list_prompts().await,
        "prompts/get" => handle_get_prompt(&request.params).await,

        _ => Err(RpcError {
            code: -32601,
            message: format!("Method not found: {}", request.method),
            data: None,
        }),
    };

    match result {
        Ok(value) => JsonRpcResponse::success(request.id, value),
        Err(error) => JsonRpcResponse::error(request.id, error),
    }
}

/// Handle initialize request
async fn handle_initialize(_params: &Value) -> Result<Value, RpcError> {
    let result = InitializeResult {
        protocol_version: PROTOCOL_VERSION.to_string(),
        capabilities: ServerCapabilities {
            resources: ResourceCapabilities {
                subscribe: false,
                list_changed: false,
            },
            tools: ToolCapabilities { list_changed: false },
            prompts: PromptCapabilities { list_changed: false },
        },
        server_info: ServerInfo {
            name: "glow-mcp".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        },
    };

    serde_json::to_value(result).map_err(|e| RpcError {
        code: -32603,
        message: e.to_string(),
        data: None,
    })
}

/// Handle list prompts
async fn handle_list_prompts() -> Result<Value, RpcError> {
    use crate::protocol::Prompt;

    let prompts = vec![
        Prompt {
            name: "glow_guide_step".to_string(),
            description: "Get AI guidance for a specific development step".to_string(),
            arguments: Some(vec![crate::protocol::PromptArgument {
                name: "fqid".to_string(),
                description: "Step FQID to get guidance for".to_string(),
                required: true,
            }]),
        },
        Prompt {
            name: "glow_validate_output".to_string(),
            description: "Validate step output before finishing".to_string(),
            arguments: Some(vec![crate::protocol::PromptArgument {
                name: "fqid".to_string(),
                description: "Step FQID to validate".to_string(),
                required: true,
            }]),
        },
    ];

    Ok(json!({ "prompts": prompts }))
}

/// Handle get prompt
async fn handle_get_prompt(params: &Value) -> Result<Value, RpcError> {
    use crate::protocol::{PromptContent, PromptMessage};

    let name = params
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| RpcError {
            code: -32602,
            message: "Missing 'name' parameter".to_string(),
            data: None,
        })?;

    let fqid = params
        .get("arguments")
        .and_then(|a| a.get("fqid"))
        .and_then(|v| v.as_str())
        .unwrap_or("ROOT");

    let messages = match name {
        "glow_guide_step" => vec![PromptMessage {
            role: "user".to_string(),
            content: PromptContent {
                content_type: "text".to_string(),
                text: format!(
                    "I need guidance for step {}. Please analyze the step context and provide recommendations.",
                    fqid
                ),
            },
        }],
        "glow_validate_output" => vec![PromptMessage {
            role: "user".to_string(),
            content: PromptContent {
                content_type: "text".to_string(),
                text: format!(
                    "Please validate the output for step {} before I mark it as complete.",
                    fqid
                ),
            },
        }],
        _ => {
            return Err(RpcError {
                code: -32602,
                message: format!("Unknown prompt: {}", name),
                data: None,
            });
        }
    };

    Ok(json!({ "messages": messages }))
}
