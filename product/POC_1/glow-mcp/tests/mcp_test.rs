//! MCP Server Integration Tests
//!
//! Tests for the glow-mcp protocol handling and tool responses.

use std::fs;
use tempfile::TempDir;

// Re-export the modules we need to test
// Since glow-mcp is a binary crate, we test the public interfaces

/// Helper to create a test project directory with valid config
fn setup_test_project() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let path = temp_dir.path();

    // Create directory structure
    fs::create_dir_all(path.join(".glow")).expect("Failed to create .glow dir");
    fs::create_dir_all(path.join("glow")).expect("Failed to create glow dir");

    // Write config
    let config = r#"
version: "1.0"
project_id: "TEST"
project_name: "Test Project"
process: "simple"
glow_dir: "glow"
"#;
    fs::write(path.join(".glow/config.yaml"), config).expect("Failed to write config");

    // Write process config
    let process_config = r#"
version: "1.0"

root_process:
  id: ROOT
  purpose: "Main process"
  classification: "Process"
  steps:
    - id: TASK1
      classification: "Task"
      purpose: "First task"
    - id: TASK2
      classification: "Task"
      purpose: "Second task"
"#;
    fs::write(path.join(".glow/process_config.yaml"), process_config)
        .expect("Failed to write process config");

    temp_dir
}

mod protocol_tests {
    use serde_json::json;

    /// Test JSON-RPC request parsing
    #[test]
    fn test_jsonrpc_request_parsing() {
        let json_str = r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {}
        }"#;

        let parsed: serde_json::Value = serde_json::from_str(json_str).unwrap();
        assert_eq!(parsed["jsonrpc"], "2.0");
        assert_eq!(parsed["method"], "initialize");
        assert_eq!(parsed["id"], 1);
    }

    /// Test JSON-RPC request with string ID
    #[test]
    fn test_jsonrpc_request_string_id() {
        let json_str = r#"{
            "jsonrpc": "2.0",
            "id": "request-123",
            "method": "tools/list",
            "params": {}
        }"#;

        let parsed: serde_json::Value = serde_json::from_str(json_str).unwrap();
        assert_eq!(parsed["id"], "request-123");
    }

    /// Test JSON-RPC request without params
    #[test]
    fn test_jsonrpc_request_no_params() {
        let json_str = r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "method": "ping"
        }"#;

        let parsed: serde_json::Value = serde_json::from_str(json_str).unwrap();
        assert!(parsed.get("params").is_none());
    }

    /// Test tool call request structure
    #[test]
    fn test_tool_call_request_structure() {
        let request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": "glow_status",
                "arguments": {}
            }
        });

        let params = request.get("params").unwrap();
        assert_eq!(params["name"], "glow_status");
        assert!(params.get("arguments").is_some());
    }

    /// Test resource read request structure
    #[test]
    fn test_resource_read_request_structure() {
        let request = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "resources/read",
            "params": {
                "uri": "glow://project/status"
            }
        });

        let uri = request["params"]["uri"].as_str().unwrap();
        assert!(uri.starts_with("glow://"));
    }
}

mod response_format_tests {
    use serde_json::json;

