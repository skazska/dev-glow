# Development Process Model

[Common definitions](definitions.md)

- **Product Quality**: Resolved, Contained, Unresolved
- **Product Costs**: Optimal, Expensive, Overwhelming
- **Product Size**: Tiny, Small, Medium, Large

## Use Cases and Scenarios

### Solo Development

#### Solo Work Modes

- Freelance
- Hobby
- Startup MVP
- Research POC

The defining characteristic is that the developer works either independently or for another party. Let us define these work modes:

- Self-employed
- Employed

#### Solo Work Costs Clarification

For a solo developer, work costs primarily consist of personal time and effort invested in the project.

#### Solo Product Quality Clarification

From a solo developer's perspective, product quality is the subjective feeling of satisfaction with the created product, based on personal standards and expectations complemented by external feedback, if any.

#### Solo Scenarios

[process model scenarios as is](process_model/solo_dev_scenarios_as_is_analysis.md)

#### Solo Developer Challenges

In Self-Employed mode, a solo developer works independently on their own product. They bear all responsibilities, risks, and rewards. The primary challenges revolve around managing all aspects of the project alone, from ideation to deployment.

In Employed mode, a solo developer works within a larger organization or team. They may in some cases have access to employer-provided resources, tools, and support. The primary challenges involve aligning with organizational goals, processes, and collaboration with other team members.

### Small Team Development

A small group of developers collaborates on the project, sharing responsibilities for design, coding, testing, and deployment.

#### Small Team Work Modes

- Startup
- IT outsourcing
- Team within a larger organization

#### Small Team Work Costs Clarification

For small teams, work costs include not only individuals' time and effort but also coordination overhead, communication efforts, and potential delays due to dependencies among team members. These typically translate to financial costs, as team members are often compensated for their work along with other operational expenses.

#### Small Team Product Quality Clarification

Unlike solo development, instead of one developer's feeling, there might be different approaches to measure quality:

- Team consensus
- Client/community feedback analysis
- Market performance
- Stakeholder reviews

#### Scenarios

[process model scenarios table](process_model/small_team_scenarios_as_is_analysis.md)

### Small Team Challenges

in Startup mode, a small team (e.g., 2-5 people) works on their own product. They share the vision, risks, and rewards. The primary challenges shift from *doing* the work to *coordinating* the work and maintaining a *shared understanding*.

in Outsourced mode, a small team (e.g., 2-5 people) works on a product for an external client. They must balance the client's requirements, expectations, and communication with their own internal processes and collaboration. The primary challenges involve managing client relationships, scope changes, and delivering value within constraints.

### Open Collaborations

We consider open collaborations as a hybrid approach combining elements of solo and small team development, where individuals or small teams contribute to a larger project, typically in an open-source or community-driven context.

#### Open Collaboration Work Modes

- Open-source
- Community-driven
- Foundation-backed
- Corporate-sponsored

From a project perspective, the work mode defines the governance and sustainability model:

- Community-governed: Project decisions made by contributor consensus or a maintainer group.
- Foundation-backed: Project hosted under a foundation with formal governance.
- Corporate-sponsored: Project primarily driven by one or more companies.
- Hybrid: A combination of the above.

#### Open Collaboration Work Costs Clarification

Work costs in open collaborations are distributed among contributors, with varying levels of investment depending on their role and commitment.

#### Open Collaboration Product Quality Clarification

Quality from the project's perspective is measured by its health and sustainability:

- Community activity level and growth.
- Responsiveness to issues and contributions.
- Release consistency and maintenance cadence.
- Documentation completeness and currency.
- Test coverage and CI/CD maturity.

#### Open Collaboration Scenarios

- **Core Maintainer Scenario:**
  - A small group of maintainers manages the project direction, reviews contributions, and ensures release quality.
  - *Challenge:* Balancing own development work with community management and review burden.
  - *Assistance:* AI-driven triage, automated code review, toxicity detection in comments.

- **Casual Contributor Scenario:**
  - An individual contributes a fix or feature to a project they use.
  - *Challenge:* Understanding the codebase, setting up the environment, adhering to contribution guidelines.
  - *Assistance:* Automated onboarding, "good first issue" identification, devcontainer configurations.

