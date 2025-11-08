# Phase 2 Feasibility Plan Comparison

Date: 2025-11-06
Owner: dev-glow RnD
Status: Draft for review

---

## Context and Inputs

This document compares three Phase 2 feasibility research plans for dev-glow against the idea, research questions, and Phase 1 findings.

Inputs considered:

- Vision and constraints: `rnd/IDEA.md`
- Research questions and plan: `rnd/research/questions.md`
- Phase 1 outcomes: `phase-1-1-summary.md`, `phase-1-2-FINAL-REPORT.md`
- Plans compared:
  - `phase-2-feasibility-research-plan_gpt.md`
  - `phase-2-feasibility-research-plan_claude.md`
  - `phase-2-feasibility-research-plan_gemini.md`

Evaluation lenses (derived from Phase 1 pains and research questions):

- Alignment with personas and pains (solo developer: simplicity/CLI/local-first; team lead: concise, up-to-date, interconnected context)
- Actionability and clarity (explicit deliverables, commands, prototypes, acceptance criteria)
- AI context extraction design (MEC definition, retrieval strategy, evolution rules, human-in-the-loop controls)
- Process model structure (Value → Feature → Requirement → Task) and automation opportunities
- Data model and local-first Git-friendly representation
- MVP scope, constraints, and setup time (< 5 minutes)
- Cost and token budgets (targets, measurement)
- Timeline and success criteria

---

## Executive Summary (Verdict)

- All three plans are strong and broadly aligned; none are mutually exclusive.
- Best-of blend:
  - Use Claude’s structure, explicit success criteria, and research workstreams.
  - Use GPT’s concrete CLI/MVP surface and prototype spikes (e.g., `glow ctx pack`, dry-run defaults, exit codes).
  - Use Gemini’s emphasis on context evolution rules and human-in-the-loop control UX.
- Recommended: Adopt a hybrid plan below with 4-week arc (feasible) and clear acceptance gates mapped to Phase 1 pains and cost caps.

---

## Side-by-Side Highlights

### Scope & Structure

- GPT: Very actionable; CLI-first MVP, concrete commands, prototype callouts; 4-week plan with weekly goals.
- Claude: Comprehensive research framing with clear deliverables per folder, time allocation per area, quality gates, and success metrics.
- Gemini: Concise, pillar-based plan focused on algorithms, process backbone, MVP scope, and blueprint architecture.

### Alignment with Phase 1 Pains

- Solo developer (lightweight, minimal setup, local-first, CLI):
  - GPT: Strong (explicit CLI design, 5–15 minute journey, dry-run safety).
  - Claude: Strong (local-first emphasis and clear templates); slightly more research-heavy.
  - Gemini: Strong (control/observability over context, CLI hints); tactical detail less explicit than GPT.

- Team lead (concise, complete, interconnected context, observability):
  - Claude: Strong (success criteria, token cost, structure, integration hooks).
  - GPT: Strong (context pack with provenance, budgets, performance targets).
  - Gemini: Strong (context evolution rules and human review flows front-and-center).

### AI Context Extraction (CEMM)

- GPT: Defines MEC, retrieval blending (heuristics/AST/embeddings), packaging to a reviewable context pack with provenance; performance and token targets.
- Claude: Full research treatment (taxonomy, algorithms, token analysis), explicit deliverable to `CEMM/` and evaluation plan.
- Gemini: Focuses on algorithm design + evolution rules + user control mockups (great for usability and governance).

### Process Model (PDM)

- GPT: Minimal viable model with automation points and DoD; templates and examples.
- Claude: Well-structured stage definitions, transitions, Git integration ideas; good for teams.
- Gemini: ERD/state machines + file-based schema proposal; “make right actions easy” principle called out.

### Data Model & Architecture

- GPT: Clear local-first file layout (`glow/values|features|requirements|tasks|context/...`), IDs, backlinks, module boundaries; provider-agnostic LLM adapter.
- Claude: Full architecture proposal coverage with stack options and extensibility considerations.
- Gemini: High-level blueprint with emphasis on file-based and Git-friendly structure.

### MVP Definition & Setup Experience

- GPT: Most precise (CLI surface, flags, success path, non-go items, 8–12 week delivery stance).
- Claude: MVP coverage is good with prioritization and guide, though CLI surface is less detailed.
- Gemini: Prioritizes MVP must-haves; leaves details to discovery (good for early design, needs concretization).

### Cost/Token Budgets and Success Criteria

- GPT: Targets (<$0.10/task for prompt, <8k tokens, <2 min end-to-end) and human-rated sufficiency.
- Claude: Targets at project-level (<$10/user/month), token analysis and quality gates.
- Gemini: Confidence and focus present; quantitative targets are lighter (can inherit GPT/Claude targets).

### Risks & Mitigations

- GPT: Calls out hallucination, token blow-up, variance; includes mitigations (budgets, provenance, human gate).
- Claude: Balanced risks across feasibility, user fit, simplicity, cost.
- Gemini: Emphasizes control/observability to mitigate over-automation concerns.

---

## Strengths by Plan

- GPT
  - Most actionable CLI and prototype plan; immediate developer value.
  - Concrete performance/cost budgets; provenance and human review built-in.
  - Tight 4-week plan with weekly outcomes.

- Claude
  - Clear workstream boundaries, research rigor, and folder-scoped deliverables.
  - Explicit success criteria and timeboxing.
  - Balanced between research depth and practicality; good for future team scaling.

