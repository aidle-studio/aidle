# TODO.md

## Operational Rules

- All tasks follow the **"Plan ➔ Harness ➔ Act ➔ Review ➔ Verify"** cycle.
- Items without an Agreement Gate are not considered complete.
- Reflect progress in `docs/AGENT_CONTEXT.md` each session and update the checkboxes `[x]` in this file.
- **Garbage Collection (GC)**: Move completed plans to `completed/` and extract lessons to `KNOWLEDGE.md`.

## Milestones

### MS1: Foundation & Bootstrapping
**Success Criteria**
- Project language/environment decided and `check_harness.sh` is operational.
- Initial versions of `ARCHITECTURE.md` and `SPEC.md` are created and agreed upon.

**TODO (AI Agent Instructions)**
- [ ] [ ] **Step 1**: Consult with the user to agree on the development language and main frameworks.
- [ ] [ ] **Step 2**: Implement `scripts/check_harness.sh` for the chosen language and set up verification rails.
- [ ] [ ] **Step 3**: Identify project goals and create `ARCHITECTURE.md` (Tech Stack) and `SPEC.md` (Initial AC-001).

### MS2: Core Feature Implementation (Phase 1)
**Success Criteria**
- `TC-001` is Green and `scripts/check_harness.sh` passes.

**TODO**
- [ ] [ ] Implement TC-001 as a Red test.
- [ ] [ ] Make TC-001 Green.
- [ ] [ ] Refactor and update `QUALITY_SCORE.md`.

## 📊 Progress Summary

- **Total Tasks**: 5
- **Completed Tasks**: 0
- **Current Milestone**: MS1
