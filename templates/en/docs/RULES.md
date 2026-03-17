# RULES.md

This document defines the development process and Definition of Done (DoD) for this project.
AI agents and developers must strictly adhere to these rules.

## 1. Development Principles (Harness-First)

- **Verification-Driven**: Prioritize and trust mechanical verification (e.g., `scripts/check_harness.sh`) over natural language instructions.
- **Specification-Driven (SDD)**: All implementations must be based on requirements and Acceptance Criteria (AC) defined in `docs/product-specs/` or `SPEC.md`.
- **Test-Driven (TDD)**: Follow the Red/Green/Refactor cycle. Never implement production code without a failing test case.
- **Atomic Commits**: Keep commits small and focused. Each commit should represent a single logical change.

## 2. Standard Development Process

1. **Plan**: Create a plan in `docs/exec-plans/active/` and reach an Agreement Gate with a human.
2. **Harness**: Verify that the necessary verification rails are set up according to `docs/HARNESS.md`.
3. **Act**: Implement the minimum code required based on specifications and tests.
4. **Review**: Perform a self-assessment based on the "Self-Review Checklist" below.
5. **Verify**: Run `scripts/check_harness.sh` to ensure mechanical compliance.

## 3. Self-Review Checklist

Agents must conduct a self-review before the Verify phase, focusing on these four perspectives:

### 📦 Source Code
- [ ] **Naming**: Do variable and function names clearly reflect their intent?
- [ ] **Safety**: Are edge cases, null pointers, and exceptions properly handled?
- [ ] **Cohesion**: Is the responsibility of each function or module appropriate?

### 🧪 Test Code
- [ ] **Density**: Do tests meet the Tier-specific goals defined in `TEST_PLAN.md` (e.g., branch coverage)?
- [ ] **Multifaceted**: Are edge cases and error paths covered based on FMEA?
- [ ] **Independence**: Can tests run in isolation without sharing state?

### 🤖 Harness
- [ ] **Automation**: Are new changes properly verified by `check_harness.sh`?
- [ ] **Feedback**: Does the harness provide clear hints for fixing failures?

### 📝 Docs
- [ ] **Consistency**: Is there any discrepancy between the code and documents like `SPEC.md` or `ARCHITECTURE.md`?
- [ ] **Clarity**: Is the "Why" behind the implementation clear to future readers?

### ✨ Product Sense - Recommended for Tier 2+
- [ ] **User Experience**: Are error messages helpful (providing solutions rather than just "Error")?
- [ ] **Intent Alignment**: Does the behavior match the expectations of the target audience defined in `PRODUCT_SENSE.md`?

### 💎 Architecture & Design - Required for Tier 1/2
- [ ] **Alignment**: Is the design consistent with `ARCHITECTURE.md`?
- [ ] **Coupling**: Is the implementation loosely coupled and reusable?

### ⚡ Performance & Optimization - Required for Tier 1
- [ ] **Efficiency**: Are there obvious bottlenecks (computational complexity, N+1 queries)?
- [ ] **Resources**: Is memory and network resource usage minimized?

## 4. Context Sync & GC

At the start of a session or when task status changes, the following files must be updated **atomically**:

1. **`docs/AGENT_CONTEXT.md`**: Update current phase, goals, and next actions.
2. **`docs/TODO.md`**: Mark tasks as done `[x]` and update the progress summary.
3. **`docs/exec-plans/active/*.md`**: Update plan status and execution logs.
4. **`docs/KNOWLEDGE.md`**: Record bug fixes and critical lessons learned.

**🧹 Context Garbage Collection (GC)**
When a milestone or task is completed:
- Move the execution plan from `docs/exec-plans/active/` to `docs/exec-plans/completed/`.
- Extract key lessons or pitfalls from the plan and record them in `docs/KNOWLEDGE.md`.

## 5. Tier Definitions (Criticality)

Tasks are classified by Tier, which dictates the required test density:

- **Tier 1 (Core)**: Foundation, data integrity, and security-critical features. (High Density)
- **Tier 2 (Feature)**: Main user features and business logic. (Standard Density)
- **Tier 3 (Utility)**: Internal tools, UI tweaks, and minor utilities. (Minimum Density)

## 6. Definition of Done (DoD)

To declare a task "Done," all of the following conditions must be met:

- **Spec Fulfillment**: All tests (`TC-*`) linked to `AC-*` are `Green`.
- **Self-Review**: All items in the "Self-Review Checklist" are verified.
- **Density Goals**: Tier-specific density targets (from `TEST_PLAN.md`) are met.
- **Harness Success**: `scripts/check_harness.sh` passes successfully.
- **Agreement**: For breaking changes, an Agreement Gate has been reached.
- **Up-to-Date**: `docs/TODO.md` and `docs/AGENT_CONTEXT.md` are synchronized.

## 7. Process Evolution

Project rules are not static. Follow these steps to improve the process:

1. **Detection**: Identify redundant steps, contradictions, or new quality improvement ideas.
2. **Proposal**: Propose changes in `docs/adr/` or within an execution plan, explaining the benefits.
3. **Application**: After agreement, update the documents and `templates/`.

## 8. Commit Conventions

- **Title**: One line, concise summary of the change.
- **Body**: Describe "Why" and any implementation notes if necessary.
- **Traceability**: Include `AC-*` or milestone IDs in the message.
