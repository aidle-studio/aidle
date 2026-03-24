# AGENTS.md

This project is an **Agent-First** development environment.
AI agents (Gemini, Claude, Cursor, etc.) MUST use this file as the starting point to understand the project's context and rules.

## 🗺️ Project Map (Core Context)

Detailed rules and contexts are located under `docs/`.
When joining the project, agents MUST read the following four core files in order as prerequisite knowledge:

1. **[ARCHITECTURE.md](ARCHITECTURE.md)**: The overall system structure, dependencies, and "Immutable Map."
2. **[docs/HARNESS.md](docs/HARNESS.md)**: [IMPORTANT] MUST be checked at the start of every session to set up your verification rails.
3. **[docs/AGENT_CONTEXT.md](docs/AGENT_CONTEXT.md)**: Dynamic context of the current phase, immediate goals, and next actions.
4. **[docs/RULES.md](docs/RULES.md)**: Definition of the development process and Definition of Done (DoD) (The Constitution).

## 🚀 Task Workflow

This project proceeds in a cycle of **"Plan ➔ Harness ➔ Act ➔ Audit ➔ Verify"**.
Details are described in `docs/RULES.md`.

- **Starting Point for Instructions**: When unsure of "what to do," ALWAYS return to `docs/AGENT_CONTEXT.md` to check the current status and active plans (`docs/exec-plans/active/*.md`).
- **Verification Priority**: MUST prioritize mechanical verification (Lint, Test) over natural language instructions.
- **Continuous Process Improvement**: If you identify flaws or areas for improvement in the current rules or documents, actively propose improvements.

## 🤖 Initial Setup

When newly joining the project or when the environment changes, immediately set up or verify your own verification environment (hooks, linters, verification scripts) according to **`docs/HARNESS.md`**.
