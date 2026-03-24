# KNOWLEDGE.md

This document is a structured memory store for accumulating project-specific "tacit knowledge," "lessons from bugs," and "reasons for special specifications."
AI agents **MUST add to this file (as part of the [x] completion criteria) when fixing a bug or making an important technical decision (that doesn't warrant an ADR)**.

## 🎯 Operational Rules

- **Standardization of Format**: New insights must be added in the following "Troubleshooting" or "Q&A" format.
- **Ensuring Searchability**: Describe related keywords and error messages as they are to make them easy to grep later.

---

## 📚 Accumulated Insights (Knowledge Base)

### 🐛 Bugs and Lessons (Troubleshooting)

<!--
[Format]
- **Phenomenon/Error**: [Example: `ModuleNotFoundError: No module named 'XXX'`]
- **Cause**: [Example: `requirements.txt` was not updated in the Docker container]
- **Measure/Lesson**: [Example: When adding dependencies, always run `docker-compose build` before running tests]
- **Date**: YYYY-MM-DD
-->

- **Phenomenon/Error**: [Describe the phenomenon here]
- **Cause**: [Describe the cause here]
- **Measure/Lesson**: [Describe the measure here]
- **Date**: YYYY-MM-DD

---

### 💡 Unique Specifications and Constraints (Project Quirks)

<!--
[Format]
- **Q**: [Example: Why is `axios` being used here instead of `fetch`?]
- **A**: [Example: The legacy backend API requires special custom headers, which are managed centrally with `axios` interceptors. Do not change to `fetch` on your own!]
-->

- **Q**: [Describe the question/context here]
- **A**: [Describe the reason/constraint here]
