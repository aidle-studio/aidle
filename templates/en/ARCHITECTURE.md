<!--
[AI Agent Instructions for Architecture]
⚠️ WARNING: This file is a Core Context and is loaded in every session.
To prevent context bloat, strictly adhere to the following update rules:
1. **Keep it as an Overview**: Only concisely describe the "system's big picture" and the "primary tech stack" in this file.
2. **Move Details Out**: Detailed internal designs for specific modules or features should be written in `docs/design-docs/*.md` instead of this file, with only links provided here.
3. **Separate History**: The "rationale (background)" for technology choices should be recorded in `docs/adr/*.md` instead of this file.
-->

# ARCHITECTURE.md (Base Specification)

This document is the "Immutable Map" showing the overall structure and tech stack of the system.
For an overview of the requirements (What), refer to the Product Requirements Document ➡️ **[PRD](PRD.md)**.

## 1. 🏗️ System Overview
- [TODO: Describe the overall system architecture (e.g., frontend and backend configuration, etc.) in a few lines]
- 💡 For the overall design philosophy (Core Beliefs), refer to ➡️ `docs/design-docs/core-beliefs.md`.

## 2. 📚 Tech Stack
* For the rationale behind each technology choice, refer to the records under `docs/adr/`.
* If there are **"project-specific best practices (cheat sheets for AI)"** for each technology or library, ALWAYS refer to `docs/references/` before implementation.

- **Frontend**: [TODO: e.g., React (Next.js)] ➡️ `docs/references/[TODO].md`
- **Backend**: [TODO: e.g., Rust (Axum)]
- **Database**: [TODO: e.g., PostgreSQL]

## 3. 📂 Main Components and Directory Structure

<!-- 
[AI Agent Instructions for Updating this Table]
- The granularity of additions should be "major directory (module) units"; do not list individual files.
- The "Role" column should be kept within 2 sentences (approx. 100 characters).
- When adding a new component, always specify the path under `docs/design-docs/` for the "Link to Detailed Design" (it's okay if the file hasn't been created yet).

✅ Ideal Example:
| Component (Path) | Role | Link to Detailed Design |
| :--- | :--- | :--- |
| `src/api/` | REST API endpoint layer for external use. Responsible for routing and I/O validation. | ➡️ `docs/design-docs/api-design.md` |
| `src/core/` | Business logic and domain models. A pure logic layer with no external dependencies. | ➡️ `docs/design-docs/domain-model.md` |
| `src/infra/` | Implementation layer for DB connections and external API clients. | ➡️ `docs/design-docs/infra-layer.md` |
-->

| Component (Path) | Role | Link to Detailed Design |
| :--- | :--- | :--- |
| `src/` | [TODO: Application's main source code] | ➡️ `docs/design-docs/main-structure.md` |
| `docs/` | Documentation for agents and humans. Manages rules and specifications. | - |

## 4. 🔗 Links to Common Technical Rules
Refer to the following for project-wide laws (rules) to be followed during implementation.

- ➡️ **Error Handling and Security Constraints**: `docs/RELIABILITY.md`
