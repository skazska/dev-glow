# 6. Configuration

> Requirements: POC_1-1, POC_1-1.1

## 6.1. General Configuration Schema (`config.yaml`)

> Requirements: POC_1-1, POC_1-1.1, POC_1-2.2

```yaml
# .glow/config.yaml
version: "1.0"                          # Config schema version
project_name: "my-project"              # Project identifier
data_folder: "glow"                     # Data folder name (default: glow)
process_config: "process_config.yaml"  # Process config file name
templates_folder: "templates"           # Templates folder name
default_template: "default-step.md"     # Default step template

```

## 6.2. Process Configuration Schema (`process_config.yaml`)

> Requirements: POC_1-1.2, POC_1-1.3, POC_1-1.3.1, POC_1-1.3.2, POC_1-1.5, POC_1-1.6, POC_1-1.6.1, POC_1-2.2.1

```yaml
# .glow/process_config.yaml
version: "1.0"

# Classification dictionaries
classifications:
  phase:
    analysis:
      default_status: "wait"
      parameters:
        scope: [REQUIREMENTS_DOC]
    design:
      default_status: "wait"
      parameters:
        inputs: [REQUIREMENTS_DOC]
        scope: [DESIGN_DOC]
    implementation:
      default_status: "wait"
      parameters:
        inputs: [DESIGN_DOC]
        outputs: [CODE_ARTIFACT]
    review:
      default_status: "wait"
      parameters:
        inputs: [CODE_ARTIFACT]
        outputs: [REVIEW_NOTES]
  priority:
    high:
      template: "high-priority-step.md"
    normal: {}
    low: {}

# Parameter type definitions
parameter_types:
  REQUIREMENTS_DOC:
    purpose: "Requirements document reference"
    data_type: CONTENT
    mime_type: "text/markdown"
    is_required: true
  DESIGN_DOC:
    purpose: "Design document reference"
    data_type: CONTENT
    mime_type: "text/markdown"
  CODE_ARTIFACT:
    purpose: "Code implementation reference"
    data_type: CONTENT
    mime_type: "text/plain"
  REVIEW_NOTES:
    purpose: "Review feedback and notes"
    data_type: STR
  PRIORITY_LEVEL:
    purpose: "Task priority"
    data_type: SET
    values:
      1: "Critical"
      2: "High"
      3: "Medium"
      4: "Low"

# Link type definitions
link_types:
  dependency:
    description: "Target step must be done before source can start"
    blocks_start: true
  predecessor:
    description: "Target step should be done before source, but not mandatory"
    blocks_start: false
  related:
    description: "Steps are related but independent"
    blocks_start: false

# Root process definition
root_process:
  id: ROOT
  purpose: "Development Process"
  steps: []  # To be filled with step configurations
```

## 6.3. Step Configuration Schema

> Requirements: POC_1-1.2.1, POC_1-1.3, POC_1-1.5

```yaml
# Step configuration within process_config.yaml
steps:
  - id: "FEAT-001"
    purpose: "Feature Development"
    classification: "phase.implementation, priority.high"
    expectations: "Feature fully implemented and tested"
    template: "feature-step.md"  # Optional, overrides classification default
    parameters:
      inputs:
        - id: REQUIREMENTS_DOC
          source: "parent.scope.REQUIREMENTS_DOC"  # Parameter mapping
      scope:
        - id: DESIGN_DOC
        - id: IMPLEMENTATION_NOTES
          data_type: STR
          is_required: false
      outputs:
        - id: CODE_ARTIFACT
          target: "parent.scope.FEATURE_CODE"  # Output mapping
    links:
      - type: dependency
        target: "REQ-001"
      - type: predecessor
        target: "DESIGN-001"
    # Nested process (if step is a process)
    steps:
      - id: "TASK-001"
        purpose: "Implementation Task"
        # ...nested step config
```

## 9. Process Templates

> Requirements: POC_1-1.4, POC_1-2.2.2

### 9.1. Pre-defined Process Templates

> Requirements: POC_1-1.4, POC_1-2.2.2

Templates provide ready-to-use process configurations for common scenarios.

#### Solo MVP Template

Minimal process for rapid solo development:

```yaml
# templates/solo-mvp.yaml
name: "Solo MVP"
description: "Minimal process for rapid solo development"
classifications:
  type:
    feature: {}
    bugfix: {}
    chore: {}
parameter_types:
  DESCRIPTION:
    data_type: STR
    is_required: true
  ACCEPTANCE_CRITERIA:
    data_type: STR
  NOTES:
    data_type: STR
root_process:
  steps:
    - id: "BACKLOG"
      purpose: "Backlog Management"
      steps: []  # Dynamic feature/bugfix/chore items
```

#### Small Team Agile Template

Structured process for small team collaboration:

```yaml
# templates/small-team-agile.yaml
name: "Small Team Agile"
description: "Sprint-based process with code review"
classifications:
  phase:
    planning: {}
    development: {}
    review: {}
    deployment: {}
  type:
    story: {}
    task: {}
    bug: {}
parameter_types:
  USER_STORY:
    data_type: STR
    is_required: true
  ACCEPTANCE_CRITERIA:
    data_type: SET
  REVIEWER:
    data_type: STR
  REVIEW_STATUS:
    data_type: SET
    values:
      1: "Pending"
      2: "Approved"
      3: "Changes Requested"
```

### 9.2. Template Application

> Requirements: POC_1-2, POC_1-1.4

`glow project init --template=solo-mvp` - Initialize project with template.

`glow project migrate --from=solo-mvp --to=small-team-agile` - Migrate between templates.