- Gemini
  - User control and context evolution rules are first-class.
  - Clear pillars that map well to core thesis; crisp deliverables list.
  - Keeps human readability vs. AI structure front-and-center.

---

## Gaps and Watchouts

- GPT
  - Could over-index on building before rounding out research taxonomy.
  - Less attention to formal ERDs/state machines than Claude.

- Claude
  - Risk of analysis overhead unless paired with a working prototype cadence.
  - CLI surface and user journey less concrete than GPT.

- Gemini
  - Leaves some metrics and budgets implicit; needs explicit acceptance gates.
  - Architecture choices and stack selection less prescriptive.

---

## Recommended Hybrid Plan (Adoptable Now)

Use Claude’s structure, GPT’s CLI/prototype concreteness, and Gemini’s governance/usability focus.

Workstreams and outputs (folder-aligned):

1. CEMM → `rnd/research/2-feasibility/CEMM/`

- Outputs:
  - CEMM design doc (Claude baseline + Gemini evolution rules + GPT MEC packaging).
  - Prototype command: `glow ctx pack --task <id> [--budget <tokens>] [--dry-run]` generating:
    - `glow/context/<task-id>/pack.md` (human-readable) + `pack.json` (machine metadata).
  - Evaluation log with metrics: relevance@K (human-rated), token usage, latency, cost.
- Acceptance: ≥80% packs rated “sufficient”, default <8k prompt tokens, <2 min on medium repo, <$0.10/task prompt by default config.

1. PDM → `rnd/research/2-feasibility/DPM/`

- Outputs:
  - Minimal state machines: Backlog → In Progress → Review → Done (+Blocked/Hold).
  - Templates (frontmatter YAML + MD): Value, Feature, Requirement, Task; DoR/DoD checklists.
  - Auto hooks spec: status bumps from commit/PR references; DoD checks; stale detection.
- Acceptance: New project onboarding with templates ≤ 5 minutes; 3 automation hooks working in demo.

1. MVP → `rnd/research/2-feasibility/POC-1/`

- Outputs:
  - CLI spec (GPT as baseline):
    - `glow init` (creates `glow/` structure and templates)
    - `glow value|feature|req|task add|list|show|edit|link`
    - `glow plan` (AI-assisted breakdown)
    - `glow ctx pack --task <id>` (preview/edit)
    - `glow status` (rollup)
  - Quickstart (15-minute journey) + non-go items list.
- Acceptance: New user completes init → first task → AI breakdown → context pack → status ≤ 15 minutes.

1. HL Architecture → `rnd/research/2-feasibility/HL_Architecture/`

- Outputs:
  - File schema (visible `glow/` dir, frontmatter IDs, backlinks, symbol/code refs proto).
  - Component diagram: CLI, indexer, CEMM, LLM adapter, Git integration.
  - Stack pick for MVP (Python + Typer/Rich; or Go/TS if preferred) with pros/cons.
- Acceptance: Greppable files, deterministic re-index, minimal dependencies, provider-agnostic LLM client.

Guardrails and governance

- Human-in-the-loop by default: preview context pack diffs, toggle inclusions, record rationale.
- Strict budgets with overridable configs; summarize when over budget.
- Provenance everywhere (files/lines/symbols, commit/PR links when present).

---

## Timeline (4 Weeks)

Week 1

- PDM templates + DoR/DoD finalized.
- Indexer spike (file tree + language detection + symbol tags for chosen stacks).
- Data schema draft + sample `glow/` layout.

Week 2

- CEMM v0: heuristics retrieval + naive MEC packaging; human review flow.
- CLI surface defined; `glow init`, CRUD for entities, and `ctx pack` dry-run.
- MVP spec draft + quickstart.

Week 3

- CEMM v1: embeddings-assisted retrieval, summarization thresholds, token caps.
- DoD checks and status automation from commit/PR references.
- CLI UX polish (help, examples, dry-run by default, clear exit codes).

Week 4

- Evaluation on 2–3 repos: relevance@K, latency, tokens, cost.
- Feasibility report + demos; lock MVP backlog and 8–12 week delivery plan.

---

## Metrics & Acceptance Criteria (Quality Gates)

- Context sufficiency (human-rated): ≥80% sufficient.
- Token budget: default <8k prompt tokens per task context; configurable.
- Latency: <2 minutes end-to-end MEC generation on medium repo.
- Cost: target <$0.10 per task prompt; budget < $10/user/month for normal usage.
- Setup: `glow init` creates structure in ≤5 minutes.
- Automation: ≥3 hooks functional (status from commits/PRs, DoD checks, stale detection).

---

## Immediate Next Actions

- Confirm stack for MVP CLI (recommend Python + Typer for speed; revisit after spike).
- Stand up `glow/` sample structure and minimal `ctx pack` prototype interface (no LLM call yet, stub outputs plus provenance capture).
- Draft PDM templates and DoR/DoD checklists; wire into `glow init`.
- Define evaluation rubric form for human rating of context packs.

---

## Notes on Traceability to Phase 1

- Solo dev pains addressed via: 5-minute init, CLI-first, MEC w/ provenance, human toggles, dry-run default.
- Team lead pains addressed via: concise/up-to-date context packs, DoD checks, status automation, Git-native linking.
- Competitive whitespace: AI-native context management + lightweight process + local-first integration preserved across all decisions.
