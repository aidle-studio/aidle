<!--
[AI Agent Instructions for Planning]
Before creating this plan, conduct research and Chain of Thought using the following steps:
1. **Confirmation of Requirements and Design**: Check the current task in `docs/TODO.md`, and confirm that the target functional specification (`docs/features/*.md`) and detailed design document (`docs/design-docs/*.md`) have already been created and agreed upon. *Note: If they do not exist, stop creating the plan and create them first.*
2. **Definition of Quality**: Determine the target Tier based on `docs/TEST_PLAN.md`, and fill in the test matrix from the perspective of "how it breaks (anomaly/boundary)" before normal cases.
After completing these, fill in the following items, present them to a human, and obtain agreement (Agreement Gate).
-->

# Plan: [Milestone/Task Name]

## Plan Meta
- Milestone ID:
- Date:
- Status: Draft / Agreed / In Progress / Done

## Context & Goal
- **Why (Purpose)**: [TODO: Why is this milestone necessary, and what value or problem does it solve?]
- **What (Goal)**: [TODO: Describe the specific state to be achieved in this milestone]
- 💡 Refer to **`PRD.md`** or each functional specification for details on specifications.

## Scope
- **In Scope (To Do)**:
  - [TODO]
- **Out of Scope (Not To Do)**:
  - [TODO: Explicitly state what is intentionally omitted from this scope to prevent bloat]

## Quality Goals
- [ ] **Target Tier**: [Tier 1 / Tier 2 / Tier 3] (Refer to `docs/TEST_PLAN.md`)
- [ ] **Quality Characteristics**: [TODO: Characteristics of ISO/IEC 25010 to prioritize, such as Functional Suitability, Reliability, Maintainability, etc.]
- [ ] **Specific State**: [TODO: Define a "good state" objectively, not just "it works"]

## Test Perspectives
💡 Comply with the density goals and mandatory anomaly rules of **`docs/TEST_PLAN.md`**.

### UT: Boundary/Anomaly Matrix
| Target (Function/State) | Valid (Normal) | Boundary (Extreme) | Empty/Zero | Invalid (Incorrect Type) |
| :--- | :--- | :--- | :--- | :--- |
| [TODO] | [TODO] | [TODO] | [TODO] | [TODO] |

### IT: FMEA (Failure Mode) Matrix
| Dependent Component | Timeout (Delay) | Disconnect | 5xx Error (Abnormal Response) | Malformed (Invalid Data) |
| :--- | :--- | :--- | :--- | :--- |
| [TODO] | [TODO] | [TODO] | [TODO] | [TODO] |

## Tasks & Reliability (Implementation Tasks and Reliability Design)
💡 Following **`docs/RELIABILITY.md`**, define the error handling and protection strategies for this task first.

- **Fail-fast**: [TODO: What invalid inputs or abnormal states will be rejected by validation?]
- **Timeout & Retry**: [TODO: For external communication or heavy processing, what will be the timeout and retry strategy?]
- **Fallback**: [TODO: How will the system behave if dependencies are completely down (how to minimize the impact on UX)?]

💡 Based on the above strategy, divide tasks into the smallest units (set of test creation and implementation) that allow for a "TDD cycle (Red -> Green -> Refactor)".
💡 Adhere to **`ARCHITECTURE.md`** (structure) during implementation.
- [ ] T1: [TODO: Smallest unit of work including test and implementation]
- [ ] T2: [TODO: ...]

## Definition of Done (DoD)
💡 All the following completion criteria must be met, and checked `[x]`.
- [ ] **(Tasks)**: All the above Tasks are completed.
- [ ] **(Audit)**: Confident that the audit criteria (quality goals, test density) of `docs/RULES.md` have been met.
- [ ] **(Harness)**: `scripts/check_harness.sh` is 100% Green.
- [ ] **(Sync)**: Requirements/design documents (PRD, features, ARCHITECTURE, design-docs) and state management documents (TODO, AGENT_CONTEXT) have been updated to the latest versions.
- [ ] **(GC)**: Moved this plan to `docs/exec-plans/completed/` and added lessons learned to `docs/KNOWLEDGE.md`. (Check this just before moving the plan)
- [ ] **(Custom)**: [TODO: Describe if there are any completion criteria specific to this plan. Delete if none]

## Execution Log
- [TODO: YYYY-MM-DD HH:MM: Summary of executed content, occurred errors, and their solutions]
