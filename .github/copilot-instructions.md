# Project

This project is a tool to manage product oriented process of software development.

## Structure

- `rnd/` - Project level Research and Design files.
- `product/` - Project's software development files.

## Idea

Refer to `rnd/IDEA.md` for the high level idea description.

Idea summary:

Toolset to drive and automate product oriented software development process, easily integratable to a project, allowing to manage collaboration and contexts for AI assistance.

## Code Quality Policy

Code can be stratified into following testability related categories:

- `utility`: helper, abstract functions, converters, validators - easy to test with unit tests.
   - Should be covered with unit tests only.
   - Code Coverage: high.
   - Functional Coverage: full. Positive, negative, edge cases.
- `compositions`: entry points, orchestrating functions - code close to user actions,  heavily depend on other components - hard or impossible to test with unit tests.
    - Should be covered with API test scenarios.
    - Interacted or Mocked - depending on complexity of external dependencies to up and prepare.
    - Code Coverage: optional.
    - Functional Coverage: high level scenarios only.
- `integrations`: code providing internal API to external tools, services, systems - affordable to test with isolated integration tests.
    - Should be covered with integration tests.
    - Internal API coverage: high.
    - Code Coverage: high.
    - Functional Coverage: full. Positive, negative, edge cases.
- `bl`: business logic or domain specific code:
    - Should be separated (as much as possible) into:
        - abstracts: bussines abstracts, utilities, closures with api defined in domain terms.
        - compositions: orchestrating functions, actual bussines process implementations.
    - Abstracts should be covered with unit tests only.
        - Code Coverage: high.
        - Functional Coverage: full. Positive, negative, edge cases.
    - Compositions should be covered with API test scenarios or integration tests.
        - Code Coverage: optional.
        - Functional Coverage: high level scenarios only.

## Code Clarity Policy

- Follow Language specific coding standards and best practices.
- Ensure modularity, reusability, extendability of components.
- Avoid dependency cycles even if the language or build system allows them.
- Avoid code and functional duplication.
- Write clear and concise documentation for APIs.
- Implement comprehensive error handling and logging.
