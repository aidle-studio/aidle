<!--
[AI Agent Instructions for Audit]
⚠️ WARNING: This document is the "final line of defense for quality" that the implemented code must satisfy.
Agents MUST strictly review (self-criticize) their own code using the criteria in this document before completing a task (checking DoD).
For significant changes, it is strongly recommended to request a peer review from an AI in another session (e.g., code-reviewer) using these criteria.
-->

# AUDIT.md (Multifaceted Self-Audit & Code Review Criteria)

This document is the "Self-Audit Criteria" for evaluating the quality of implemented code.
Agents (or reviewers) must strictly check whether there are any compromises or defects in the implemented code according to the following five quality characteristics (based on ISO/IEC 25010).

## 1. Maintainability
Verify the lifespan of the code to ensure that future you or your teammates won't struggle.
- **Analyzability**: Do variable and function names accurately represent their roles? (e.g., naming it `userProfile` instead of `data`)
- **Changeability**: Is a single function or module too large? (Is one function = one responsibility?)
- **Reusability**: Is there similar logic elsewhere? (Is the DRY principle violated?)
- **Testability**: Is the code structured so that unit tests are easy to write? (Is dependency injection possible?)

## 2. Functional Suitability
Confirm whether what you "wanted to do" is actually achieved.
- **Functional Completeness**: Does it satisfy all Acceptance Criteria (AC) described in the requirements definition (`PRD.md`) or functional specifications (`docs/features/*.md`)?
- **Functional Correctness**: Are there any mistakes in calculation logic or conditional branching? (Check boundary values such as off-by-one errors)
- **Functional Appropriateness**: Is the implementation method the simplest and most valid means to achieve the goal? (Are you over-engineering with excessive abstraction?)

## 3. Reliability
Confirm that the system does not stop even if an error occurs, or that it stops safely.
- **Maturity**: Have exceptional cases (network errors, missing files, invalid input, etc.) been considered?
- **Availability**: Is there any possibility of falling into an infinite loop or deadlock in a specific process?
- **Recoverability**: When an error occurs, is an appropriate structured log (Trace ID, etc.) output, and are retry or cleanup processes (disconnection, transaction rollback) performed?

## 4. Performance Efficiency
Confirm that resources are not being wasted.
- **Time Efficiency**: Are unnecessary API calls or DB queries (N+1 problem) being issued within loops?
- **Resource Efficiency**: Are large objects that consume a lot of memory being held even after they are no longer needed? (Proper scope and lifecycle management)

## 5. Security
As a gatekeeper for developers, check for any introduced vulnerabilities.
- **Confidentiality**: Are passwords, personal information, or API keys being output in plain text in logs or error messages?
- **Integrity**: Is external input (user input, URL parameters, etc.) being used directly in SQL or HTML? (Thorough measures against SQL injection and XSS)

## 6. Test Validity
Verify the quality and coverage of not only the implemented code but also the "test code itself" that guarantees it.
- **Proof of Coverage**: **Have test cases actually been implemented and executed for all items** in the "Boundary/Anomaly Matrix" and "FMEA Matrix" defined in the plan (`PLAN_TEMPLATE.md`)?
- **Elimination of Bloat**: Are there any meaningless tests that don't have assertions (verification of expected values) just to increase coverage?
- **Independence**: Is the test stable and executable on its own (not flaky), without depending on the execution order or state of other tests?

## 7. Documentation Sync
Verify whether code changes are correctly reflected in "The Source of Truth" of the project.
- **Specification Updates**: Are the requirements/AC in `PRD.md` or `docs/features/*.md` updated to match the implementation?
- **Design Updates**: When adding new components or dependencies, are `ARCHITECTURE.md` or `docs/design-docs/*.md` updated?
- **Intent Recording**: Have you left records in `docs/adr/` or `docs/KNOWLEDGE.md` about why you chose that design and what traps you fell into?
