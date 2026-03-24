# RULES.md

This is the "Constitution" that defines the development process and Definition of Done (DoD) of the project.
This file serves as a hub (portal) to the "Laws to be Obeyed" in each phase.

## 1. Development Principles (Harness-First)

- **Verification-Driven**: Prioritize and trust mechanical verification such as `scripts/check_harness.sh` over natural language instructions.
- **Specification-Driven (SDD)**: Treat the requirements in `PRD.md` and the acceptance criteria (`AC-*`) in each functional specification (`docs/features/*.md`) as the only Source of Truth.
- **Test-Driven (TDD)**: Repeat the `Red -> Green -> Refactor` cycle as the smallest unit.

## 2. Standard Development Process and Navigation

Development proceeds through the following steps. For details on each step, comply with the specialized documents (laws) linked.

1. **Spec & Design**:
   - When adding new tasks or features, do not write code or plans immediately. First, create or update functional specifications (What) using `docs/features/FEATURE_TEMPLATE.md`.
   - Next, create or update detailed design documents (How) using `docs/design-docs/DESIGN_TEMPLATE.md`, and link both.
   - At this stage, obtain human agreement (Agreement Gate).
2. **Plan**:
   - Based on the specifications and design defined and agreed upon above, copy `docs/PLAN_TEMPLATE.md` to create a new plan file in `docs/exec-plans/active/`.
   - In the plan, define specific "test perspectives (FMEA, etc.)" and "task division" based on the design, and obtain agreement again.
3. **Harness**:
   - Confirm and set up the verification rails necessary to prove the defined quality.
   - ➡️ **Setup Method**: Follow `docs/HARNESS.md`.
4. **Act**:
   - Implement the minimum code based on specifications and tests.
   - ➡️ **Specifications**: Refer to `PRD.md` or functional specifications (`docs/features/*.md`).
   - ➡️ **Test Density and Perspectives**: Strictly follow `docs/TEST_PLAN.md` (density goals per Tier, mandatory anomaly cases, etc.).
   - ➡️ **Reliability and Error Handling**: Follow `docs/RELIABILITY.md`.
   - ➡️ **Specific Technical Rules**: If there are AI cheatsheets for libraries, etc., in `docs/references/`, be sure to read them thoroughly.
5. **Audit**:
   - Before verification, self-audit whether the implementation meets the quality goals.
   - ➡️ **Audit Criteria**: Confirm whether the indicators in `docs/AUDIT.md` are met.
   - ➡️ **Quality Score**: Update `docs/QUALITY_SCORE.md` upon completion of a milestone.
   - 💡 **Recommendation**: For important changes such as Tier 1, an independent peer review using another session (e.g., `code-reviewer`) is strongly recommended.
6. **Verify**:
   - Run `scripts/check_harness.sh` and confirm mechanical passing.
7. **Sync and Commit**:
   - Update specifications (`PRD.md`, `docs/features/*.md`), detailed designs (`ARCHITECTURE.md`, `docs/design-docs/*.md`), `TODO.md`, `AGENT_CONTEXT.md`, etc., to the latest versions, and commit after preparing the completion state.
   - ➡️ **Recording Lessons**: If there were any pitfalls, add them to `docs/KNOWLEDGE.md`.

## 3. Quality Gates (Definition of Done: DoD)

To declare a task "Done," all the following conditions must be met.

- **Achievement of Quality Goals**: Quality goals defined in the plan are proven by tests.
- **Sufficiency of Test Density**: Density goals per Tier defined in `docs/TEST_PLAN.md` are cleared.
- **Audit Passed**: Confident that audit criteria are cleared and quality is valid.
- **Harness Success**: `scripts/check_harness.sh` is 100% successful.
- **Sync Completed**: Contexts such as `TODO.md` and `AGENT_CONTEXT.md` are updated to the latest versions.
