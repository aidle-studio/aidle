# Copilot Instructions

In this repository, `AGENTS.md` is treated as the Single Source of Truth.
Decisions regarding implementation, specifications, and testing should always refer to the following in order:

1. `AGENTS.md`
2. `docs/RULES.md`
3. `PRD.md`
4. `docs/TEST_PLAN.md`
5. `docs/TODO.md`
6. `docs/KNOWLEDGE.md`

## Execution Rules (Summary)
- Specification-Driven Development (SDD) and Test-Driven Development (TDD) are mandatory.
- Add corresponding `TC-*` before implementation, and follow the `Red -> Green -> Refactor` cycle.
- Maintain traceability (correspondence between `AC-*` and `TC-*`) for all changes.

## Prohibitions (Summary)
- Do not proceed with implementation based on unconfirmed specifications.
- Do not make destructive changes to existing rules or files without permission.
- Do not treat individual rules that contradict `AGENTS.md` as the source of truth.