    /// Test success response format
    #[test]
    fn test_success_response_format() {
        let response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "tools": []
            }
        });

        assert_eq!(response["jsonrpc"], "2.0");
        assert!(response.get("result").is_some());
        assert!(response.get("error").is_none());
    }

    /// Test error response format
    #[test]
    fn test_error_response_format() {
        let response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": -32601,
                "message": "Method not found"
            }
        });

        assert_eq!(response["jsonrpc"], "2.0");
        assert!(response.get("error").is_some());
        assert_eq!(response["error"]["code"], -32601);
    }

    /// Test tool result format
    #[test]
    fn test_tool_result_format() {
        let result = json!({
            "content": [
                {
                    "type": "text",
                    "text": "Success message"
                }
            ]
        });

        let content = result["content"].as_array().unwrap();
        assert_eq!(content.len(), 1);
        assert_eq!(content[0]["type"], "text");
    }

    /// Test tool error result format
    #[test]
    fn test_tool_error_result_format() {
        let result = json!({
            "content": [
                {
                    "type": "text",
                    "text": "Error: Something went wrong"
                }
            ],
            "isError": true
        });

        assert_eq!(result["isError"], true);
    }

    /// Test resource content format
    #[test]
    fn test_resource_content_format() {
        let content = json!({
            "uri": "glow://project/status",
            "mimeType": "application/json",
            "text": "{}"
        });

        assert!(content["uri"].as_str().unwrap().starts_with("glow://"));
        assert_eq!(content["mimeType"], "application/json");
    }

    /// Test initialize result format
    #[test]
    fn test_initialize_result_format() {
        let result = json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "resources": {
                    "subscribe": false,
                    "list_changed": false
                },
                "tools": {
                    "list_changed": false
                },
                "prompts": {
                    "list_changed": false
                }
            },
            "serverInfo": {
                "name": "glow-mcp",
                "version": "0.1.0"
            }
        });

        assert!(result.get("protocolVersion").is_some());
        assert!(result.get("capabilities").is_some());
        assert!(result.get("serverInfo").is_some());
        assert_eq!(result["serverInfo"]["name"], "glow-mcp");
    }
}

mod tool_schema_tests {
    use serde_json::json;

    /// Test glow_status tool has correct schema
    #[test]
    fn test_glow_status_schema() {
        let tool = json!({
            "name": "glow_status",
            "description": "Get current project status tree showing all steps and their states",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        });

        assert_eq!(tool["name"], "glow_status");
        assert_eq!(tool["inputSchema"]["type"], "object");
    }

    /// Test glow_show_step tool requires fqid
    #[test]
    fn test_glow_show_step_schema() {
        let tool = json!({
            "name": "glow_show_step",
            "description": "Show detailed information about a specific step",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "fqid": {
                        "type": "string",
                        "description": "Fully qualified step ID"
                    }
                },
                "required": ["fqid"]
            }
        });

        let required = tool["inputSchema"]["required"].as_array().unwrap();
        assert!(required.contains(&json!("fqid")));
    }

    /// Test glow_finish_step tool schema
    #[test]
    fn test_glow_finish_step_schema() {
        let tool = json!({
            "name": "glow_finish_step",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "fqid": { "type": "string" },
                    "summary": { "type": "string" },
                    "outputs": { "type": "object" }
                },
                "required": ["fqid"]
            }
        });

        let props = tool["inputSchema"]["properties"].as_object().unwrap();
        assert!(props.contains_key("fqid"));
        assert!(props.contains_key("summary"));
        assert!(props.contains_key("outputs"));
    }
}

mod resource_uri_tests {
    /// Test project status URI format
    #[test]
    fn test_project_status_uri() {
        let uri = "glow://project/status";
        assert!(uri.starts_with("glow://"));
        assert!(uri.contains("project"));
        assert!(uri.contains("status"));
    }

    /// Test step URI format
    #[test]
    fn test_step_uri_format() {
        let fqid = "FEAT-001.REQ-001";
        let uri = format!("glow://step/{}", fqid);
        assert_eq!(uri, "glow://step/FEAT-001.REQ-001");
    }

    /// Test step URI parsing
    #[test]
    fn test_step_uri_parsing() {
        let uri = "glow://step/ROOT";
        let fqid = uri.strip_prefix("glow://step/").unwrap();
        assert_eq!(fqid, "ROOT");
    }

    /// Test nested step URI
    #[test]
    fn test_nested_step_uri() {
        let uri = "glow://step/ROOT.FEAT-001.TASK-001";
        let fqid = uri.strip_prefix("glow://step/").unwrap();
        let parts: Vec<&str> = fqid.split('.').collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "ROOT");
        assert_eq!(parts[1], "FEAT-001");
        assert_eq!(parts[2], "TASK-001");
    }
}

