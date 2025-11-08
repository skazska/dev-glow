# Software development cycle management idea

Software development process management (SDPM) is the process of overseeing and controlling the various stages of software development to ensure that projects are completed on time, within budget, and to the required quality standards. It involves planning, organizing, directing, and monitoring the activities involved in software development. (source: [Wikipedia](https://en.wikipedia.org/wiki/Software_development_process_management))

## Existing solutions

There must be ton of existing solutions for this. One popular example is [Jira](https://www.atlassian.com/software/jira).
However, my personal needs as a developer trying to make my own product in the context of my feeling of zeitgeist are not adequately served by solutions i know of.

## Context

I am a solo developer working on my own product. Or I am a lead developer or team lead in a small team working on a product.
In both cases, I need development cycle to be managed effectively to ensure successful delivery of the product.

As of my current understanding, dev cycle management is a driver to product creation and delivery. It allows to structure the work, track progress, and ensure that the final product meets the desired quality standards, and most importantly, aligns with the initial goal.

To effectively manage the software development process simple to use, fairly cheap but yet empowering tool is needed.

Zeitgeist context is important here.
The world is changing rapidly, and AI is becoming increasingly prevalent in software development.
AI allows to automate repetitive tasks, improve code quality, break down development work into detailed steps, and execute those tasks, which emporers small teams and solo-developers to become a team in some sense.
On the other hand, for now getting quality AI assistance is not easy.

## Expectations and vision

1. Even a solo developer should manage dev cycle to be able create and deliver a product especially if use AI. Small teams indeed.
2. The tool should be lightweight, easy to use, easily integratable to a project, allowing developers to focus on their work rather than the tool itself.
3. Tool should be suitable for solo developers and small teams.
4. Tool should drive and automate process of product creation and delivery.
5. Tool should provide at least a way to:
    - Define value -> features -> requirements
    - Track requirements implementation progress
    - Manage delivery iterations
    - Manage development iterations of delivery iteration
    - Manage collaboration.
    - Manage contexts for AI assistance.
6. Tool should allow to use AI to enhance the development process like perform tasks or steps of development iteration, and even to allow AI to automate some management actions in the process itself, like check conditions to proceed, identify bottlenecks, inconsistencies, struggles, missing things, tracking, and suggest improvements.

Such expectations make challenging to manage trade-offs between simplicity, ease of use, and powerful features, balances to be found.

### Solo/Small team and AI leverage crossroad focus

1. Solo and small team development might need self-discipline to manage dev cycle effectively. So driving and automating the process is important.
2. Effective use of AI requires clear, specific, detailed, yet concise context to perform well. So structuring the work in small steps with careful context management is important.
3. Context management and clear definition of work items is hard to do in solo/small team (maybe in bigger teams too), but actually required for achieve quality, even without AI.
4. Heavy use of AI with quality might give solo/small team developers big leverage to achieve more.

### Key expectations

1. Tool drives dev process from idea to delivery and support.
2. Tool allows to define/customize whole process.
3. Tool allows to manage context evolution through whole delivery iterations, its development iterations, and extraction of context for tasks.
4. Tool allows to use AI to perform tasks or steps of development iteration, and even to allow AI to assist management actions in the process itself.
5. Tool requires minimal effort to setup and use, but expendable to cover more advanced use scenarios.
6. Tool provides high observability and control of current state, tasks preparation and results.

## Some expectations clarifications

### fairly simple and cheap

#### cheap

Three options in perspective:

- free and open-source - a basic core, might have tight scope which is to evaluated based on further research and design.
- low-cost subscription - as hosted service with more advanced features which implementation based on basic core
- self-hosted paid version - for enterprises with more advanced features which implementation based on basic core

#### simple

- locally as basic core - simple CLI tools + configuration, ide plugins, file-based data storage
- hosted service - web UI + API + integrations
- ide plugins

### easily integratable to a project

#### as CLI toolset

- installable via package manager
- configurable via simple text files
- in project directory

#### as hosted service

- via API

### drive and automate process

#### drive

- encouraging to use
- guiding through the process steps
- providing clear undestanding of current state and what to do next and results of actions
- product centered

#### automate

- perform repetitive, mechanical tasks, i.e. updating statuses, commits, pushes, running tests, deployments, notifications
- perform well-defined tasks based on clear context using AI assistance
- perform some process management tasks using AI assistance
- provide automation configuration options

### Manage collaboration

- autoassign roles and responsibilities
- provide commenting system
- provide notifications

### Manage contexts for AI assistance

In fact more realistic expectation is that tool helps facilitate context preparation for tasks by providing structured templates and linking mechanisms.

This problem is a subject of separate research and design.

but for now:

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

### Allow AI to automate management actions in the process itself

In two aspects:

#### automate task management actions

- AI can support development process by helping with:
  - checking conditions to proceed (i.e. DOD check),
  - refining requirements,
  - extracting context for tasks,
  - identifying bottlenecks, inconsistencies, struggles,
- This requires a well-defined set of requirements, configurations and task management protocols.

#### management tasks

At the moment it seems AI can work on any task, including requirement breakdown, task creation, prioritization, assignment, progress tracking, etc.
Given careful context management, observability and human participation, AI can be allowed to participate in development management process itself via let say management tasks.
It means tool should allow assigning any single task to AI, yet keeping control of what to be done and results.

---

## Missing details to be worked out in other docs

- **Specific technical implementation details**: (TBD in other docs)
- **Clear differentiation from existing tools beyond "lightweight" and "AI-enhanced"**: (TBD in other docs, )
  Will need to research existing tools more thoroughly to identify uniqueness and worthiness at all.
- **Concrete examples of AI integration**
- **Version control integration details** actually out of scope for tool itself in general, but as common practice maight need to be mentioned in some other docs.
- **Data storage and synchronization approach** technical aspect, TBD in other docs
- **User interface considerations** (TBD in other docs)
- **Business model aspects** (TBD in other docs)

## Grand milestones

- research phase:
  - validate the idea
  - refine the idea
  - high-level design on aspects of intergrandmilestone dependencies
- free and open-source:
  - research
  - design
  - implement
  - deliver
  - get feedback
  - improve and support
- reconnaissance:
  - revalidate the idea
  - refine the idea
  - high-level redesign on aspects of intergrandmilestone dependencies
  - start business model exploration
- low-cost subscription:
  - research
  - design
  - implement
  - deliver
  - get feedback
  - improve and support
- reconnaissance 2:
  - revalidate the idea
  - refine the idea
  - high-level redesign on aspects of intergrandmilestone dependencies
  - finalize business model
- self-hosted paid version:
  - research
  - design
  - implement
  - deliver
  - get feedback
  - improve and support
- .... PROFIT!