- **Corporate-Backed Open Source:**
  - A company releases a project as open source but retains control.
  - *Challenge:* Balancing community interests with corporate roadmap.
  - *Assistance:* Roadmap visualization, feedback aggregation and sentiment analysis.

#### Open Collaboration Challenges

- **Governance and Decision Making:** Reaching consensus in a diverse community can be slow and contentious.
- **Quality Control:** Ensuring contributions from unknown sources meet project standards without discouraging contributors.
- **Sustainability and Burnout:** Maintainers often work unpaid and face high demands, leading to burnout.
- **Security and Trust:** Preventing malicious code injection and managing supply chain risks.
- **Fragmented Communication:** Discussions happen across issues, PRs, chats, and forums, making it hard to track decisions.

### Enterprise Process Scenario Reference

#### Enterprise Work Modes

- In-house development
- Dedicated teams
- Hybrid models

#### Enterprise Work Costs Clarification

In enterprise settings, work costs encompass not only direct labor costs but also infrastructure, tooling, compliance, training, and overhead associated with larger organizational structures.

#### Enterprise Product Quality Clarification

In enterprises, product quality is often measured against formal standards and benchmarks, including regulatory compliance, performance metrics, user satisfaction scores, and market competitiveness.

#### Enterprise Scenarios

[process model scenarios as reference](./process_model/enterprise_process_scenario_as_reference.md))

#### Enterprise Challenges

## Problem Introduction

In the context of software development processes, several limiting factors can reduce process effectiveness and product quality:

1. **Drifting or deviation from initial goals, values, and vision**, caused by:
    - Lack of clarity or understanding of goals, vision, or initial requirements among team members.
        - Poor communication or documentation of initial goals and requirements.
        - Misalignment of goals and vision with requirements.
    - Changing requirements or priorities over time.
        - Following trends or external pressures without critical evaluation and reconciliation with initial goals.
        - Scope creep due to a lack of disciplined change management.
    - Lack of tracking and measuring progress and alignment with initial goals and requirements.
        - Insufficient feedback loops to validate ongoing work against initial goals and requirements.
    - Poor semantic connections between analysis, design, task breakdown, specification, implementation, testing, and validation phases.
        - Treating phases as isolated silos rather than interconnected components of a unified process.
        - Lack of referencing and reasoning in information artifacts to maintain context and traceability.
    - Inadequate retrospectives or reviews to identify and address deviations from initial goals.

2. **Skill gaps or behavioral issues among team members**, caused by:
    - Lack of a considerable, clear, informative, and simple (not overcomplicated) policy or guiding process to follow.
    - Lack of training or mentoring to develop and maintain necessary skills and mindset.
    - Lack of a clear and considerable mechanism to enforce adherence to process and standards.
    - Lack of accountability and ownership for quality and outcomes.
    - Poor team dynamics and collaboration practices, communication breakdowns, conflicts, and low trust.

3. **Pressure related to deadlines, budget, and resources**, caused by:
    - Lack of planning and estimation capabilities.
        - Lack of reliable support (such as data, experience, or expertise).
    - Incorrect planning control model.
        - Unnecessary restrictions and requirements.
        - Unluckily chosen planning approach.
    - Ignorance of efforts and tasks important for estimation, risk management, and quality assurance.

4. **Poor quality assurance practices**, caused by:
    - Lack of clear quality standards and criteria.
    - Insufficient testing and validation coverage.
    - Lack of automated testing and continuous integration.
    - Poor defect tracking and resolution processes.
    - Inadequate code reviews and peer feedback mechanisms.

## Expected scenarios inference

Provided enterprise scenarios are not suitable for small teams and solo developers due to their complexity, resource intensiveness, and costliness, but are quality-focused and progress-forcing.

Small teams and solo developers can-not afford the overhead of enterprise processes, in contrary ideal solution would be an absence of process at all, but that leads to low quality and progress issues.

So a golden middle is required.

**From cost, resource, and complexity th solution lies in**:

1. process configurability and adaptability to different team sizes, project types, and contexts.
2. Usage simplicity and actions reasomnability to minimize learning curve and cognitive load.
3. AI-driven automation and augmentation to reduce manual effort and overhead.

**From quality and progress perspective the solution lies in**:

1. Clear and concise process guidance and best practices tailored for small teams and solo developers. (configuration templates?)
2. Context awareness and semantic connection across all development phases and artifacts.
3. AI-driven quality assurance and continuous improvement mechanisms.

