# AGENTS.md

This project is an **Agent-First** development environment.
AI agents (e.g., Claude Code, Cursor, Windsurf) should use this file as their starting point to understand the project.

## 🗺️ Project Map

Detailed rules and context are organized in the `docs/` directory following the principle of "Progressive Disclosure."
Agents should read the following files in order to understand the current situation:

1. **[docs/TODO.md](docs/TODO.md)**: Overall roadmap, milestones, and task list.
2. **[docs/AGENT_CONTEXT.md](docs/AGENT_CONTEXT.md)**: Current phase, immediate goals, and next actions.
3. **[docs/RULES.md](docs/RULES.md)**: Definition of the development process and DoD.
4. **[docs/SPEC.md](docs/SPEC.md)**: Requirements and Acceptance Criteria (Source of Truth).
5. **[docs/TEST_PLAN.md](docs/TEST_PLAN.md)**: Testing strategy and AC-TC traceability.
6. **[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)**: High-level design and technical stack.

## 🚀 Development Workflow

This project proceeds through the **"Plan ➔ Harness ➔ Act ➔ Review ➔ Verify"** cycle.

- **Self-Review & Self-Criticism**: Before submitting any results, agents must conduct a self-review using the checklist in `docs/RULES.md`. Emphasis is placed on identifying and fixing vulnerabilities in one's own code and logic (Self-Criticism).
- **Continuous Process Kaizen**: If you find flaws or improvements in project rules or documentation, propose an update according to the "Process Evolution" section in `docs/RULES.md`. Improving the process itself is a key duty of the agent.
- **Verification First**: Trust mechanical verification (Lint, Test) over natural language instructions.
- For details, refer to the specialized documents in `docs/` (SPEC.md, TEST_PLAN.md, etc.).

## 🤖 Initial Setup

Upon joining the project or when the environment changes, immediately follow **`docs/HARNESS.md`** to set up or verify your own verification environment (hooks, linters, scripts).
