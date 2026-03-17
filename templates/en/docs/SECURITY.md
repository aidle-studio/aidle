# SECURITY.md

This document defines the security standards, handling of sensitive information, and secure coding guidelines for this project.

## 🛡️ Secure by Design Principles

Regardless of the environment (Web, CLI, Infra), adhere to these universal security principles:

1. **Principle of Least Privilege**: Grant only the minimum permissions necessary for a program or component to perform its task.
2. **Zero Trust**: Treat all external data (user input, API responses, env vars, files) as untrusted. Always validate and sanitize.
3. **Fail Securely**: When an error or exception occurs, the system must default to a secure state (no access). Never expose secrets or internal structures in logs or error messages.

## 🤖 AI Agent Watchlist (Requires Agreement)

AI agents must treat the following patterns as **"Strongly Discouraged (Avoid)"**.
If absolutely necessary, explain the risks and alternatives to a human and obtain an **Agreement Gate** before implementation.

- **OS Command/Shell Injection**: Dynamic code execution using `eval`, `exec`, or direct string concatenation of user input into shell commands. (*Zero Trust Violation*)
- **Plain-text Secret Leakage**: Outputting passwords, API keys, or other secrets in logs, error messages, or code comments. (*Fail Securely Violation*)
- **Path Traversal**: Constructing file paths directly from user input or granting unrestricted access to system areas.

## 🔒 Security Scanning

Build the following security checks into the Harness:
- **Static Analysis (SAST)**: Detect vulnerabilities in code.
- **Dependency Scanning**: Detect packages with known vulnerabilities.
- **Secret Scanning**: Detect accidentally committed credentials.

## 🚨 Vulnerability Reporting

If you find a security concern, record it in `SECURITY.md` and report it to a human immediately. Confirm if a private contact route exists before creating a public issue.
