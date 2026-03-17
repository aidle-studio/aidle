# TEST_PLAN.md

This document defines the test plan to verify the Acceptance Criteria (AC-*) defined in `SPEC.md`.
In AIDD, this file is the **"Traceability Link"** between specifications and implementation.

## 🧪 Test Design Principles

1. **AC-TC Traceability & Density**: For each `AC-*`, ensure **Multifaceted Test Density** by creating test cases (`TC-*`) covering Normal, Edge, and Error paths.
2. **Testing Pyramid Compliance**: Maintain the ratio of **UT > IT > E2E > Live**.
   - **UT (Unit)**: Cover all logic branches (Fulfillment-focused).
   - **IT (Integration)**: Verify component boundaries and state transitions (Interface-focused).
   - **E2E (Scenario)**: Verify major user scenarios (Experience-focused).
3. **Failure Mode Analysis (FMEA)**: Before implementation, list "how it might break" and design tests to cover those modes.
4. **Harness Verification**: All tests must be executable via `scripts/check_harness.sh`.
5. **SDD & TDD**: Test code is the most explicit "prompt" for the AI.

## 📊 Test Levels and Density Goals

Achieve the following density goals based on the task's Tier:

| Level | Target | Density Metric | Goal (Tier 1) | Goal (Tier 2) |
| :--- | :--- | :--- | :--- | :--- |
| **UT (Unit)** | Internal Logic | **Branch Coverage** | 100% | 80%+ |
| **IT (Integration)** | Boundary/State | **Interface/State Coverage**| All Patterns | Major Patterns |
| **E2E (Scenario)** | User Experience | **AC Fulfillment / Scenario** | All ACs | Major ACs |
| **Live (Verify)** | Production | **SLO / Synthetic Mon** | Required | Optional |

## 🛡️ Failure Mode Analysis (FMEA)

[TODO: List potential failure modes and their corresponding test strategies.]

## 🗺️ Traceability Matrix

| TC ID | Target AC | Level (UT/IT/E2E/Live) | Perspective (Normal/Edge/Error) | Implementation Status |
| :--- | :--- | :--- | :--- | :--- |
| **TC-001** | AC-001 | UT | Normal | [ ] Red |
| **TC-002** | AC-001 | UT | Edge | [ ] Red |
| **TC-003** | AC-001 | IT | Error | [ ] Red |

## 🛠️ Test Case Details (TC-*)

### TC-001: [Test Name]
- **Preconditions**: [e.g., Data exists in DB]
- **Steps**:
  1. [Action 1]
  2. [Action 2]
- **Expected Result**: [Expected outcome]
