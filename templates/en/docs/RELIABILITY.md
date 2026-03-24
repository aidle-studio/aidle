<!--
[AI Agent Instructions for Reliability & Security]
⚠️ WARNING: This document is the "absolute law" for ensuring the robustness and safety of the system.
When implementing code (Act), AI agents must not break the following rules:
- Do not write code that swallows exceptions (Silent Failure).
- Always set a Timeout for calls to external dependencies (network, DB, file I/O).
- When implementing retries, ensure "idempotency" so that side effects are not duplicated.
- When propagating errors upward, always wrap them with "runtime context (parameters, Trace ID, etc.)".
- Do not execute external data or user input without verification (eval, OS commands, etc.) (Zero Trust).
-->

# RELIABILITY.md (Law of Reliability and Security)

This document provides design guidelines for ensuring the robustness and safety of the project, and is the **"Law of Defensive Coding"** that agents must always comply with during implementation.

## 🛡️ 1. Core Principles

The system prioritizes **"safe recovery/shutdown even if it breaks, and immediate identification of the cause"** over "never breaking."

- **❌ Must Not**: Swallowing exceptions (e.g., just logging and continuing, silent `return null`) is strictly prohibited.
- **✅ Must/Fail-fast**: If an unexpected state or invalid input is detected, immediately interrupt processing and propagate it as an error or panic safely (do not create a breeding ground for hidden bugs).
- **🔐 Zero Trust**: Assume all external data (user input, external API responses, environment variables, etc.) is "malicious" and always verify/sanitize it.

## 🚫 2. Prohibited Dangerous Patterns for AI Agents

AI agents must treat the following patterns as **"Avoid in principle."** If you absolutely need to use them, you must explain the risks and alternatives to a human and obtain **explicit agreement (Agreement Gate)** before implementation.

- **OS Command/Shell Injection**: Dynamic code execution using `eval`, `exec`, etc., or execution of shell commands with directly concatenated user input.
- **Path Traversal**: Construction of file paths using user input directly, or unrestricted access to system areas.

## 🔄 3. Resilience Strategies

When calling external dependencies (APIs, databases, file systems, etc.), be sure to incorporate the following protection mechanisms.

### 2.1 Timeout
- **Rule**: For all external communication and processing that may cause long blocks, always set an **explicit timeout duration** (No infinite waiting/resource exhaustion).

### 2.2 Retry & Idempotency
- **Rule**: For processing where "temporary failures" such as momentary network interruptions are expected, implement **automatic retries with Exponential Backoff and Jitter**.
- **Ensuring Idempotency**: Design so that even if the same operation is executed multiple times during retry, no side effects (data duplication, etc.) occur (e.g., using DB Upsert, checking processed status with a unique request ID).

### 2.3 Graceful Degradation / Fallback
- **Rule**: When part of the system (external services, etc.) fails, design to disconnect value-added functions to maintain core functionality, rather than bringing down the whole system in an "All or Nothing" manner.
- **Implementation Example**: If data retrieval fails, continue the service by displaying old cached data or default values instead of showing an error screen.

## 🔍 4. Observability and Traceability

When an error occurs, ensure that a human or **the AI agent itself** can mechanically and immediately identify "why it happened" later.

- **Structured Logging**: It is strongly recommended to output logs in a **structured format such as JSON**, rather than just strings. This allows AI agents to quickly parse and analyze (troubleshoot) errors using tools like `jq`.
- **Context Wrapping**: Do not return low-layer errors (e.g., `Connection Refused`) upward as they are. Propagate them by adding the context of the operation, such as "what you were trying to do."
- **Distributed Tracing (Trace ID)**: To follow a series of processing flows horizontally, not just single log outputs, generate a Trace ID at the start of processing and propagate it to all logs and external communication headers.
- **Protection of Sensitive Information**: **Never include** passwords, API keys, or personally identifiable information (PII) in error messages or log outputs.

## 💾 5. Prevention of Data Inconsistency

- **Transactions and Rollbacks**: If an error occurs during processing involving multiple database writes or file operations, thoroughly manage transactions (Rollback) so that the state always returns to "before processing." Do not leave halfway "garbage data" in the system.
