# Dev-Glow User Guide

**Version: POC_1**

Dev-Glow is a development process management tool that helps you organize and track your software development workflow. It maintains structured context that can be leveraged by AI assistants for more effective collaboration.

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Core Concepts](#core-concepts)
3. [CLI Command Reference](#cli-command-reference)
4. [Process Configuration](#process-configuration)
5. [Workflow Examples](#workflow-examples)
6. [File Structure](#file-structure)
7. [AI Integration (MCP)](#ai-integration-mcp)

---

## Getting Started

### Installation

```bash
# Clone the repository
cd dev-glow/product/POC_1

# Build and install
cargo build --release
cargo install --path glow-cli
```

### Your First Project

```bash
# 1. Create and enter your project directory
mkdir my-project && cd my-project

# 2. Initialize glow
glow project init --name "My Project"

# 3. Start the root process
glow init ROOT
glow start ROOT

# 4. Check your status
glow status
```

You now have a glow-managed project ready for structured development.

---

## Core Concepts

### Steps

A **step** is a unit of work in your development process. Steps can be:

- **Process steps**: Container steps that have sub-steps (e.g., a Feature containing Requirements and Tasks)
- **Task steps**: Leaf steps representing actual work to be done

### Fully Qualified ID (FQID)

Each step has a unique identifier called FQID. Nested steps use dot notation:

```
ROOT                    # Root process
FEAT-001                # A feature under ROOT
FEAT-001.REQ-001        # A requirement under FEAT-001
FEAT-001.REQ-001.TASK-001   # A task under REQ-001
```

### Step Status Lifecycle

Steps progress through these statuses:

```
Wait → Todo → InProgress → Done
```

| Status | Meaning |
|--------|---------|
| `Wait` | Blocked by dependencies |
| `Todo` | Ready to start (dependencies satisfied) |
| `InProgress` | Currently being worked on |
| `Done` | Completed |

### Dependencies

Steps can depend on other steps. A step remains in `Wait` status until all its dependencies are `Done`.

---

## CLI Command Reference

### Project Management

#### `glow project init`

Initialize a new glow project in the current directory.

```bash
glow project init [--name <PROJECT_NAME>]
```

**Options:**

- `--name`: Project name (optional, defaults to directory name)

**Creates:**

- `.glow/config.yaml` - Project configuration
- `.glow/process_config.yaml` - Process definition
- `.glow/templates/` - Step templates
- `.glow/schemas/` - Validation schemas
- `glow/` - Step data directory

---

### Step Lifecycle Commands

#### `glow init <FQID>`

Initialize a step, making it ready to work on.

```bash
glow init <FQID> [--new] [-- PARAM=value ...]
```

**Arguments:**

- `FQID`: Fully qualified step ID

**Options:**

- `--new`: Force new iteration for repeatable steps
- `-- PARAM=value`: Scope parameters (after `--`)

**Examples:**

```bash
# Initialize root process
glow init ROOT

# Initialize with parameters
glow init FEAT-001 -- FEATURE_NAME="User Auth" PRIORITY="high"

# Force new iteration
glow init FEAT --new
```

---

#### `glow start <FQID>`

Start working on an initialized step. Transitions from `Todo` to `InProgress`.

```bash
glow start <FQID>
```

**Example:**

```bash
glow start FEAT-001.REQ-001
```

---

#### `glow finish <FQID>`

Complete a step. Transitions from `InProgress` to `Done`.

```bash
glow finish <FQID> [--summary <TEXT>] [-- PARAM=value ...]
```

**Options:**

- `--summary`: Summary of work completed
- `-- PARAM=value`: Output parameters

**Examples:**

```bash
# Simple finish
glow finish FEAT-001.REQ-001

# With summary and outputs
glow finish FEAT-001.TASK-001 \
  --summary "Implemented user registration API" \
  -- IMPLEMENTATION_LINK="src/auth.rs" TEST_LINK="tests/auth_test.rs"
```

---

### Status & Information Commands

#### `glow status`

Show the project status tree.

```bash
glow status [--list]
```

**Options:**

- `--list`: Show as flat list instead of tree

**Example Output:**

```
ROOT (in-progress)
├── FEAT-001 (done)
│   ├── REQ-001 (done)
│   ├── REQ-002 (done)
│   └── TASK-001 (done)
├── FEAT-002 (todo)
└── FEAT-003 (wait)
```

---

#### `glow show <FQID>`

Show detailed information about a specific step.

```bash
glow show <FQID> [--context]
```

**Options:**

- `--context`: Include full context chain

**Example Output:**

```
════════════════════════════════════════════════════════════
Step: REQ-001
FQID: FEAT-001.REQ-001
Status: done
Classification: Requirement,Core,Must

Purpose:
  Define task data model

Expectations:
  Task struct defined with all necessary fields

Outputs:
  ACCEPTANCE_CRITERIA: "Task has id, title, status, created_at"
════════════════════════════════════════════════════════════
```

---

#### `glow next`

Get recommended next actions based on current state.

```bash
glow next
```

**Example Output:**

```
Recommended Next Actions:

→ Start FEAT-001.REQ-001: Define task data model
→ Start FEAT-001.REQ-002: CLI command parsing for add

Run: glow start <FQID> to begin
```

---

#### `glow progress`

Show progress metrics for the project.

```bash
glow progress [--format <text|json>]
```

**Example Output:**

```
Project Progress
────────────────

Total Steps:  13
Completed:    4 (31%)
In Progress:  1 (8%)
Waiting:      8 (61%)

Progress: ████████░░░░░░░░░░░░░░░░░░ 31%
```

---

#### `glow validate`

Validate context quality and check for issues.

```bash
glow validate [<FQID>] [--fix]
```

**Options:**

- `FQID`: Validate specific step only
- `--fix`: Attempt automatic fixes

**Example Output:**

```
Context Quality Report
──────────────────────

Completeness: 85%
✓ All required parameters filled for completed steps
✓ No broken references

Warnings:
⚠ FEAT-002: 3 steps waiting, not started
⚠ FEAT-003: 2 steps waiting, not started
```

---

### Global Options

These options apply to all commands:

| Option | Description |
|--------|-------------|
| `--project-dir <PATH>` | Project root directory (default: current directory) |
| `--config-dir <PATH>` | Configuration directory (overrides `.glow/`) |
| `-v, --verbose` | Enable verbose output |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

**Environment Variables:**

- `DEV_GLOW_CONFIG_DIR`: Default configuration directory

---

## Process Configuration

### Project Configuration (`.glow/config.yaml`)

```yaml
version: "0.1.0"
project_name: "My Project"
data_folder: "glow"                    # Where step files are stored
process_config: "process_config.yaml"  # Process definition file
templates_folder: "templates/"         # Step templates
default_template: "any-step.md"        # Default template
```

### Process Definition (`.glow/process_config.yaml`)

```yaml
version: "0.1.0"

# Classifications for categorizing steps
classifications:
  - id: stage
    name: "Development Stage"
    values:
      - key: Feature
        name: "Feature"
      - key: Requirement
        name: "Requirement"
      - key: Task
        name: "Task"

  - id: priority
    name: "Priority"
    values:
      - key: Must
        name: "Must Have"
      - key: Should
        name: "Should Have"

# Reusable parameter types
parameter_types:
  - id: DESCRIPTION
    purpose: "Detailed description"
    data_type: CONTENT    # CONTENT = file reference
    
  - id: ACCEPTANCE_CRITERIA
    purpose: "Definition of done"
    data_type: STR        # STR = string value
    is_required: true

# Root process definition
root_process:
  id: ROOT
  purpose: "Project Development Process"
  scope:
    - id: PROJECT_VISION
      type_ref: DESCRIPTION

  steps:
    - id: FEAT-001
      purpose: "Add Task Feature"
      classification: "Feature,Must"
      
      steps:
        - id: REQ-001
          purpose: "Define data model"
          classification: "Requirement,Must"
          outputs:
            - id: ACCEPTANCE_CRITERIA
              type_ref: ACCEPTANCE_CRITERIA

        - id: TASK-001
          purpose: "Implement feature"
          classification: "Task,Must"
          inputs:
            - id: REQ_CRITERIA
              type_ref: ACCEPTANCE_CRITERIA
              mapping: "links.REQ-001.output.ACCEPTANCE_CRITERIA"

      # Dependencies within this feature
      links:
        - type: dependency
          from: TASK-001
          to: REQ-001
```

### Parameter Data Types

| Type | Description | Example |
|------|-------------|---------|
| `STR` | String value | `"User authentication"` |
| `CONTENT` | File reference or content block | `"./docs/spec.md"` |
| `SET` | Collection of values | `["item1", "item2"]` |
| `TEMPLATE` | Templated content | Handlebars template |

---

## Workflow Examples

### Example 1: Simple Feature Development

```bash
# Start the project
glow project init --name "TaskTracker"
glow init ROOT -- PROJECT_VISION="Simple task tracker"
glow start ROOT

# Work on first feature
glow init FEAT-001 -- FEATURE_DESCRIPTION="Add task command"
glow start FEAT-001

# Gather requirements
glow init FEAT-001.REQ-001
glow start FEAT-001.REQ-001
# ... do requirements work ...
glow finish FEAT-001.REQ-001 \
  --summary "Defined Task struct" \
  -- ACCEPTANCE_CRITERIA="Task has id, title, status"

# Check what's next
glow next

# Implement
glow init FEAT-001.TASK-001
glow start FEAT-001.TASK-001
# ... write code ...
glow finish FEAT-001.TASK-001 \
  --summary "Implemented add command" \
  -- IMPLEMENTATION_LINK="src/add.rs"

# Check progress
glow progress
```

### Example 2: Checking Status During Work

```bash
# See overall status
glow status

# See what you can work on
glow next

# Get details on a specific step
glow show FEAT-001.REQ-001

# Validate quality
glow validate
```

---

## File Structure

After initialization, your project will have:

```
my-project/
├── .glow/                          # Configuration directory
│   ├── config.yaml                 # Project config
│   ├── process_config.yaml         # Process definition
│   ├── schemas/                    # JSON schemas
│   │   ├── config.schema.json
│   │   ├── process_config.schema.json
│   │   └── step_data.schema.json
│   └── templates/                  # Step templates
│       └── any-step.md
│
└── glow/                           # Step data directory
    ├── ROOT.md                     # Root step file
    ├── ROOT/                       # Root sub-folder
    │   └── description.md
    ├── FEAT-001.md                 # Feature step file
    └── FEAT-001/
        ├── description.md
        ├── REQ-001.md
        ├── REQ-001/
        │   └── description.md
        └── TASK-001.md
```

### Step File Format

Step files use YAML frontmatter with Markdown content:

```markdown
---
id: "REQ-001"
parent: "FEAT-001"
status: "done"
classification: "Requirement,Core,Must"
purpose: "Define task data model"
expectations: "Task struct defined with all necessary fields"
scope: []
input: []
output:
  - id: "ACCEPTANCE_CRITERIA"
    value: "Task has id, title, status, created_at"
own_steps: []
---

# REQ-001: Define task data model

## Description

Define the core Task data structure...
```

---

## AI Integration (MCP)

Dev-Glow includes an MCP (Model Context Protocol) server for AI integration.

### Starting the MCP Server

```bash
glow-mcp --project-dir /path/to/project
```

### Available Resources

| URI | Description |
|-----|-------------|
| `glow://project/status` | Current project status tree |
| `glow://step/{fqid}` | Specific step details |
| `glow://next` | Recommended next actions |
| `glow://progress` | Progress metrics |

### Available Tools

| Tool | Description |
|------|-------------|
| `glow_status` | Get project status |
| `glow_next` | Get next actions |
| `glow_show_step` | Show step details |
| `glow_start_step` | Start working on a step |
| `glow_finish_step` | Complete a step |
| `glow_progress` | Get progress metrics |
| `glow_validate` | Validate context quality |

### AI Prompts

| Prompt | Description |
|--------|-------------|
| `glow_guide_step` | Get AI guidance for a step |
| `glow_validate_output` | Validate step output before finishing |

---

## Troubleshooting

### Common Issues

**"Project already exists"**

```bash
# Remove existing config and reinitialize
rm -rf .glow
glow project init
```

**"Step not found"**

```bash
# Check correct FQID with status
glow status

# Initialize parent steps first
glow init ROOT
glow start ROOT
glow init FEAT-001
```

**"Invalid state transition"**

```bash
# Check current status
glow show FEAT-001

# Steps must follow: init → start → finish
# Cannot start a 'Wait' step (dependencies not met)
# Cannot finish a 'Todo' step (must start first)
```

**"Missing required parameter"**

```bash
# Check what's required
glow show FEAT-001

# Provide required scope parameters
glow init FEAT-001 -- REQUIRED_PARAM="value"
```

---

## POC_1 Limitations

This is a Proof of Concept. Known limitations:

1. **Cross-feature dependency updates**: When a feature completes, sibling features that depend on it don't automatically transition from `Wait` to `Todo`. Workaround: manually re-init dependent steps.

2. **Iteration naming**: Steps use template IDs (FEAT, REQ) rather than auto-generated iteration IDs (FEAT-001, FEAT-002).

3. **Input mapping resolution**: Input mappings are defined but values aren't automatically resolved from dependency outputs.

4. **Template customization**: Only the default template is provided.

---

## Getting Help

```bash
# General help
glow --help

# Command-specific help
glow init --help
glow finish --help

# Version info
glow --version
```
