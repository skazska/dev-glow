# Dev-Glow

AI generated README and all content of this folder.

**AI-Assisted Development Process Management Tool**

Dev-Glow is a development process management tool that maintains structured context for AI assistants, enabling more effective AI-human collaboration during software development.

## Overview

Dev-Glow provides:

- **Structured Process Management**: Define and track development workflows with steps, dependencies, and quality gates
- **Context Preservation**: Maintain rich context in step files that AI can leverage for better assistance
- **CLI Interface**: Command-line tool for developers to manage their workflow
- **MCP Server**: Model Context Protocol server for direct AI integration

## Architecture

The project is organized as a Rust workspace with three crates:

```
source/
├── glow-core/      # Core library with data models, engine, and templating
├── glow-cli/       # CLI application (glow command)
└── glow-mcp/       # MCP server for AI integration
```

## Quick Start

### Installation

```bash
cd source
cargo build --release

# Install CLI
cargo install --path glow-cli

# Run MCP server
cargo run --bin glow-mcp
```

### Initialize a Project

```bash
# Initialize a new glow project
glow project init --name "MyProject"

# Initialize the root process
glow init ROOT

# Start working
glow start ROOT

# Check status
glow status
```

### CLI Commands

| Command | Description |
|---------|-------------|
| `glow project init` | Initialize a new glow project |
| `glow init <FQID>` | Initialize a step |
| `glow start <FQID>` | Start working on a step |
| `glow finish <FQID>` | Complete a step |
| `glow status` | Show project status tree |
| `glow show <FQID>` | Show step details |
| `glow next` | Get recommended next actions |
| `glow progress` | Show progress metrics |
| `glow validate` | Validate context quality |

### MCP Integration

The MCP server exposes resources and tools for AI assistants:

**Resources:**

- `glow://project/status` - Current project status
- `glow://step/{fqid}` - Step context and details
- `glow://next` - Recommended next actions
- `glow://progress` - Progress metrics

**Tools:**

- `glow_status` - Get project status
- `glow_next` - Get next actions
- `glow_show_step` - Show step details
- `glow_start_step` - Start a step
- `glow_finish_step` - Complete a step
- `glow_progress` - Get progress metrics
- `glow_validate` - Validate context

## Process Configuration

Projects are configured using YAML files in the `.glow/` directory:

### config.yaml

```yaml
version: "1.0"
project_id: "MY-PROJECT"
project_name: "My Project"
process: "my-process"
glow_dir: "glow"
```

### process_config.yaml

```yaml
version: "1.0"
process:
  id: "my-process"
  name: "My Development Process"

steps:
  - id: "ROOT"
    classification: "Process"
    purpose: "Main project process"
    own_steps:
      - id: "FEATURE"
        classification: "Task"
        purpose: "Implement a feature"
```

## Step Files

Step data is stored in Markdown files with YAML frontmatter:

```markdown
---
id: "FEAT-001"
status: "in-progress"
scope:
  FEATURE_NAME: "User Management"
---
# FEAT-001: User Management

## Purpose
Implement user registration and authentication.

## Work Log
- 2024-01-15: Started requirements gathering
```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_tasktrack_full_scenario

# Run with output
cargo test -- --nocapture
```

### Project Structure

```
glow-core/
├── src/
│   ├── config/     # Configuration loading and validation
│   ├── engine/     # Process execution engine
│   ├── model/      # Data models (Step, Process, Parameter)
│   ├── quality/    # Quality assessment
│   ├── storage/    # File system operations
│   └── template/   # Handlebars templating
└── tests/          # Integration tests

glow-cli/
├── src/
│   ├── main.rs     # CLI entry point
│   ├── commands.rs # Command implementations
│   └── output.rs   # Output formatting
└── tests/          # CLI tests

glow-mcp/
├── src/
│   ├── main.rs     # MCP server entry
│   ├── protocol.rs # JSON-RPC protocol types
│   ├── handlers.rs # Request handlers
│   ├── resources.rs # MCP resources
│   └── tools.rs    # MCP tools
```

## License

MIT License - see LICENSE file for details.
