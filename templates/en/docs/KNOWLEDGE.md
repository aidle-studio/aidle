# KNOWLEDGE.md

This document is a structured knowledge base for project-specific implicit knowledge, lessons learned from bugs, and reasons for specific architectural constraints.
AI agents must update this file when a bug is fixed or a significant technical decision is made (**[x] part of the DoD**).

## 🎯 Operational Rules

- **Unified Format**: Record new knowledge using the "Troubleshooting" or "Q&A" format below.
- **Searchability**: Include relevant keywords and error messages to make it easily grep-able.

---

## 📚 Knowledge Base

### 🐛 Troubleshooting (Bugs & Lessons)

<!--
[Format]
- **Symptom/Error**: [e.g., `ModuleNotFoundError: No module named 'XXX'`]
- **Root Cause**: [e.g., `requirements.txt` was not updated inside the Docker container]
- **Remedy/Lesson**: [e.g., Always run `docker-compose build` after adding dependencies before running tests]
- **Date**: 20XX-MM-DD
-->

- **Symptom/Error**: [Describe the symptom here]
- **Root Cause**: [Describe the root cause here]
- **Remedy/Lesson**: [Describe the remedy or lesson here]
- **Date**: 20XX-MM-DD

---

### 💡 Project Quirks (Constraints & Decisions)

<!--
[Format]
- **Q**: [e.g., Why are we using `axios` instead of `fetch` here?]
- **A**: [e.g., Legacy backend API requires specific custom headers managed centrally via axios interceptors. Do not switch to fetch.]
-->

- **Q**: [Question or context]
- **A**: [Reason or constraint]