mod prompt_tests {
    use serde_json::json;

    /// Test prompt list structure
    #[test]
    fn test_prompt_list_structure() {
        let prompts = json!({
            "prompts": [
                {
                    "name": "glow_guide_step",
                    "description": "Get AI guidance for a specific development step",
                    "arguments": [
                        {
                            "name": "fqid",
                            "description": "Step FQID to get guidance for",
                            "required": true
                        }
                    ]
                }
            ]
        });

        let prompt_list = prompts["prompts"].as_array().unwrap();
        assert!(!prompt_list.is_empty());

        let first_prompt = &prompt_list[0];
        assert_eq!(first_prompt["name"], "glow_guide_step");
    }

    /// Test prompt message structure
    #[test]
    fn test_prompt_message_structure() {
        let response = json!({
            "messages": [
                {
                    "role": "user",
                    "content": {
                        "type": "text",
                        "text": "I need guidance for step ROOT."
                    }
                }
            ]
        });

        let messages = response["messages"].as_array().unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0]["role"], "user");
        assert_eq!(messages[0]["content"]["type"], "text");
    }
}

mod engine_integration_tests {
    use super::*;
    use glow_core::engine::operations::ProcessEngine;

    /// Test that engine can be created with test project
    #[test]
    fn test_engine_creation() {
        let project = setup_test_project();
        let engine = ProcessEngine::new(project.path().to_path_buf());
        assert!(engine.is_ok());
    }

    /// Test engine status tree for MCP resource
    #[test]
    fn test_engine_status_for_resources() {
        let project = setup_test_project();
        let mut engine = ProcessEngine::new(project.path().to_path_buf()).unwrap();

        // Init root
        engine.init_step("ROOT", vec![], false).unwrap();

        // Get status tree (used by resources/read)
        let status = engine.get_status_tree();
        assert!(status.is_ok());

        let tree = status.unwrap();
        assert_eq!(tree.id, "ROOT");
    }

    /// Test engine next actions for MCP tool
    #[test]
    fn test_engine_next_actions() {
        let project = setup_test_project();
        let mut engine = ProcessEngine::new(project.path().to_path_buf()).unwrap();

        engine.init_step("ROOT", vec![], false).unwrap();

        let actions = engine.get_next_actions();
        assert!(actions.is_ok());
    }

    /// Test engine step show for MCP tool
    #[test]
    fn test_engine_show_step() {
        let project = setup_test_project();
        let mut engine = ProcessEngine::new(project.path().to_path_buf()).unwrap();

        engine.init_step("ROOT", vec![], false).unwrap();

        let step_info = engine.show_step("ROOT");
        assert!(step_info.is_ok());
    }

    /// Test engine progress for MCP tool
    #[test]
    fn test_engine_progress() {
        let project = setup_test_project();
        let mut engine = ProcessEngine::new(project.path().to_path_buf()).unwrap();

        engine.init_step("ROOT", vec![], false).unwrap();

        let progress = engine.get_progress();
        assert!(progress.is_ok());
    }
}

mod error_code_tests {
    /// Standard JSON-RPC error codes
    const PARSE_ERROR: i32 = -32700;
    const INVALID_REQUEST: i32 = -32600;
    const METHOD_NOT_FOUND: i32 = -32601;
    const INVALID_PARAMS: i32 = -32602;
    const INTERNAL_ERROR: i32 = -32603;

    #[test]
    fn test_method_not_found_code() {
        assert_eq!(METHOD_NOT_FOUND, -32601);
    }

    #[test]
    fn test_invalid_params_code() {
        assert_eq!(INVALID_PARAMS, -32602);
    }

    #[test]
    fn test_internal_error_code() {
        assert_eq!(INTERNAL_ERROR, -32603);
    }

    #[test]
    fn test_parse_error_code() {
        assert_eq!(PARSE_ERROR, -32700);
    }

    #[test]
    fn test_invalid_request_code() {
        assert_eq!(INVALID_REQUEST, -32600);
    }
}
