# Enterprise Process Scenario as Reference

This document outlines the ideal process scenario of product development. It defines the expected outcomes, results, and activities for each phase of the development process, from initial idea to delivery with quality control.

It's aim is to describe a comprehensive and structured approach to software development that ensures alignment with product vision, goals, and requirements while maintaining high standards of quality

This is To be used as example of heavyweight model example contrasting current "as is" processes described [here](./solo_dev_scenarios_as_is_analysis.md) and [here](./small_team_scenarios_as_is_analysis.md)   for evaluating approaches and tools that can help achieve results of this ideal process results but with lightweight and resouce efficient means by solo developers and small teams.

> In the scope of this project, we suppose quality includes all aspects of the product, including but not limited to functionality, performance, security, usability, maintainability, traceability, scalability, and compatibility and user experience.

> In the scope of this project, we suppose costs include all aspects of the development process, including but not limited to time, effort, resources, tools, infrastructure, and opportunity costs.

## Universal Development Scenario

### Initial Idea/Concept/Vision bake

Expected Outcomes:

1. Clear and well-defined product vision and goals.
2. Comprehensive understanding of target audience and needs.
3. Initial roadmap and feature set outlined.

Expected Results:

1. Documented product vision and goals.
2. User personas and market research findings.
3. Preliminary product roadmap and reasons.

Expected Activities:

1. Conduct brainstorming sessions to refine the product idea.
2. Perform market research to identify target users and competitors.
3. Outline initial features and create a product roadmap.
4. Validate the concept with potential users or stakeholders.
5. Document findings and decisions for future reference.
6. Review and iterate on the vision and roadmap as needed.

### Processes Setup

Expected Outcomes:

1. Established development environment and tools covering:
   - Documentation
   - Analysis
   - Design
   - Communication
   - Planning
   - Issue Tracking
   - Knowledge Management
   - Project Management
   - Change Management
   - Quality Assurance
   - Collaboration
   - Review and Validation
2. Defined workflows and processes for efficient development.

Expected Results:

1. Documented and established workflows and processes for development activities.
2. Vision and Process setup documentation is hosted, processed, reviewed and validated by tools in environment established.

Expected Activities:

1. Research and select appropriate tools for each  activity.
2. Configure and set up process environment.
3. Define workflows and processes for development activities.
4. Document the setup and processes for future reference.
5. Review and iterate on the setup and processes as needed.
6. Establish integration between tools to streamline workflows.
7. Train oneself on using the selected tools and following the defined processes.
8. Continuously evaluate and improve the development setup as needed.

### High Level Analysis

Inputs:

1. Initial Product Vision and Goals.
2. Preliminary Product Roadmap and reasons.

Expected Outcomes:

1. Identified key usage-scenarios/bussiness-processes, user-stories/use-cases, and acceptance criteria.
2. Clear understanding of key product requirements and specifications.
3. Identified resource and skill gaps and constraints.
4. Identified potential risks and mitigation strategies.
5. Established tracewiring between requirements, initial vision, decisions.
6. Correlated Goals, Vision, Roadmap, Requirements.

Expected Results:

1. Documented product requirements and specifications. including:
   - Key usage-scenarios/bussiness-processes.
   - User-stories/use-cases.
   - Acceptance criteria.
2. Documented resource and skill gaps fulfillment plan.
3. Documented risk assessment and mitigation strategies.
4. Docemented changes to initial vision based on analysis findings.

Expected Activities:

1. Decompose high-level roadmap items into specific user stories and use cases.
2. Define product (or Bussiness) functional and non-functional requirements, including acceptance criteria.
3. Document and review requirements in the established environment.
4. Analyze requirements against current skills and resources to identify gaps.
5. Develop a plan to address identified resource and skill gaps, document and review it.
6. Identify potential risks (technical, schedule, motivation) and define mitigation strategies. Document and review them.
7. Map requirements back to the Initial Vision and Goals to ensure alignment and traceability.
8. Review and update the Product Vision and Roadmap and analysis documentations to ensure consistency.

### High Level Design

Inputs:

1. Documented Product Vision and Goals.
2. Documented Product Requirements and Specifications.
3. Documented Risk Assessment and Mitigation Strategies.
4. Documented Resource and Skill Gaps Fulfillment Plan.

Expected Outcomes:

1. Identified crititical functions and possible incapabilities in context of product/bussiness FRs and NFRs.
2. Reached compromise on design decisions vs business/product requirements balancing out costs, quality, time, and risks.
3. Correlated Goals, Vision, Roadmap, Requirements, Design Decisions with traceability.
4. Identified Technology Stack and Runtime Infrastructure.
5. Identified limitations and constraints against FRs and NFRs.
6. Established system architecture and design patterns:
   - Defined product modularization and component interactions.
   - Defined scalability, performance, security, and maintainability considerations.
   - Defined Observability and Quality Assurance strategies.
7. Defined Development tools and Environment.
   - Version Control
   - Coding
   - CI/CD
   - Testing
   - Deployment
   - Maintenance