### Process configurability and adaptability

Process is a cycle.
Cycle iteration is a set of steps.
Step can be atomic or be a process itself.
Set of steps can be linked by precedence, dependency, or other relations.
Step links can be strong (precedence, dependency) or weak (relations which does-not restrict execution).
Strong links define process structure and can only refer steps within the same process iteration.
Steps of iteration can not form closed loops, if they do - they are sub-process.
Links can be conditional, defining different paths through the process based on context or outcomes of previous steps.

These allow to build a process tree for any desirable complexity level.

Step features:

1. Classification: steps must be classifiable.
2. Configuration: step characteristics must be configurable.
   - characteristics: e.g.: id, name, description, estimates, metrics, etc.
   - parameters: inputs, outputs, attributes
   - context structure
   - process structure (for process steps):

Links features:

1. Classification: links must be classifiable.
2. Configuration: link characteristics must be configurable.
   - characteristics: id, name, description, type, condition, isDependecy, isPrecedency, etc...

Step paramenters features:

1. Classification: parameters must be classifiable.
2. Configuration: parameter characteristics must be configurable.
   - characteristics: id, name, description, type, validators

Step parameter validators features:

1. Parameter type: validators must be type-specific.
2. Validator types: validators must support different validation types (e.g., range, format, size, mime-type, regex).

### AI-driven automation and augmentation

AI agents can act as virtual team members, handling repetitive or cognitive-heavy tasks to reduce the burden on human developers.

- **Generative Tasks:**
  - **Code Generation:** Boilerplate, unit tests, documentation comments.
  - **Content Generation:** Release notes, changelogs, user documentation, marketing copy.
- **Analytical Tasks:**
  - **Code Review:** Detecting bugs, security vulnerabilities, and style violations.
  - **Requirement Analysis:** Identifying ambiguities, conflicts, and missing details.
  - **Root Cause Analysis:** Analyzing logs and stack traces to pinpoint issues.
- **Process Management:**
  - **Task Management:** Breaking down features into tasks, estimating effort, and suggesting assignments.
  - **Context Management:** Summarizing discussions, linking related artifacts, and maintaining a semantic knowledge base.

### Usage simplicity and reasonableness of actions

To ensure the process is an enabler rather than a blocker, it must be designed with the user's cognitive load in mind.

- **Minimal Friction:** Process steps should be automated where possible. Manual inputs should be requested only when necessary and should be brief.
- **Just-in-Time Guidance:** Instructions and requirements should be presented exactly when they are needed, not in a massive upfront manual.
- **Reasonable Defaults:** The system should provide sensible defaults for all configurations, allowing users to start immediately and customize later.
- **Actionable Feedback:** When a process step fails or a standard is not met, the system should provide clear, actionable advice on how to fix it, rather than just an error message.
- **Progressive Disclosure:** The process should start simple and reveal complexity only as the project grows or specific needs arise.

### Template-based process configuration

To cater to different project types and team sizes without overwhelming configuration, the system should use a template-based approach.

- **Pre-defined Templates:**
  - **Solo MVP:** Minimal process, focus on speed, basic CI/CD.
  - **Small Team Agile:** Scrum/Kanban basics, code review enforcement, automated testing.
  - **High Assurance:** Strict quality gates, comprehensive documentation, security scanning (for regulated domains).
- **Composition:** Templates can be composed of "Process Modules" (e.g., "GitHub Flow Module", "Release Notes Module").
- **Customization:** Users can override specific parameters or swap out modules within a template.
- **Evolution:** A project can switch templates (e.g., from Solo MVP to Small Team Agile) as it matures, with the system handling the migration of process artifacts.

### Context maintenance and semantic connection

Context is like a call stack in javascript.
For each step execution a context frame is created in the context stack.

Frame contains:

1. Step parameters (inputs, outputs, attributes)
2. Step metadata (id, name, description, type, timestamps, etc.)
3. Additional step artifacts.

When a step is executed, it can access its own frame and all previous frames in the stack.

This allows steps to maintain context and semantic connections across different phases and artifacts of the development process.

#### Context quality

Context quality assessment can be approached in terms of:

1. is context complete? (complete, missing, not enough)
2. is context consistent? (consistent, inconsistent, contradictory)
3. is context solid? (solid, weak, fragmented)
