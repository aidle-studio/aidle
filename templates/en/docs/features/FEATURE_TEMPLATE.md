<!--
[AI Agent Instructions for Feature Specification]
⚠️ WARNING: This document is a template for a "functional specification" that defines "requirements (What)" and "acceptance criteria (AC)" for specific features.
To prevent context bloat and ensure the success of Test-Driven Development (TDD), strictly follow the rules below:
1. **Do NOT write How (Internal Design)**: Leave "which DB table to use" and "which library to use" to `docs/design-docs/`, and define only "how it looks from the perspective of the user or external system (behavior)" here.
2. **Strictness of AC (Acceptance Criteria)**: Each AC must have the level of specificity (including boundary values and behavior in case of errors) that can be directly translated into test cases (UT/IT/E2E) for `TEST_PLAN.md`.
-->

# Feature: [TODO: Feature Name (e.g., User Authentication)]

## 📖 1. Overview
- **Purpose**: [TODO: Briefly describe why this feature is necessary and how it links to the core values of the PRD]
- **Target Audience**: [TODO: Who will use this feature]

## ✨ 2. Use Cases
Lists scenarios that users (or external systems) can achieve using this feature.

- **UC-1**: [Example: Registered users can log in to the system with their email address and password]
- **UC-2**: [Example: Users who have forgotten their password can receive a reset link by email]

## ✅ 3. Acceptance Criteria (AC)
Defines specific behavioral conditions (AC) that serve as criteria for implementation and testing.
*Note: Agents must create tests (TC) that satisfy all these AC during implementation.*

### AC-1: [TODO: Example: Successful Login]
- **Given**: [TODO: User has already created an account]
- **When**: [TODO: Enter a correct email address and password and submit]
- **Then**: [TODO: An authentication token is issued, and the user is redirected to the dashboard]

### AC-2: [TODO: Example: Incorrect Password (Anomaly Case)]
- **Given**: [TODO: User has already created an account]
- **When**: [TODO: Enter an incorrect password and submit]
- **Then**: [TODO: HTTP 401 Unauthorized is returned, and "Incorrect password" is displayed]
- **Then (Result 2)**: [TODO: For security reasons, lock the account after 5 consecutive failures]

## 🚫 4. Out of Scope / Constraints
- **Out of Scope**: [TODO: What will intentionally not be implemented in this feature release (e.g., SNS login feature is not included this time)]
- **Constraints**: [TODO: Constraints on performance or security (e.g., passwords must always be hashed before saving)]