8. Smoke-tested critical design decisions through research, prototyping, and POC if necessary.
product  
9. Defined technology Evolution and Change Management strategies.

Expected Results.

1. Documented design decisions and rationale.
   - Technology Stack and Runtime Infrastructure.
   - Critical core functions implementation concept.
   - Established limitations and constraints.
2. Documented system architecture and design patterns.
3. Documented development tools and environment setup.
4. Documented technology evolution and change management strategies.
5. Documented changes bussines / product requirements and initial vision based on design findings.

Expected Activities:

1. Identify critical functions and potential incapabilities in the context of product/business functional and non-functional requirements.
2. Research and Test possible design decisions and their implications, including prototyping and POC if necessary.
3. Revaluate business/product requirements to reach a compromise on design decisions balancing costs, quality, time, and risks. Correct initial goals and vision if necessary.
4. Document changes to business/product requirements and initial vision based on design findings.
5. Document design decisions and rationale, including technology stack and runtime infrastructure choices.
6. Define system architecture and design patterns, including modularization, component interactions, scalability, performance, security, and maintainability considerations.
7. Define development tools and environment for version control, coding, CI/CD, testing, deployment, and maintenance.
8. Define technology evolution and change management strategies.
9. Map design decisions back to the Goals, Vision, Roadmap, and Requirements to ensure alignment and traceability.
10. Review and update the Product Vision, Roadmap, Requirements, and Design documentation to ensure consistency.

### High Level Quality Assurance Standards & Processes Setup

Inputs:

1. Documented Product Vision and Goals.
2. Documented Product Requirements and Specifications.
3. Documented Design Decisions and Rationale.
4. Documented System Architecture and Design Patterns.
5. Documented Development Tools and Environment Setup.

Expected Outcomes:

1. Defined quality standards and metrics for product covering:
   - functionality and usability, compatibility and user experience.
   - performance, stability, maintainability and scalability.
   - security.
2. Defined quality standard and metrics for development process covering:
   - code readability, maintainability, structure and documentation,
   - design quality,
   - testing coverage and effectiveness,
   - deployment and release quality, (FIXME what is this about?)
   - traceability and compliance (FIXME what is this about).
3. Defined quality assurance processes and practices to ensure quality covering:
   - code review practices.
   - testing strategies (unit, integration, system, acceptance).
   - continuous integration and continuous deployment (CI/CD) requirements.
   - defect tracking and resolution processes.
   - documentation standards (why? what documentation? quaity documentation standards or documentation quality standards?).
4. Defined quality assurance roles and responsibilities.
5. Defined code standards and guidelines.
6. Correlated Goals, Vision, Roadmap, Requirements, Design Decisions, Quality Assurance with traceability.

Expected Results:

1. Documented quality standards, metrict and practices including:
   - for product.
   - for development process.
   - for quality assurance processes.
2. Documented quality assurance roles and responsibilities.
3. Documented code standards and guidelines.
4. Documented changes to Product Vision, Roadmap, Requirements, and Design based on quality assurance findings.

Expected Activities:

1. Define quality standards and metrics for the product, covering functionality, usability, performance, stability, maintainability, scalability, and security.
2. Define quality standards and metrics for the development process, covering code readability, maintainability, structure, documentation, design quality, testing coverage, effectiveness, deployment quality, traceability, and compliance.
3. Define quality assurance processes and practices, including code review practices, testing strategies (unit, integration, system, acceptance), continuous integration and deployment (CI/CD) requirements, defect tracking and resolution processes, and documentation standards.
4. Define quality assurance roles and responsibilities within the development team.
5. Define code standards and guidelines to ensure consistency and maintainability.
6. Map quality assurance elements back to the Goals, Vision, Roadmap, Requirements, and Design Decisions to ensure alignment and traceability.
7. Review and update the Product Vision, Roadmap, Requirements, and Design documentation based on quality assurance findings to ensure consistency.

### Decomposition & Integration Planning

Code, Infrastructure, Environment, Delivery Decomposition & Integration Planning includes breaking down the high-level design into manageable components, planning their integration, and defining delivery strategies.

To run Delivery Loop it is necessary to have strategy or plan for iterative development, release, and delivery of the product. Each iteration should produce a potentially shippable product increment keepning it consistent and aligned with the overall product vision, goals, and requirements.

Inputs:

1. Documented Product Vision and Goals.
2. Preliminary Product Roadmap and reasons.
3. Documented Product Requirements and Specifications.
4. Documented Design Decisions and Rationale.
5. Documented System Architecture and Design Patterns.
6. Documented Development Tools and Environment Setup.
7. Documented Quality Standards, Metrics, and Practices.

Expected Outcomes:

1. Defined requirement sets (as features? traits? capabilities?) to be delivered in first iteraition and subsequent planned iterations (MVP, MMP, ...?).
2. Defined strategy for preparing product increment requirement sets for regular iterations, and requirement dependency and integrity maintenance.
3. Defined Strategy of preparing Test-runs for further increments.
4. Defined strategy for integrating delivered increments into the overall product.

Expected Results:

