# POC_1 requirements

## Expectations

### Functional expectations

1. PoC version of tooling for development process conduction is ready to try out in small team and solo developer scenarios.
2. Process which tooling can conduct is configurable to fit different small team and solo developer process needs.
3. Tooling allows small teams and solo developers to ensure progress and quality.
4. Tooling provides context maintenance and quality assurance capabilities.

### Non-functional expectations

1. Tooling is easy to use and does not add significant overhead to the development process.
2. Tooling stores data in files within the project's repository to ensure portability and ease of access.
3. Tooling files are human-readable and editable to allow for manual adjustments if necessary.
4. Tooling requires no or minimal setup to get started.

## Hi level Design

1. Tooling is designed as a command-line interface (CLI) application to facilitate easy integration into existing workflows.
2. Tooling allows to configure the development process via configuration files for different small team and solo developer scenarios.
3. Tooling allows for AI-driven automation and augmentation of development tasks via Model Context Protocol server.
4. Toolchain: rust programming language for CLI application and MCP server, YAML for configuration files, Markdown for step data-files.

## Test template

1. Create a sample project repository to test the tooling.
2. Define a simple development process configuration suitable for small teams or solo developers.
3. Use the tooling to conduct the development process on the sample project.
4. Verify that the tooling correctly follows the defined process and maintains context quality.
5. Evaluate the usability and performance of the tooling during the test.

### conduct the development process

1. use CLI commands to initialize the process in the sample project repository.
2. use CLI to familiarize with current project context and status.
3. use CLI to see what to do next in the process.
4. use CLI to start and complete development tasks according to the defined process.
    - familiarize with task requirements and context.
    - implement the task.
    - close the task and update context.
5. use CLI to monitor progress and context quality throughout the process.

## Scope

focus on:

