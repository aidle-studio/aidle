# TEST_PLAN.md

This document provides guidelines for the test plan to verify the "requirements, constraints, and acceptance criteria" defined in the Product Requirements Document (`PRD.md`), Architecture Document (`ARCHITECTURE.md`), and functional specifications (`docs/features/*.md`). It is the **"Law of Testing"** that agents must always comply with when implementing tests.

## 🧪 Principles and Prohibitions of Test Design

Maintain the pyramid ratio of **UT > IT > E2E** for the number of tests.
When writing and executing tests, agents must strictly follow the following principles and **prohibitions**.

1. **Elimination of Bloat and Slacking**:
   - **❌ Must Not (Assert-free Test)**: Do not write meaningless tests that simply call a function without verifying the result (e.g., using `assert`) just to increase branch coverage.
   - Meaningless duplicate tests or tests just to "increase the count" are strictly prohibited.
2. **No Test Cheating**:
   - **❌ Must Not**: If a test fails during the implementation of product code, **"never forcibly pass it by rewriting the expected values of the test code."** Modifying tests is only allowed when the functional specifications (AC) themselves are changed.
3. **Multifaceted Testing (Thoroughness of FMEA and Boundary Values)**:
   - Include not only normal cases but also boundary values (Empty, Null, Max, Emoji/Special Characters, extremely long strings, etc.).
   - In integration tests (IT), **perspectives on failure modes such as "Timeout," "Connection Error," and "Invalid Data"** are mandatory.
4. **Deterministic Testing**:
   - Utilize mocks and stubs to ensure that tests always return the same results, independent of external factors such as network, current time, or random numbers.

## 🧪 Mandatory Test Perspectives Based on Software Quality Characteristics

Design tests covering the following quality characteristics based on ISO/IEC 25010, not just "does it work."

### 1. Functional Suitability
The most basic test to verify "does it work correctly according to specifications."
- **Functional Completeness**: Are all use cases described in the specifications (`PRD.md`, `features/*.md`) executable? Is necessary data correctly saved, updated, and displayed?
- **Functional Correctness**: Does the calculation result match the expected value perfectly? Are decisions made as expected at boundary values (Min, Max, before/after, Empty, Null)?
- **Functional Appropriateness**: Are there any unnecessary operations or contradictions in the steps for the user to achieve the goal?

### 2. Performance Efficiency
Tests to verify "how resources are being used."
- **Time Efficiency**: Is the response speed within the target value (e.g., within 1 second)? Does the time not increase exponentially when processing large amounts of data (presence or absence of N+1 problems, etc.)?
- **Resource Efficiency**: Does memory usage or CPU load return to the original level after processing is completed (prevention of memory leaks)?
- **Capacity Satisfaction**: When load is applied, can the system withstand it with queuing, etc., without resulting in an error?

### 3. Reliability
Tests to verify "how it withstands harsh conditions" (Mandatory in the IT layer).
- **Maturity**: Do unexplained crashes or hangs not occur within the range of normal operations?
- **Fault Tolerance**: When an external API or DB is down, can the system as a whole withstand it with "fallback" or "error display" without crashing?
- **Recoverability**: After abnormal termination, is data not corrupted when restarted, and can it be restored from the state just before (transaction rollback)?

### 4. Usability
Tests to verify "whether the user is confused or self-destructs."
- **Operability**: Is the content of the error message specific, and can the user understand "what to do next"?
- **User Error Protection**: Can incorrect input (different character types, extremely long strings, etc.) be rejected immediately to keep the system safe?

### 5. Security
Tests to verify "resistance to malicious operations."
- **Confidentiality**: Can unauthorized users not access other people's data (verification of authorization)?
- **Integrity**: Is external input (SQL injection, XSS, etc.) processed after being sanitized?
- **Non-repudiation**: Are logs (Trace ID, etc.) of "who did what and when" reliably kept?

---

## 📊 Definition of Importance (Tier) and Density Goals

To ensure the reliability of the project, the following **minimum test density** must be achieved according to the importance (Tier) of the task. When creating a plan (`PLAN_TEMPLATE.md`), identify test cases based on these criteria.

| Tier (Importance) | Definition | Density Goal: Number of cases per Requirement (AC) | Coverage Goal (Branch) |
| :--- | :--- | :--- | :--- |
| **Tier 1 (Core)** | Core system logic, data integrity, security, etc. | **Avg 5.0 cases or more**<br>(e.g., Normal 1, Boundary 2, Anomaly 2) | **100%** |
| **Tier 2 (Feature)** | Major user features, business logic, etc. | **Avg 3.0 cases or more**<br>(e.g., Normal 1, Boundary 1, Anomaly 1) | **80% or more** |
| **Tier 3 (Utility)** | Auxiliary tools, internal utilities, UI tweaks, etc. | **Avg 1.0 case or more**<br>(Normal cases only is acceptable) | Main path conduction |

## 🛡️ Roles and Verification Perspectives of Each Test Level

Agents should place tests according to the following division of roles.

### UT (Unit Test)
- **Role**: Quickly verify the logic of the smallest units (functions/modules).
- **Mandatory Perspectives**: Equivalence partitioning, boundary value analysis (Max, Min, Empty, Zero, invalid types, etc.).
- **Goal**: Crushing all branches (if/else, match, etc.) here.

### IT (Integration Test)
- **Role**: Verify boundaries between components (DB collaboration, external API calls, etc.).
- **Mandatory Perspectives**: Recovery/fallback behavior when the other party is broken (Timeout, 5xx Error, disconnection, schema mismatch).

### E2E (End-to-End Test)
- **Role**: Verify whether major user scenarios pass from beginning to end (conduction).
- **Mandatory Perspectives**: Happy path, and typical user runaways (repeated clicks, interruptions, etc.).

## 🗺️ Traceability Matrix (Reference)

When planning according to `docs/PLAN_TEMPLATE.md`, identify perspectives in a matrix format like the one below to prove that there are no omissions.

| TC ID | Corresponding AC | Level | Perspective (Normal/Edge/Error) |
| :--- | :--- | :--- | :--- |
| **TC-001** | AC-001 | UT | Normal: Valid ID |
| **TC-002** | AC-001 | UT | Edge: Empty ID |
| **TC-003** | AC-001 | IT | Error: Retry on DB timeout |
