# Phase 2: Feasibility Research Plan (dev-glow)

Confidence: 9/10

Date: November 2025

Status: Draft for execution

---

## Purpose

Validate technical and practical feasibility of dev-glow’s core thesis: an AI-native, local-first development cycle manager that uses structured, evolving project context to automate both tasks and process management for solo developers and small teams.

This plan operationalizes the priorities emerging from Phase 1 results and the research questions in `questions.md`.

Inputs:

- Phase 1.1 Executive Summary (`phase-1-1-summary.md`)
- Phase 1.2 User Research & Validation (`phase-1-2-FINAL-REPORT.md`)
- Research questions (`questions.md`)

---

## Scope and Workstreams

Phase 2 focuses on four tightly-coupled workstreams, mapped to existing subfolders:

- AI Context Extraction Algorithm (CEMM) → `2-feasibility/CEMM/`
- Process Model (Value → Feature → Requirement → Task) (PDM) → `2-feasibility/DPM/`
- MVP Specifications (MVP) → `2-feasibility/POC-1/`
- Data Model & Architecture (HL Architecture) → `2-feasibility/HL_Architecture/`

Each workstream defines questions, methods, deliverables, success criteria, and risks.

---

## 1) AI Context Extraction Algorithm (CEMM)

Objective: Design and validate a low-friction, token-efficient pipeline that assembles “Minimum Effective Context” (MEC) for a task, with human control and traceability.

Research questions:

- What is the MEC for common task types (bug, small feature, refactor, doc update)?
- How to extract, rank, and package context from repo (code, docs, history) + project model?
- What blend of static analysis, heuristics, embeddings, and LLM is cost-effective and reliable?
- How to keep context concise, complete, and up-to-date without bloat?
- What oversight controls are required (accept/reject, diffs, audit trail)?

Approach:

- Define context schema: Project-level (values, goals), Feature/Requirement-level, Task-level, Code-level references, Decisions, Tests.
- Indexing pipeline (local-first):
  - Parse repo metadata: file tree, languages, module boundaries.
  - Static cues: filenames, imports, symbol xrefs (ctags/LSP), test-to-code links.
  - Light embeddings: chunk code/docs; store vector index locally (e.g., sqlite + FAISS-like via optional lib; start with naive cosine on small datasets).
  - Heuristics for task type: map task metadata to candidate regions (paths, tags, changed files, recent commits).
- Packaging algorithm for MEC:
  - Start from task anchors (paths, symbols, requirement links) → expand neighborhood (callers/callees, tests, related docs).
  - Deduplicate and compress (summarize with LLM when beneficial) with explicit token budget.
  - Produce “context pack” (markdown/JSON) with provenance links, sizes, and costs.
- Human-in-the-loop controls: preview context pack, toggle inclusions, persist rationale.

Experiments (prototype spikes):

- Run on 2-3 open-source repos of varying sizes to measure:
  - Relevance@K (human-rated), coverage vs. token budget.
  - Latency (target: < 2 min end-to-end on mid repo).
  - Token cost per task (target: < $0.10/task on typical prompts; tunable).
- Compare retrieval strategies: heuristics-only vs. embeddings-assisted vs. AST+heuristics.
- Evaluate summarization step impact on accuracy vs. cost.

Deliverables:

- CEMM design doc with diagrams and data flow.
- Local prototype: `glow ctx pack` producing a MEC bundle from a task spec.
- Evaluation log with metrics, costs, sample outputs, and UX notes.

Success criteria:

- Human evaluators rate ≥ 80% of context packs as “sufficient” for task start.
- Token budget respected (configurable caps; default < 8k prompt tokens).
- End-to-end time < 2 minutes on a medium repo; works offline except for LLM calls.

Risks & mitigations:

- Hallucinated or irrelevant context → emphasize provenance links, rank by multi-signal, human toggle step.
- Token/cost blow-up → strict budgets, summarization thresholds, task-type presets.
- Language/tooling variance → start with common stacks (TS/JS, Python), extensible analyzers.

---

## 2) Process Model: Value → Feature → Requirement → Task (PDM)

Objective: Define the minimal viable process that is structured, flexible, and automation-friendly.

Definitions (working):

- Value: Outcome/goal tied to user or business impact.
- Feature: Capability that realizes value; may span iterations.
- Requirement: Verifiable statement of behavior; links to DoR/DoD.
- Task: Executable unit with owner, scope, and acceptance criteria.

Artifacts & state:

- Each entity has:
  - ID, title, description, status, links (up/down/sideways), DoR/DoD.
  - Optional tags: area, component, risk.
- State machines (simple, per-entity): Backlog → In Progress → Review → Done (plus Blocked/On Hold).
- Automation points: auto status bumps from PRs/tests; DoD validation checks; stale detection; suggested next steps.

Deliverables:

- PDM spec with examples and diagrams.
- Templates (markdown/YAML) for Value, Feature, Requirement, Task.
- DoR/DoD checklists per level; default ruleset + project overrides.
- Example walkthrough: idea → feature → requirements → tasks → shipped.

Success criteria:

- Solo dev can onboard a project in < 5 minutes using templates.
- Team can explain states and transitions in < 2 minutes (low cognitive load).
- At least 3 automation hooks that remove manual status changes.

Risks & mitigations:

- Over-rigidity → allow free-form notes and bypass with warnings; make “right” path easiest.
- Jargon fatigue → developer-centric language and examples.

---

## 3) MVP Specifications (CLI-first)

Objective: Define a shippable v1 in 8–12 weeks that addresses validated pains with minimal surface area.

MVP core features:

