# HARNESS.md

This document provides **guidelines for AI agents to autonomously set up the project environment and put quality on a self-healing track**.
Always check the content at the start of a session to guarantee that your verification tools and hooks are functioning correctly.

## 🤖 Agent's Mission: Harness Setup

Immediately after joining a project or when detecting changes in the environment, autonomously set up the following "Harness (Training Wheels)".

### 1. Build Fast Feedback
- **Selection of Fast Linters**: Depending on the project's language (Rust, TS, Python...), install and configure the fastest tools (Oxlint, Ruff, Biome, etc.).
- **Configuration of Auto-fix Hooks**: Set up editor settings or Git Hooks (`pre-commit`, etc.) so that formatting and lint fixes run automatically immediately after the agent saves.

### 2. Protection of Rules and Configuration Files (Prevention of Test Cheating)
To prevent the AI from "loosening settings" to forcibly avoid errors (cheating), set up the following guardrails.
- **Monitoring Targets**: Configuration files such as `.eslintrc`, `rustfmt.toml`, `nextest.toml`, and existing test code.
- **Guardrail Example**: Build a mechanism that always issues a Warning if these files are changed and requires **"explicit human (user) review and agreement"** before committing.

### 3. Security Scan (Secret & SAST)
To prevent accidentally committing sensitive information (API keys, passwords), it is strongly recommended to **incorporate a lightweight secret scanner that can be run locally (`gitleaks`, `trufflehog`, or custom Linter rules, etc.) into `check_harness.sh` or a Pre-commit hook**.

### 4. Implementation of Definition of Done Checker (`check_harness.sh`)
Create `scripts/check_harness.sh` at the root of the project and implement logic to collectively execute the following "Quality Gates".
- **Lint & Format**: Zero warnings, zero diffs.
- **Test**: All tests are Green.
- **Coverage**: Strictly check whether the coverage targets defined in `TEST_PLAN.md` (e.g., Branch 100%) are met using CI tools (`llvm-cov`, `jest --coverage`, etc.).
- **Output**: Output `SUCCESS` (or `Done`) only when all succeed, and terminate immediately with `exit 1` if even one fails.

## 🛠️ Expected Values for Verification Commands

Agents have an obligation to maintain a state where the following commands (or equivalent integrated commands) always return `SUCCESS` during implementation.

1. **Lint**: `cargo clippy`, `eslint`, `ruff check`, etc. (Zero warnings recommended)
2. **Format**: `cargo fmt`, `prettier`, `black`, etc. (Zero modification diffs)
3. **Test & Coverage**: `cargo llvm-cov`, `jest --coverage`, etc. (Clear thresholds)
4. **Harness**: `scripts/check_harness.sh` (Collective check)

## 🔄 Self-Healing Flow

If verification fails, the AI should autonomously attempt to resolve it using the following steps.

1. Identify the "violated part" and "cause" from the error message.
2. Refer to hints provided by the harness or related documents (RULES.md, RELIABILITY.md) to establish a fix policy.
3. Apply the fix and run the verification script again to confirm `SUCCESS`.
