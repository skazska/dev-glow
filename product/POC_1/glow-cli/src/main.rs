//! Glow CLI Application
//!
//! Command-line interface for dev-glow development process management.

mod commands;
mod output;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Dev-Glow: Development Process Management Tool
#[derive(Parser)]
#[command(name = "glow")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Configuration directory path (overrides default .glow/)
    #[arg(long, env = "DEV_GLOW_CONFIG_DIR")]
    config_dir: Option<PathBuf>,

    /// Project root directory (defaults to current directory)
    #[arg(long, default_value = ".")]
    project_dir: PathBuf,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Project management commands
    Project {
        #[command(subcommand)]
        action: ProjectAction,
    },

    /// Initialize a step or process iteration
    Init {
        /// Step FQID (e.g., ROOT, FEAT-001, FEAT-001.REQ-001)
        fqid: String,

        /// Force new iteration for process steps
        #[arg(long)]
        new: bool,

        /// Scope parameters (--PARAM_NAME=value)
        #[arg(last = true)]
        params: Vec<String>,
    },

    /// Start a task step
    Start {
        /// Step FQID
        fqid: String,
    },

    /// Finish a task step
    Finish {
        /// Step FQID
        fqid: String,

        /// Summary of work done
        #[arg(long)]
        summary: Option<String>,

        /// Output parameters (--PARAM_NAME=value)
        #[arg(last = true)]
        params: Vec<String>,
    },

    /// Show project status
    Status {
        /// Show as list instead of tree
        #[arg(long)]
        list: bool,

        /// Attributes to display (comma-separated)
        #[arg(long, default_value = "id,status,purpose")]
        attrs: String,

        /// Filter expression
        #[arg(long)]
        filter: Option<String>,
    },

    /// Show step details
    Show {
        /// Step FQID
        fqid: String,

        /// Include full context
        #[arg(long)]
        context: bool,
    },

    /// Get recommended next actions
    Next,

    /// Show progress metrics
    Progress {
        /// Output format (text or json)
        #[arg(long, default_value = "text")]
        format: String,
    },

    /// Validate context quality
    Validate {
        /// Specific step FQID to validate (optional)
        fqid: Option<String>,

        /// Attempt automatic fixes
        #[arg(long)]
        fix: bool,
    },
}

#[derive(Subcommand)]
enum ProjectAction {
    /// Initialize a new glow project
    Init {
        /// Project name
        #[arg(long)]
        name: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(log_level)),
        )
        .init();

    // Resolve project directory
    let project_dir = std::fs::canonicalize(&cli.project_dir)?;

    // Execute command
    match cli.command {
        Commands::Project { action } => match action {
            ProjectAction::Init { name } => {
                commands::project_init(&project_dir, name)?;
            }
        },

        Commands::Init { fqid, new, params } => {
            commands::init_step(&project_dir, &fqid, new, params)?;
        }

        Commands::Start { fqid } => {
            commands::start_step(&project_dir, &fqid)?;
        }

        Commands::Finish { fqid, summary, params } => {
            commands::finish_step(&project_dir, &fqid, summary, params)?;
        }

        Commands::Status { list, attrs, filter } => {
            commands::show_status(&project_dir, list, &attrs, filter.as_deref())?;
        }

        Commands::Show { fqid, context } => {
            commands::show_step(&project_dir, &fqid, context)?;
        }

        Commands::Next => {
            commands::show_next(&project_dir)?;
        }

        Commands::Progress { format } => {
            commands::show_progress(&project_dir, &format)?;
        }

        Commands::Validate { fqid, fix } => {
            commands::validate(&project_dir, fqid.as_deref(), fix)?;
        }
    }

    Ok(())
}