- Local-first project scaffolding: `glow init` creates structure in repo (files + templates).
- CLI task/requirement management: `glow value|feature|req|task add/list/show/edit/link`.
- AI-assisted breakdown: `glow plan` converts Feature → Requirements → Tasks using CEMM context.
- Context pack generation: `glow ctx pack --task <id>` preview and edit.
- Lightweight automation:
  - Status updates from Git commits/PR titles referencing IDs.
  - DoD checks via simple test hooks and checklists.
  - Progress summary `glow status` (per entity and overall).

Non-go for MVP:

- Full web UI (optional later dashboard).
- Deep third-party integrations (Jira/GitLab) – spec only; stubs if trivial.
- Complex workflow engines.

CLI UX principles:

- Plain, fast, discoverable (`--help` rich output; examples).
- Dry-run by default for destructive actions; `--yes` to apply.
- Clear, concise output with links to files.

Deliverables:

- MVP spec doc (commands, flags, examples, exit codes).
- Minimal user guide and quickstart.
- Backlog with 8–12 week roadmap and milestones.

Success criteria:

- New user completes init → first task → AI-assisted breakdown → context pack → status in ≤ 15 minutes.
- Positive UX feedback from 3 test users (solo + small team rep).

Risks & mitigations:

- Scope creep → strict backlog, weekly scope review.
- AI variance → deterministic presets, retry/backoff; human acceptance gate.

---

## 4) Data Model & Architecture (Local-first)

Objective: Specify entity schema, file layout, linking strategy, and high-level system architecture.

Data model (initial):

- Files in repo under `.glow/` (hidden) or `glow/` (visible) – choose visible for transparency in MVP.
- Entities:
  - Value: `glow/values/<id>.md`
  - Feature: `glow/features/<id>.md`
  - Requirement: `glow/requirements/<id>.md`
  - Task: `glow/tasks/<id>.md`
- IDs are short, human-friendly slugs; cross-links via frontmatter YAML and backlinks.
- Code references stored as file:line or symbol anchors; PR/commit links optional.
- Context packs stored under `glow/context/<task-id>/pack.md` (+ metadata JSON).

Architecture (high level):

- CLI app (Go/TS/Python TBD; start with Python for rapid prototyping) with modules:
  - Repo scanner/indexer (static analysis; pluggable analyzers).
  - Retriever/packager (CEMM).
  - Process engine (PDM rules, DoR/DoD checks).
  - Storage adapter (file IO + simple sqlite index optional).
  - LLM adapter (provider-agnostic; token accounting).
- Event hooks: git hooks optional; simple watchers for file changes.
- Security & privacy: local-by-default; explicit opt-in for remote calls; redact secrets.

Deliverables:

- Schema spec (frontmatter fields, link conventions, IDs).
- File layout and naming conventions.
- Architecture diagram + module boundaries + extension points.

Success criteria:

- Greppable, reviewable files; diffs make sense in PRs.
- Adding an entity or link is a single CLI command and one commit.
- Index can be regenerated deterministically from files.

Risks & mitigations:

- Link rot → nightly lint `glow check` for broken links.
- Performance on large repos → lazy indexing, per-task focus, cache.

---

## Timeline (4 weeks)

Week 1

- Finalize PDM templates and DoR/DoD.
- Spike indexer (file tree, symbol tags) and task-type heuristics.
- Draft data schema and file layout; create sample repo.

Week 2

- CEMM v0: retrieval + naive packaging; manual review flow.
- Define CLI surface for init, entity CRUD, and ctx pack.
- Start MVP spec and quickstart docs.

Week 3

- CEMM v1: embeddings-assisted retrieval and summarization thresholds.
- Implement DoD checks and status automation from commits.
- Usability pass on CLI; dry-runs and helpful output.

Week 4

- Evaluation: measure relevance, latency, token costs on 2–3 repos.
- Polish docs; produce feasibility report and demos.
- MVP backlog and 8–12 week delivery plan locked.

---

## Evaluation & Metrics

- Context sufficiency (human-rated): target ≥ 80%.
- Token cost per task: default target ≤ $0.10 (configurable; report actuals).
- End-to-end latency per context pack: ≤ 2 minutes on medium repo.
- Setup time: ≤ 5 minutes to init and create first task.
- Automation impact: reduce manual status edits by ≥ 50% on test runs.

---

## Risks & Mitigations (Summary)

- Context bloat → strict budgets, summarization, MEC discipline.
- Developer trust → transparent previews, provenance, easy accept/reject.
- Model variance → constrain prompts, use templates, retries; cache good outputs.
- Integration pressure → provide clean file/CLI surfaces; defer deep integrations to next phase.

---

## Phase 2 Deliverables (Artifacts)

- CEMM design + prototype + evaluation log.
- PDM spec + templates + example walkthrough.
- MVP spec + CLI command map + quickstart.
- Data schema + file layout + architecture diagram.
- Feasibility Report summarizing learnings, trade-offs, and go-forward plan.

---

## Notes on Alignment with Phase 1 Findings

- Prioritizes context management as the core bottleneck (shared by both personas).
- Emphasizes CLI-first, local-first, minimal setup.
- Provides control over context extraction and evolution (key user requirement).
- Focuses on lightweight automation that removes repetitive mechanics without rigidity.
- Defers heavy integrations/UI until value is proven.

---

## Next Steps (Kickoff Checklist)

- Create sample repo for experiments; seed with varied stacks.
- Stand up minimal CLI skeleton and indexing spike.
- Draft PDM templates and run through 1–2 dry runs with real tasks.
- Define evaluation rubric and recruit 3 evaluators (solo + small team rep).
