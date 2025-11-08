# Phase 2: Feasibility Research Plan

**Date**: November 2025
**Status**: Proposed
**Confidence**: 9/10

---

## 1. Overview

Following the strong market and user validation from Phase 1, Phase 2 shifts focus from "What?" and "Why?" to **"How?"**. The objective of this phase is to determine the technical and product feasibility of `dev-glow`.

This research will produce the core technical and product specifications required to build the MVP. It will address the most critical and innovative aspects of the project: AI-native context management and the lightweight process model.

## 2. Core Focus Areas

This research phase is structured around four key pillars:

1. **AI Context Extraction Algorithm Design**: Designing the "brain" of `dev-glow`.
2. **Process Model (Value → Feature → Requirement → Task)**: Defining the "backbone" of the workflow.
3. **MVP Specifications**: Scoping the first shippable version.
4. **Data Model & Architecture**: Outlining the "blueprint" for the system.

---

## 3. Research Activities & Deliverables

### 3.1. AI Context Extraction & Management

**Objective**: Design and prototype the core algorithms for extracting, structuring, and evolving project context. This must address the user need for both powerful automation and human oversight.

**Key Questions to Answer**:

- How can context be structured for optimal AI consumption *and* human readability (Markdown vs. YAML vs. JSON)?
- What is the "minimum effective context" for core tasks like bug fixing, feature development, and AI-assisted code generation?
- How can the system automatically evolve context as tasks are completed and code changes?
- How do we provide the control and observability over context extraction that users (specifically the solo developer) requested?

**Tasks & Experiments**:

1. **Technical Spike 1: Context Source Analysis**:
    - Prototype scripts to parse different sources:
        - Git history (commit messages, file changes).
        - Code files (function signatures, comments, dependencies).
        - Project documentation (`README.md`, `docs/`).
    - **Goal**: Identify reliable patterns for extracting meaningful data.
2. **Technical Spike 2: Context Structuring**:
    - Experiment with representing extracted context in different formats (e.g., a structured Markdown file with frontmatter, a `context.json` file).
    - **Goal**: Find the right balance between structure for AI and readability for humans.
3. **Design Task: Context Evolution Rules**:
    - Define a set of rules for how context is updated. For example: "On task completion, summarize the associated PR and update the feature's implementation notes."
    - **Goal**: Create a clear specification for keeping context up-to-date automatically.
4. **Design Task: User Control Mockups**:
    - Sketch CLI commands and/or simple UI for reviewing, editing, and approving changes to the project context.
    - **Goal**: Ensure the design aligns with the user feedback for "human-in-the-loop" oversight.

**Deliverable**: **`AI Context Management Specification`** - A document detailing the proposed algorithm, data structures, evolution rules, and user control mechanisms.

### 3.2. Process Model & Data Model

**Objective**: Formalize the lightweight `Value → Feature → Requirement → Task` process model and define the underlying data structures.

**Key Questions to Answer**:

- What are the essential properties, states (e.g., `todo`, `in-progress`, `done`), and relationships for each entity?
- How does the tool "drive the process" without being overly rigid, making the "right" actions easy (as per the team lead's suggestion)?
- How does this data model translate to a local, file-based system that integrates with Git?

**Tasks**:

1. **Design Task: Entity-Relationship Definition**:
    - For each entity (Value, Feature, Requirement, Task), define its attributes (e.g., `ID`, `title`, `status`, `description`, `owner`).
    - Map the relationships (e.g., a `Feature` has many `Requirements`).
2. **Design Task: Workflow Diagramming**:
    - Create state machine diagrams for each entity, showing the transitions and the triggers (manual or automated).
    - **Goal**: Visualize the entire project lifecycle from idea to completion.
3. **Design Task: File-Based Data Model**:
    - Propose a directory and file structure for storing the project data. Example: `/dev-glow/features/FEATURE-1.md`, `/dev-glow/tasks/TASK-123.md`.
    - Define how relationships are stored (e.g., using IDs in Markdown frontmatter).
    - **Goal**: Create a blueprint for a Git-friendly, human-readable project structure.

**Deliverable**: **`Process & Data Model Specification`** - A document containing ERDs, state diagrams, and the proposed file-based schema.

### 3.3. MVP Specifications

**Objective**: Define a tightly-scoped MVP that delivers immediate value to the solo developer persona while laying the groundwork for team features.

**Key Questions to Answer**:

- What is the absolute minimum set of features needed to validate the core loop (define task -> extract context -> assist -> complete task -> update context)?
- Which features from the user interviews are "V1" vs. "V2"? (Prioritizing CLI, AI task breakdown, and context extraction).
- What does the "5-minute setup" look like in practice?

**Tasks**:

1. **Scoping Task: Feature Prioritization**:
    - Use the MoSCoW method (Must-have, Should-have, Could-have, Won't-have) to categorize features based on Phase 1 research.
    - **Must-haves for MVP**:
        - CLI for project initialization (`glow init`).
        - CLI for managing tasks and requirements.
        - AI-assisted task breakdown.
        - Automated context extraction for a given task.
        - Process automation for status updates.
2. **Design Task: User Journey Mapping**:
    - Write a step-by-step narrative of a solo developer using the MVP to build a small feature.
    - **Goal**: Solidify the user experience and identify any gaps.

**Deliverable**: **`MVP Specification Document`** - A prioritized feature list and user journey map for the first version.

### 3.4. High-Level Architecture

**Objective**: Outline the technical architecture, confirming a local-first, CLI-driven approach.

**Key Questions to Answer**:

- What are the primary components of the application (e.g., CLI, context engine, Git wrapper, AI service client)?
- How do these components interact?
- What is the technology stack (e.g., Python/Typer for CLI, LangChain/LlamaIndex for AI)?

**Tasks**:

1. **Design Task: Component Diagram**:
    - Create a high-level block diagram showing the system's components and their interactions.
2. **Decision Task: Technology Stack Selection**:
    - Propose and justify the choice of programming language, libraries, and frameworks for the MVP.

**Deliverable**: **`Architecture Overview Document`** - A document with the component diagram and recommended technology stack.
