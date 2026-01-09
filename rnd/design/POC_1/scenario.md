# POC_1 Validation Scenario

TODO: AI generated scenario, might need manual adjustments.

## Overview

This scenario validates the POC_1 implementation by simulating a realistic solo developer workflow building a small CLI utility. The scenario exercises all major features: process configuration, step lifecycle, context management, and CLI operations.

## Scenario: "TaskTrack" - A Simple Task Tracker CLI

A solo developer wants to build a minimal command-line task tracker. This is a realistic small product that exercises the development process without overwhelming complexity.

### Product Vision

**TaskTrack**: A file-based task tracker for developers who want simple local task management without cloud dependencies.

**Core Features**:

1. Add tasks with title and optional description
2. List tasks with filtering by status
3. Mark tasks as done
4. Delete tasks

**Technology**: Rust CLI (aligns with dev-glow's own toolchain)

---

## Process Configuration

### Development Process Model

```Plain
ROOT (Development Process)
├── FEAT-001: Add Task
│   ├── REQ-001: Define task data model
│   ├── REQ-002: CLI command parsing
│   └── TASK-001: Implement add command
├── FEAT-002: List Tasks
│   ├── REQ-003: List display format
│   ├── REQ-004: Status filtering
│   └── TASK-002: Implement list command
├── FEAT-003: Complete Task
│   ├── REQ-005: Task completion logic
│   └── TASK-003: Implement done command
└── FEAT-004: Delete Task
    ├── REQ-006: Task deletion behavior
    └── TASK-004: Implement delete command
```

### Classifications

| Dimension | Values | Purpose |
|-----------|--------|---------|
| **Stage** | Feature, Requirement, Task | Development lifecycle stage |
| **Layer** | Core, CLI, Storage | Technical layer |
| **Priority** | Must, Should, Could | MoSCoW prioritization |

### Parameter Types

| ID | Purpose | Data Type | Usage |
|----|---------|-----------|-------|
| `DESCRIPTION` | Feature/requirement description | CONTENT | Detailed markdown file |
| `ACCEPTANCE_CRITERIA` | Definition of done | STR | Comma-separated criteria |
| `IMPLEMENTATION_LINK` | Link to code artifact | CONTENT | Path to source file |
| `TEST_LINK` | Link to test artifact | CONTENT | Path to test file |
| `REQ_LIST` | Requirements list | SET | Hierarchical requirement codes |
| `NOTES` | Implementation notes | TEMPLATE | Context-aware notes template |

---

## Scenario Walkthrough

### Phase 1: Project Initialization

**Goal**: Initialize dev-glow in an empty project directory.

**Commands**:

```bash
mkdir tasktrack && cd tasktrack
git init
glow project init
```

**Expected Result**:

- `.glow/` folder created with:
  - `config.yaml` - project configuration
  - `process_config.yaml` - process definition
  - `templates/` - step templates
  - `schemas/` - validation schemas
- `glow/` folder created with:
  - `description.md` - project description placeholder

**Validation Criteria**:

- [ ] All files created with correct structure
- [ ] YAML files pass schema validation
- [ ] Default template is functional

---

### Phase 2: Process Configuration

**Goal**: Configure the development process for TaskTrack.

**Actions**:

1. Edit `.glow/process_config.yaml` with feature/requirement/task structure
2. Define classifications for Stage, Layer, Priority
3. Define parameter types for DESCRIPTION, ACCEPTANCE_CRITERIA, etc.

**Configuration**: See [scenario/process_config.yaml](scenario/process_config.yaml)

**Validation Criteria**:

- [ ] Process configuration passes schema validation
- [ ] Classifications are correctly defined
- [ ] Parameter types cover all needed data

---

### Phase 3: First Feature Iteration (FEAT-001: Add Task)

**Goal**: Walk through complete feature lifecycle.

#### 3.1 Initialize Root Process

```bash
glow init ROOT --PRODUCT_VISION="Simple CLI task tracker"
```

**Expected**:

- Root process enters `in-progress` status
- First iteration folder created: `glow/iteration_000001/`
- Feature steps created in `wait` status

#### 3.2 Start First Feature

```bash
glow init ROOT.FEAT-001 --FEATURE_DESCRIPTION="./artifacts/feat-001-description.md"
```

**Expected**:

- FEAT-001 enters `in-progress` status
- Iteration folder created: `glow/iteration_000001/FEAT-001/iteration_000001/`
- REQ-001, REQ-002 enter `todo` status (no dependencies)
- TASK-001 remains in `wait` (depends on requirements)

#### 3.3 Work on Requirements

```bash
# View what to do next
glow next
# Output: "You can start: REQ-001, REQ-002"

# Start requirement
glow start ROOT.FEAT-001.REQ-001

# View task context
glow show ROOT.FEAT-001.REQ-001
```

**Expected**:

- REQ-001 enters `in-progress` status
- Description file rendered with context
- Show displays full step context

#### 3.4 Complete Requirement

```bash
glow finish ROOT.FEAT-001.REQ-001 \
  --ACCEPTANCE_CRITERIA="Task has id, title, status, created_at" \
  --summary="Defined Task struct with basic fields"
```

**Expected**:

- REQ-001 enters `done` status
- Output parameters captured
- Dependent steps (TASK-001) checked but still waiting for REQ-002

#### 3.5 Complete All Requirements, Start Implementation

```bash
# Complete REQ-002
glow start ROOT.FEAT-001.REQ-002
glow finish ROOT.FEAT-001.REQ-002 \
  --ACCEPTANCE_CRITERIA="add <title> [--desc <description>]" \
  --summary="CLI uses clap with add subcommand"

# Now TASK-001 should be ready
glow next
# Output: "You can start: TASK-001"

glow start ROOT.FEAT-001.TASK-001
```

**Expected**:

- TASK-001 enters `in-progress` with full context from requirements
- Description file contains all requirement outputs

#### 3.6 Complete Implementation

```bash
glow finish ROOT.FEAT-001.TASK-001 \
  --IMPLEMENTATION_LINK="./src/commands/add.rs" \
  --TEST_LINK="./tests/add_test.rs" \
  --summary="Implemented add command with file-based storage"
```

**Expected**:

- TASK-001 enters `done` status
- Feature iteration complete check
- FEAT-001 enters `done` status (all steps complete)

---

### Phase 4: Progress Monitoring

**Goal**: Verify status and progress tracking.

**Commands**:

```bash
# Tree view of project
glow status

# Progress metrics
glow progress

# Context quality
glow validate
```

**Expected Output** (after FEAT-001 complete):

```
glow status:
ROOT (in-progress)
├── FEAT-001 (done)
│   ├── REQ-001 (done)
│   ├── REQ-002 (done)
│   └── TASK-001 (done)
├── FEAT-002 (wait)
├── FEAT-003 (wait)
└── FEAT-004 (wait)

glow progress:
Total Steps: 13
  done: 4 (31%)
  wait: 9 (69%)
Feature Progress: 1/4 (25%)
```

---

### Phase 5: Context Quality Validation

**Goal**: Verify context consistency and completeness.

```bash
glow validate
```

**Expected Output**:

```
Context Quality Report:
✓ Completeness: 100% (all required parameters filled for done steps)
✓ Consistency: Valid (no broken references)
✓ Semantic Connection: Valid (all outputs mapped correctly)

Warnings:
- FEAT-002: 3 steps waiting, no iteration started
- FEAT-003: 2 steps waiting, no iteration started
- FEAT-004: 2 steps waiting, no iteration started
```

---

## Success Criteria for POC_1

### Functional Validation

| Criterion | Requirement | Test |
|-----------|-------------|------|
| Process Init | POC_1-2 | `glow project init` creates valid structure |
| Process Config | POC_1-1 | YAML config parsed and validated |
| Step Lifecycle | POC_1-5,6,7 | wait→todo→in-progress→done transitions work |
| Status View | POC_1-3 | Tree view shows correct hierarchy and statuses |
| Next Actions | POC_1-4 | Correctly identifies available steps |
| Progress Metrics | POC_1-8 | Accurate counts and percentages |
| Context Quality | POC_1-10 | Completeness and consistency assessed |

### Data Model Validation

| Criterion | Requirement | Test |
|-----------|-------------|------|
| File Structure | POC_1-9 | Step files created in correct locations |
| Frontmatter | POC_1-9.1.5 | YAML frontmatter valid and complete |
| Templates | POC_1-1.4 | Data files rendered correctly from templates |
| Parameters | POC_1-1.6 | All data types work (STR, CONTENT, SET, etc.) |
| Links | POC_1-1.5 | Dependencies block correctly, resolve on completion |

### User Experience Validation

| Criterion | Test |
|-----------|------|
| Ease of Setup | < 5 minutes from clone to first step |
| Command Clarity | Commands are intuitive, help is useful |
| Error Messages | Validation errors are actionable |
| Git Friendliness | Diffs are readable, merges work |

---

## Artifacts Reference

| File | Description |
|------|-------------|
| [scenario/config.yaml](scenario/config.yaml) | Project configuration for TaskTrack |
| [scenario/process_config.yaml](scenario/process_config.yaml) | Full process definition |

---

## Notes for Evaluation

### What This Scenario Tests

1. **Full Lifecycle**: From project init through feature completion
2. **Dependency Resolution**: TASK-001 waits for REQ-001 and REQ-002
3. **Context Propagation**: Requirement outputs become task inputs
4. **Parameter Types**: Uses STR, CONTENT, SET types
5. **Progress Tracking**: Metrics calculated correctly

### What This Scenario Does NOT Test

1. **MCP Integration**: AI assistant usage (deferred to separate validation)
2. **Multiple Iterations**: Only one iteration per feature
3. **Complex Classifications**: Uses simple 3-dimension classification
4. **Template Customization**: Uses default templates only
5. **Concurrent Work**: Single-threaded execution

### Known Limitations to Observe

1. **Manual Context Extraction**: No AI-assisted context preparation
2. **Output Mapping**: Manual parameter value entry
3. **Validation Depth**: Basic completeness check only
