<!--
[AI Agent Instructions for TODO]
⚠️ WARNING: This file is the place to manage "coarse-grained milestones (Epics)" for the entire project.
Do NOT write detailed functional specifications (like AC) here in the early stages. Detailed specifications and designs MUST be created as separate files in `docs/features/` and `docs/design-docs/` when starting each milestone.
Agents MUST always return here to fill in the checkmark `[x]` and update the progress summary whenever a milestone or task is completed.
-->

# TODO.md

## Operational Rules

- This document MUST ONLY contain the macro implementation order (milestones) for the entire project.
- All tasks MUST proceed in the cycle of **"Requirements/Design ➔ Plan ➔ Harness ➔ Act ➔ Audit ➔ Verify ➔ Sync"**.
- When starting a new task (milestone), first define and agree upon the specifications and design in `docs/features/` and `docs/design-docs/`, and then create a plan using `docs/PLAN_TEMPLATE.md`.
- Progress MUST be reflected in `docs/AGENT_CONTEXT.md` per session, and the completion marks in this file MUST be updated.

## Milestones

### MS1: Foundation Setup and Requirements Definition (Bootstrapping)
**Completion Criteria**
- The project's language and environment are determined, and `check_harness.sh` is functioning.
- The initial versions of `PRD.md` and `ARCHITECTURE.md` are created and agreed upon.

**TODO (Instructions for AI Agents)**
- [ ] **Step 1**: Interview the user (or guess) the development language and main frameworks, and reach an agreement.
- [ ] **Step 2**: Implement `scripts/check_harness.sh` according to the decided language, and set up your verification rails (Harness).
- [ ] **Step 3**: Interview the user about the project's purpose, and create `PRD.md` (Product Requirements Document and feature list) and `ARCHITECTURE.md` (Tech Stack).

### MS2: Initial Feature Implementation (Phase 1)
**Completion Criteria**
- The first feature defined in the PRD is implemented and has passed all quality gates.

**TODO**
- [ ] **Spec & Design**: Create or update the specifications and design for the feature to be implemented in this milestone in `docs/features/` and `docs/design-docs/`, and get agreement.
- [ ] **Plan**: Copy `docs/PLAN_TEMPLATE.md` to create a plan, and get agreement on the testing perspectives (matrix).
- [ ] **Act**: Follow the plan and proceed with implementation using TDD (Red ➔ Green ➔ Refactor).
- [ ] **Audit/Verify**: Perform a self-audit based on `docs/AUDIT.md` and pass `check_harness.sh`.
- [ ] **Sync**: Update documents such as `docs/QUALITY_SCORE.md` and move the plan to completed.

## 📊 Progress Summary

- **Total Tasks**: 7
- **Completed Tasks**: 0
- **Current Milestone**: MS1