1. [parocess model](../../research/2-feasibility/POC_1/process_model.md#expected-scenarios-inference)
2. [parocess model](../../research/2-feasibility/POC_1/process_model.md#process-configurability-and-adaptability)
3. [parocess model](../../research/2-feasibility/POC_1/process_model.md#context-maintenance-and-semantic-connection)
4. [process model](../../research/2-feasibility/POC_1/process_model.md#context-quality)
5. providing CLS and MCP server implementations to support the defined process model and scenarios.

## POC_1 Requirements

| RID        | Requirement                                                  |
|------------|--------------------------------------------------------------|
|POC_1-1     | CLI uses configuration files stored in the project repository|
|            | to define the development process.                           |
|------------|--------------------------------------------------------------|
|POC_1-1.1   | Configuration files are in YAML format and are human-readable|
|            | and editable.                                                |
|------------|--------------------------------------------------------------|
|POC_1-1.2   | Defines cyclic processes with iterations and steps.          |
|------------|--------------------------------------------------------------|
|POC_1-1.2.1 | Steps can be defined as tasks or sub-processes.              |
|------------|--------------------------------------------------------------|
|POC_1-1.3   | Defines step classifications with:                           |
|------------|--------------------------------------------------------------|
|POC_1-1.3.1 | default values for attributes.                               |
|------------|--------------------------------------------------------------|
|POC_1-1.3.2 | sets of parameters for inputs, outputs and scope.            |
|------------|--------------------------------------------------------------|
|POC_1-1.4   | Defines step templates for rendering step content based on   |
|            | classification.                                              |
|------------|--------------------------------------------------------------|
|POC_1-1.5   | Defines step links to represent dependencies or relationships|
|            | between steps.                                               |
|------------|--------------------------------------------------------------|
|POC_1-1.6   | Defines parameter types (id, purpose, data type).            |
|------------|--------------------------------------------------------------|
|POC_1-1.6.1 | Parameter data types are:                                    |
|            | STR, INT, DEC, BOOL, DATE, RANGE, SET, SUBSET                |
|            | CONTENT - representing artefact file by link, mime-type      |
|------------|--------------------------------------------------------------|
|POC_1-2     | CLI provides command to initialize the development process in|
|            | the project repository.                                      |
|------------|--------------------------------------------------------------|
|POC_1-2.1   | creates necessary directories and files.                     |
|------------|--------------------------------------------------------------|
|POC_1-2.2   | prefill configuration file                                   |
|------------|--------------------------------------------------------------|
|POC_1-2.2.1 | empty process root step.                                     |
|------------|--------------------------------------------------------------|
|POC_1-2.2.2 | default template                                             |
|------------|--------------------------------------------------------------|
|POC_1-3     | CLI provides command to view current project status:         |
|------------|--------------------------------------------------------------|
|POC_1-3.1   | hierarchy of process steps with statuses.                    |
|------------|--------------------------------------------------------------|
|POC_1-3.2   | current project context summary.                             |
|------------|--------------------------------------------------------------|
|POC_1-4     | CLI provides command to get next recommended actions in the  |
|            | process.                                                     |
|------------|--------------------------------------------------------------|
|POC_1-5     | CLI provides command to initialize process cycles:           |
|------------|--------------------------------------------------------------|
|POC_1-5.1   | define iteration scope (provide scope params).               |
|------------|--------------------------------------------------------------|
|POC_1-5.2   | initialize iteration steps.                                  |
|------------|--------------------------------------------------------------|
|POC_1-6     | CLI provides command to start a development task:            |
|------------|--------------------------------------------------------------|
|POC_1-6.1   | fetch task requirements and context.                         |
|------------|--------------------------------------------------------------|
|POC_1-6.2   | mark task as in-progress.                                    |
|------------|--------------------------------------------------------------|
|POC_1-7     | CLI provides command to complete a development task:         |
|------------|--------------------------------------------------------------|
|POC_1-7.1   | submit task implementation.                                  |
|------------|--------------------------------------------------------------|
|POC_1-7.2   | update project context.                                      |
|------------|--------------------------------------------------------------|
|POC_1-7.3   | mark task as completed.                                      |
|------------|--------------------------------------------------------------|
|POC_1-8     | CLI provides command to monitor project progress:            |
|------------|--------------------------------------------------------------|
|POC_1-8.1   | overall progress metrics.                                    |
|------------|--------------------------------------------------------------|
|POC_1-9     | Process data is stored in dedicated folder (`glow/`) within  |
|            | the project repository (root).                               |
|------------|--------------------------------------------------------------|
|POC_1-9.1   | each step presented by separate markdown file.               |
|------------|--------------------------------------------------------------|
|POC_1-9.1.1 | file might be accompanied by same-named folder for additional|
|            | artifacts (CONTENT) and sub-step files.                      |
|------------|--------------------------------------------------------------|
|POC_1-9.1.2 | File contents are human-readable and editable: YAML.         |
|------------|--------------------------------------------------------------|
|POC_1-9.1.3 | file can represent step's attributes.                        |
|------------|--------------------------------------------------------------|
|POC_1-9.1.4 | file contents rendered by step template if defined.          |
|------------|--------------------------------------------------------------|
|POC_1-9.1.5 | file contents contain front-matter section for metadata.     |
|------------|--------------------------------------------------------------|
|POC_1-9.1.6 | file can hold values of step parameters (in, scope)          |
|------------|--------------------------------------------------------------|
|POC_1-9.1.7 | file can hold values of parent step parameters (in, scope)   |
|------------|--------------------------------------------------------------|
|POC_1-9.1.8 | file can hold values of sibling step parameters via parent   |
|------------|--------------------------------------------------------------|
|POC_1-9.1.9 | file can hold values of own step parameters (step-as-process)|
|------------|--------------------------------------------------------------|
|POC_1-9.2   | folder structure depends on the process steps configuration. |
|------------|--------------------------------------------------------------|
|POC_1-10    | Context quality assessment is provided based on:             |
|------------|--------------------------------------------------------------|
|POC_1-10.1  | completeness of step parameters (inputs, scope).             |
|------------|--------------------------------------------------------------|
|POC_1-10.2  | semantic connection between steps contexts.                  |
|------------|--------------------------------------------------------------|
|POC_1-10.3  | consistency of context across process.            |
|------------|--------------------------------------------------------------|
|POC_1-11    | MCP server implementation to support the defined process     |
|            | model and scenarios.                                         |
|------------|--------------------------------------------------------------|