1. Documented iteration plans and delivery strategies.
2. Documented integration strategies and plans.
3. Documented Test-Run preparation strategy for increments.

Expected Activities:

1. Break down High Level design to requirement mappings to define planned iterations.
2. Define strategy for preparing product increment requirement sets for regular iterations, and requirement dependency and integrity maintenance.
3. Define strategy for integrating delivered increments into the overall product.

### Delivery Loop

The Delivery Loop is the iterative process of converting planned requirements into a delivered product increment. It ensures that every unit of work is context-aware, quality-assured, and traceable back to the initial value proposition.

It consists of cycles of delivering increments of the product to users or customers, gathering feedback, and making necessary adjustments.

Each Cycle may include Development Loop and Release Loop resulting in product increment, Runtime Environment increment, Infrastructure increment and Development Environment increment.

Product increment of Delivery Cycle might be separated into epics for parallel development of independent features, to keep related tasks grouped and connected through development loop.

#### Development Loop

Development Loop is the iterative process of converting planned requirements into potentially shippable product increment.

It consists of cycles containing low level analysis and design, planning, implementing, testing.

Each Cycle may result in changes to the product, runtime environment, infrastructure, development environment.

##### Low Level Analysis & Design

Low Level Analysis & Design supposed to provide tasks for further development of delivery increment.

Low Level Analysis & Design bridges the gap between high-level requirements and code. It translates "what to do" into "how to do it" within the specific context of the existing codebase.

Inputs:

1. Delivery Cycle planned product requirements.
2. High Level Design & System Architecture.
3. Current Codebase State - development documentation.
4. Coding Standards and Guidelines.
5. Quality Standards, Metrics, and Practices.

Expected Outcomes:

1. Precise definitions of tasks. Includng:
   - Detailed requirements.
   - Technical constraints and specifications.
   - Acceptance criteria.
2. Validated feasibility of the implementation plan (what is this about?).
3. Contextual understanding of how new tasks fit into the existing codebase, architecture, and design patterns, product functionality, quality and vision.
4. Tasks Relevant test cases.
5. Traceability between tasks and:
   - product requirements
   - acceptance criteria,
   - design decisions.
   - system architecture.
   - quality assurance standards and processes.
   - coding standards and guidelines.
6. Low-Level Design decisions.
7. Correlated Goals, Vision, Roadmap, Requirements, Design Decisions, Quality Assurance, Tasks with traceability.

Expected Results:

Documented Low-Level Design decisions.

Created Tasks to do in the established environment with:

1. enriched requirement details
2. technical constraints and specifications
3. scoped acceptance criteria and test-cases
4. Context including task-relevant extract summaries (with source links for traceability) of:
   - codebase parts to be affected, used or interfered with,
   - Low-Level Design decisions,
   - Design Patterns to be followed,
   - Architecture components to be integrated with,
   - Quality Assurance standards and processes to be followed,
   - Coding Standards and Guidelines to be followed,
   - Product increment delivery cycle requirements,
   - Product Vision and Goals.

Activities:

1. Translate product requirements, High Level Design:
   - detailed requirements
   - technical constraints and specifications
   - acceptance criteria
2. Define Low-Level Design decisions.
3. Define test-cases.
4. Identify codebase parts to be affected, used or interfered with.
5. Extract relevant context from codebase, design decisions, architecture, quality assurance standards and coding guidelines.
6. Create tasks in the established environment with enriched requirement details, technical constraints and specifications, scoped acceptance criteria and test-cases, and relevant context.
7. Map tasks back to Goals, Vision, Roadmap, Requirements, Design Decisions, Quality Assurance to ensure alignment and traceability.
8. Review and update the Product Vision, Roadmap, Requirements, Design documentation, Quality Assurance, and Coding Standards to ensure consistency.

##### Implementation

Implementation includes coding, testing, code review, integration, and documentation activities to develop the product increment according to the defined tasks.

#### Release Loop

Release Loop includes iterative cycles of deployment, delivery increment testing, regression testing, stabilization, and release.

#### Delivery

Delivery includes delivery and deployment of the product, gathering feedback, support, maintenance, hot-fix delivery.

## Solo-Developer Small-Team Modes Differences

### Solo-Developer Modes

In Self-Employed mode, a solo developer works independently on their own product. They bear all responsibilities, risks, and rewards. The primary challenges revolve around managing all aspects of the project alone, from ideation to deployment.

In Employed mode, a solo developer works within a larger organization or team. They may in some cases have access to employer-provided resources, tools, and support. The primary challenges involve aligning with organizational goals, processes, and collaboration with other team members.

### Small-Team Modes

in Startup mode, a small team (e.g., 2-5 people) works on their own product. They share the vision, risks, and rewards. The primary challenges shift from *doing* the work to *coordinating* the work and maintaining a *shared understanding*.

in Outsourced mode, a small team (e.g., 2-5 people) works on a product for an external client. They must balance the client's requirements, expectations, and communication with their own internal processes and collaboration. The primary challenges involve managing client relationships, scope changes, and delivering value within constraints.
