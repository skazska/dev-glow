# Current status

Project idea: `rnd/IDEA.md` - human composed and refined AI assisted and reviewed.
Research questions and plan is in `rnd/research/questions.md` - AI generated, then carefully reviewed and refined using different AI models review and comparisons.
Research plan - AI generated.

## Phase 1 (market & competitive landscape)

competitive analysis conducted by AI.

- `rnd/research/1-market_competitive_landscape/1/phase-1-1-competitive-analysis.md`
- `rnd/research/1-market_competitive_landscape/1/phase-1-1-summary.md`

user research & validation:

Internet research conducted by AI: `rnd/research/1-market_competitive_landscape/2/phase-1-2-internet-research.md`

Interview research conducted by AI with human answering Interview questions: `rnd/research/1-market_competitive_landscape/2/phase-1-2-interview-framework.md`
Results of interview research: `rnd/research/1-market_competitive_landscape/2/phase-1-2-FINAL-REPORT.md`

## Phase 2 (feasibility research)

There is 3 research plans suggested by different AI models:

- `rnd/research/2-feasibility/phase-2-feasibility-research-plan_claude.md`
- `rnd/research/2-feasibility/phase-2-feasibility-research-plan_gemini.md`
- `rnd/research/2-feasibility/phase-2-feasibility-research-plan_gpt.md`

## Considerations

As ideas originator and project initiator, I have following key considerations on Pase 2 feasibility research plans suggested by different AI models.

### Focuses switched

Originally in [Expectations and vision](../../IDEA.md#expectations-and-vision) document:

1. The First focus of dev-glow project is to help solo developers and small teams to manage their development process with simple solution as development cycle management essential to succsess and quality of product - regardless of AI assistance level, but stornger use of AI stronger this dependency.
2. The Second focus is increasing usage of AI assistance in development process which has big potential to grow and improve developer productivity and product quality and thus, need in finding better ways to integrate AI assistance into development workflows.
3. Context management (for dev project and tasks) was called out as important thing, which looks underserved in current tools landscape and has big potential to improve development process management, regardles of AI assistance, but especially with AI assistance.

However, in all 3 feasibility research plans suggested by different AI models, priorities seems to be not aligned with above vision:

1. All 3 plans put strong focus on AI assistance at first place:
   - first of all context preparation for AI assistance,
   - LLM models and usage patterns,
   - AI assistance integration into development workflows.
   - all sounding hardcore AI agent develop research.
2. All 3 plans suggest to research Context Extraction and Process Model separately.

Of course AI assistance is very important and has big potential and definitely must be considered in dev-glow project, even maybe including techniques as RAG, fine-tuning, custom models, etc. and agents. But not in first place.

Of course bussiness metrics like LLM usage cost optimization are important too from bussiness perspective and must be considered. But not in first place.

### Some insights and expectations

I beleive (keeping in mind irrelevance of AI assistance level):

One way to help achieve context consistensy throughout product creation process is to make efort to have it, doing right things. So first thing to research is: How to do Development Process Management from Product Creation Process Context Consistency perspective.

Let remember Originally in [Expectations and vision](../../IDEA.md#manage-contexts-for-ai-assistance):

```markdown
#### context evolution

Context is crucial to perform both human and AI work effectively.
Context birth happens before projects as idea and goals is defined and then matured via research and refined into features and requirements.
Context growth happens as development and delivery iterations progress as designs, code, tests, and technical documentation is created.
Crucial part of context evolution is keeping consistency and relevance of context to current state of the project and its goals.
To have effective AI assistance, context needs also keep clarity, conciseness, structure, relevance and concentration.

#### context extraction

Any task needs carefully prepared context describing what needs to be done, why, and how, still being part of overall project context.
Context extraction is the process of gathering and preparing relevant information from the overall project context to provide to AI for performing specific tasks.
This involves identifying the key elements of the project that are pertinent to the task at hand, and organizing this information in a way that is clear and concise for the AI to understand and utilize effectively.
Context preparation might become a complex task itself, so tool should provide help in that.
```

[insights from interviews in Phase 1](../1-market_competitive_landscape/2/phase-1-2-interview-framework.md#c4-ai-integration--potential-use-cases)

```markdown
> 1. Context for a task must be relevant to that task (regardless of AI involvement: no AI, AI-Assists, AI-Agent is assigned as executor). Which means task context must be:
> - Concise - clear short
> - Complete - sufficient to do task with quality
> - Up-to-date - consistent with current state of project
> 2. To achieve that for every task of project, there must be a process of context management:
> - Project context structure and evolution - is a key. It is a structured set of information linking, and describing all aspects of project, like: foundation documentation (ideas, goals, values etc.), analytical documentation (requirements, stories, albums, expectations etc), design documents (architecture, tools, infrastructure, decisions), development documentation (code organization and standards, practices, patterns, code comments, etc.), testing documentation (test plans, test cases, test results, etc.), deployment documentation (deployment plans, environments, configurations, etc.), progress log (index of tasks, requirement coverage, test coverage, statuses, etc.).
> - All Project context items must be interconnected
> - Project context must be updated with each task completion. There might also be special tasks for context maintenance and evolution.
> 3. Code itself must contatin references to relevant parts of project context (requirements, decisions, designs, tests, etc.) to ensure AI can access full context when needed.
> 4. Task context must be created as set of parts extracted from Project context in addition to task-specific information. There might be tasks which are repetitive or routine or rule based, so no task-specific information is needed to be provided by human.
```

So i expect focus and research-develop cycles:

1. POC-1: Research and POC of Development Process Management from Product Creation Process Context Consistency perspective:
   - Development Process Model (Value → Feature → Requirement → Task → Review → Delivery)
   - Context as backbone of Development Process Management
   - Context Evolution Model
   - Context Extraction Model
   - How to ensure Context Consistency throughout Product Creation Process via Development Process Management
   - Local file-based Data Model supporting Development Process Management and Context Management
   - Light AI Assistance Use Cases: via Copilot and MCP integration
   - CLI and MCP tools for above.
2. POC-2: Research and POC of deep AI Assistance Integration into dev-glow product:
   - AI Assistance Use Cases: AI-Assists and AI-Agent
   - AI Context Extraction techniques and algorithms
   - LLM models and usage patterns
   - AI Assistance Integration into Development Workflows
   - Advanced AI Assistance Cost Optimization techniques
3. High-level Architecture and MVP Specifications - to be defined during POC-1 and POC-2, iteratively refined.