<!--
[AI Agent Instructions for Design Document]
⚠️ WARNING: This document is a template for defining "detailed design (How)" regarding specific features or system changes.
To prevent context bloat and clarify responsibilities, strictly follow the rules below:
1. **Do NOT write What (Requirements/Behavior)**: Leave user-perspective requirements and acceptance criteria (AC) to the corresponding `docs/features/*.md`, and define only "how to realize it (API, DB schema, algorithm)" here.
2. **Explicitly link to the corresponding Feature**: To make this design traceable to "which requirements it satisfies," always include a link to `features/*.md` at the beginning.
3. **Do NOT deviate from ARCHITECTURE.md**: Designs here must follow the tech stack and principles defined in the project's Architecture Document (`ARCHITECTURE.md`).
-->

# Design: [TODO: Design Target (e.g., User Authentication Infrastructure)]

## 🔗 1. Target Feature
This design is for realizing the following functional requirements.
- ➡️ **Feature**: [TODO: Link to corresponding functional specification (e.g., `docs/features/auth.md`)]

## 📐 2. System Architecture
- [TODO: Briefly describe component diagrams or data flows (sequences) that this feature affects]
- [TODO: Interactions between client and server, etc.]

## 💾 3. Data Model
- [TODO: Define the structure of newly added or changed DB tables, file formats, and states]
- [TODO: Describe constraints (unique keys, foreign keys, etc.) as well]

## 🔌 4. Interfaces (API / Interfaces)
- [TODO: Define newly added or changed endpoints, function signatures, event payloads, etc.]
- [TODO: JSON schema for Request / Response, etc.]

## 🛡️ 5. Security & Reliability
- [TODO: Based on `RELIABILITY.md`, describe the fallback and timeout strategy for errors in this design]
- [TODO: Explicitly state authorization verification methods and transaction scope]

## 🔄 6. Migration / Compatibility
- [TODO: Describe the procedures if migration from existing systems or data is required. If not required, "N/A"]
