//! Glow MCP Server
//!
//! Model Context Protocol server for AI-assisted development process management.

mod handlers;
mod protocol;
mod resources;
mod tools;

use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use glow_core::engine::operations::ProcessEngine;
use tokio::sync::RwLock;

/// MCP Server for Dev-Glow
#[derive(Parser)]
#[command(name = "glow-mcp")]
#[command(version, about)]
struct Args {
    /// Project directory
    #[arg(long, default_value = ".")]
    project_dir: PathBuf,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

/// Server state
pub struct ServerState {
    pub engine: RwLock<Option<ProcessEngine>>,
    pub project_dir: PathBuf,
}

impl ServerState {
    pub fn new(project_dir: PathBuf) -> Self {
        Self {
            engine: RwLock::new(None),
            project_dir,
        }
    }

    pub async fn ensure_engine(&self) -> Result<()> {
        let mut engine = self.engine.write().await;
        if engine.is_none() {
            *engine = Some(ProcessEngine::new(self.project_dir.clone())?);
        }
        Ok(())
    }
}

/// MCP Protocol version
const PROTOCOL_VERSION: &str = "2024-11-05";

fn main() -> Result<()> {
    let args = Args::parse();

    // Setup logging
    let log_level = if args.verbose { "debug" } else { "warn" };
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(log_level)),
        )
        .with_writer(std::io::stderr)
        .init();

    // Run async runtime
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let project_dir = std::fs::canonicalize(&args.project_dir)?;
        let state = Arc::new(ServerState::new(project_dir));

        // Run the stdio-based JSON-RPC server
        run_stdio_server(state).await
    })
}

/// Run the MCP server over stdio
async fn run_stdio_server(state: Arc<ServerState>) -> Result<()> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    tracing::info!("MCP server starting...");

    for line in stdin.lock().lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        tracing::debug!("Received: {}", line);

        let response = match serde_json::from_str::<protocol::JsonRpcRequest>(&line) {
            Ok(request) => handlers::handle_request(&state, request).await,
            Err(e) => protocol::JsonRpcResponse::error(
                serde_json::Value::Null,
                protocol::RpcError {
                    code: -32700,
                    message: format!("Parse error: {}", e),
                    data: None,
                },
            ),
        };

        let output = serde_json::to_string(&response)?;
        tracing::debug!("Sending: {}", output);

        writeln!(stdout, "{}", output)?;
        stdout.flush()?;
    }

    Ok(())
}
